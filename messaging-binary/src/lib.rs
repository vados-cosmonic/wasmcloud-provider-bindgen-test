wit_bindgen_wasmcloud_provider_binary::generate!(
    NatsMessagingProvider,
    "wasmcloud:messaging",
    "messaging"
);

use std::{collections::HashMap, sync::Arc, time::Duration};

use async_trait::async_trait;
use futures::StreamExt;
use tokio::sync::{OwnedSemaphorePermit, RwLock, Semaphore};
use tokio::task::JoinHandle;
use tracing::{debug, error, instrument, warn};
use tracing_futures::Instrument;
use wascap::prelude::KeyPair;

use wasmcloud_provider_sdk::core::HostData;
use wasmcloud_provider_sdk::error::ProviderResult;
use wasmcloud_provider_sdk::{core::LinkDefinition, Context};

mod types;
pub(crate) use types::ConnectionConfig;
use types::NatsClientBundle;

/// Nats implementation for wasmcloud:messaging
#[derive(Default, Clone)]
pub struct NatsMessagingProvider {
    // store nats connection client per actor
    actors: Arc<RwLock<HashMap<String, NatsClientBundle>>>,
    pub(crate) default_config: ConnectionConfig,
}

impl NatsMessagingProvider {
    /// Create a NatsMessaging Provider
    pub fn from_host_data(host_data: HostData) -> NatsMessagingProvider {
        if let Some(c) = host_data.config_json.as_ref() {
            // empty string becomes the default configuration
            if c.trim().is_empty() {
                NatsMessagingProvider::default()
            } else {
                let config: ConnectionConfig = serde_json::from_str(c)
                    .expect("JSON deserialization from connection config should have worked");
                NatsMessagingProvider {
                    default_config: config,
                    ..Default::default()
                }
            }
        } else {
            NatsMessagingProvider::default()
        }
    }

    /// Attempt to connect to nats url (with jwt credentials, if provided)
    async fn connect(
        &self,
        cfg: ConnectionConfig,
        ld: &LinkDefinition,
    ) -> anyhow::Result<NatsClientBundle> {
        let opts = match (cfg.auth_jwt, cfg.auth_seed) {
            (Some(jwt), Some(seed)) => {
                let key_pair = std::sync::Arc::new(KeyPair::from_seed(&seed)?);
                async_nats::ConnectOptions::with_jwt(jwt, move |nonce| {
                    let key_pair = key_pair.clone();
                    async move { key_pair.sign(&nonce).map_err(async_nats::AuthError::new) }
                })
            }
            (None, None) => async_nats::ConnectOptions::default(),
            _ => {
                anyhow::bail!("must provide both jwt and seed for jwt authentication")
            }
        };

        // Use the first visible cluster_uri
        let url = cfg.cluster_uris.get(0).unwrap();

        let client = opts
            .name("NATS Messaging Provider") // allow this to show up uniquely in a NATS connection list
            .connect(url)
            .await?;

        // Connections
        let mut sub_handles = Vec::new();
        for sub in cfg.subscriptions.iter().filter(|s| !s.is_empty()) {
            let (sub, queue) = match sub.split_once('|') {
                Some((sub, queue)) => (sub, Some(queue.to_string())),
                None => (sub.as_str(), None),
            };

            sub_handles.push((
                sub.to_string(),
                self.subscribe(&client, ld, sub.to_string(), queue).await?,
            ));
        }

        Ok(NatsClientBundle {
            client,
            sub_handles,
        })
    }

    /// Add a regular or queue subscription
    async fn subscribe(
        &self,
        client: &async_nats::Client,
        ld: &LinkDefinition,
        sub: String,
        queue: Option<String>,
    ) -> anyhow::Result<JoinHandle<()>> {
        let mut subscriber = match queue {
            Some(queue) => client.queue_subscribe(sub.clone(), queue).await,
            None => client.subscribe(sub.clone()).await,
        }
        .map_err(|e| {
            error!(subject = %sub, error = %e, "error subscribing subscribing");
            e
        })?;

        let link_def = ld.to_owned();

        // Spawn a thread that listens for messages coming from NATS
        // this thread is expected to run the full duration that the provider is available
        let join_handle = tokio::spawn(async move {
            // MAGIC NUMBER: Based on our benchmark testing, this seems to be a good upper limit
            // where we start to get diminishing returns. We can consider making this
            // configurable down the line.
            // NOTE (thomastaylor312): It may be better to have a semaphore pool on the
            // NatsMessagingProvider struct that has a global limit of permits so that we don't end
            // up with 20 subscriptions all getting slammed with up to 75 tasks, but we should wait
            // to do anything until we see what happens with real world usage and benchmarking
            let semaphore = Arc::new(Semaphore::new(75));

            // Listen for NATS message(s)
            while let Some(msg) = subscriber.next().await {
                // Set up tracing context for the NATS message
                let span = tracing::debug_span!("handle_message", actor_id = %link_def.actor_id);

                let permit = match semaphore.clone().acquire_owned().await {
                    Ok(p) => p,
                    Err(_) => {
                        warn!("Work pool has been closed, exiting queue subscribe");
                        break;
                    }
                };

                tokio::spawn(dispatch_msg(link_def.clone(), msg, permit).instrument(span));
            }
        });

        Ok(join_handle)
    }
}

#[instrument(level = "debug", skip_all, fields(actor_id = %link_def.actor_id, subject = %nats_msg.subject, reply_to = ?nats_msg.reply))]
async fn dispatch_msg(
    link_def: LinkDefinition,
    nats_msg: async_nats::Message,
    _permit: OwnedSemaphorePermit,
) {
    let msg = BrokerMessage {
        body: Some(nats_msg.payload.into()),
        reply_to: nats_msg.reply,
        subject: nats_msg.subject,
    };
    let actor = InvocationHandler::new(&link_def);
    if let Err(e) = actor.handle_message(msg).await {
        error!(
            error = %e,
            "Unable to send subscription"
        );
    }
}

/////////////////////////////////////////////
// Trait fulfillment / Invocation handling //
/////////////////////////////////////////////
//
// The methods in this section are forced
// to be implemented/written due to trait + impl
// generated by the bindgen
//
#[async_trait]
impl WasmcloudMessagingConsumer for NatsMessagingProvider {
    #[instrument(level = "debug", skip(self, ctx), fields(actor_id = ?ctx.actor, subject = %msg.subject))]
    async fn publish(&self, ctx: Context, msg: BrokerMessage) -> ProviderResult<Result<(), String>> {
        let actor_id = if let Some(id) = ctx.actor.as_ref() {
            id
        } else {
            return Ok(Err("no actor in request".to_string()));
        };

        // get read lock on actor-client hashmap to get the connection, then drop it
        let nats_client = {
            let _rd = self.actors.read().await;

            let nats_bundle = if let Some(bundle) = _rd.get(actor_id) {
                bundle
            } else {
                return Ok(Err(format!("actor not linked:{}", actor_id)));
            };

            nats_bundle.client.clone()
        };
        // TODO(thomastaylor312): Since we've removed the tracing from normal NATS messaging, we
        // don't have the injector that works for us here. We can consider adding this back in in a
        // different way (or contributing to the async-nats library to make this easier)
        let body = msg.body.unwrap_or_default();
        debug!(body_len = body.len(), "publishing message");
        let res = match msg.reply_to {
            Some(reply_to) => nats_client
                .publish_with_reply(msg.subject, reply_to, body.into())
                .await
                .map_err(|e| e.to_string()),
            None => nats_client
                .publish(msg.subject, body.into())
                .await
                .map_err(|e| e.to_string()),
        };

        let _ = nats_client.flush().await;
        Ok(res)
    }

    async fn request_multi(
        &self,
        _ctx: Context,
        _subject: String,
        _body: Option<Vec<u8>>,
        _timeout_ms: u32,
        _max_results: u32,
    ) -> ProviderResult<Result<Vec<BrokerMessage>, String>> {
        todo!("request_multi not implemented")
    }

    #[instrument(level = "debug", skip(self, ctx, body), fields(actor_id = ?ctx.actor))]
    async fn request(
        &self,
        ctx: wasmcloud_provider_sdk::Context,
        subject: String,
        body: Option<Vec<u8>>,
        timeout_ms: u32,
    ) -> ProviderResult<Result<BrokerMessage, String>> {
        let actor_id = if let Some(id) = ctx.actor.as_ref() {
            id
        } else {
            return Ok(Err("no actor in request".to_string()));
        };

        let nats_client = {
            let _rd = self.actors.read().await;

            let nats_bundle = if let Some(bundle) = _rd.get(actor_id) {
                bundle
            } else {
                return Ok(Err(format!("actor not linked:{}", actor_id)));
            };

            nats_bundle.client.clone()
        };

        // Perform the request with a timeout
        let request_with_timeout = tokio::time::timeout(
            Duration::from_millis(timeout_ms as u64),
            nats_client.request(subject, body.unwrap_or_default().into()),
        )
        .await;

        // Process results of request
        match request_with_timeout {
            Err(_timeout_err) => Ok(Err("nats request timed out".to_string())),
            Ok(Err(send_err)) => Ok(Err(format!("nats send error: {}", send_err))),
            Ok(Ok(resp)) => Ok(Ok(BrokerMessage {
                body: Some(resp.payload.into()),
                reply_to: resp.reply,
                subject: resp.subject,
            })),
        }
    }
}

//////////////////////////////////
//// Wasmcloud-internal methods //
//////////////////////////////////
//
// The WasmcloudCapabilityProvider trait is auto-generated, and the
// traits within it are used to ensure compatibility with a surrounding
// wasmCloud lattice.
#[async_trait]
impl WasmcloudCapabilityProvider for NatsMessagingProvider {
    /// Provider should perform any operations needed for a new link,
    /// including setting up per-actor resources, and checking authorization.
    /// If the link is allowed, return true, otherwise return false to deny the link.
    #[instrument(level = "debug", skip(self, ld), fields(actor_id = %ld.actor_id))]
    async fn put_link(&self, ld: &LinkDefinition) -> bool {
        // If the link definition values are empty, use the default connection configuration
        let config = if ld.values.is_empty() {
            self.default_config.clone()
        } else {
            // create a config from the supplied values and merge that with the existing default
            match ConnectionConfig::new_from(&ld.values) {
                Ok(cc) => self.default_config.merge(&cc),
                Err(e) => {
                    error!(error = %e, "Failed to build connection configuration");
                    return false;
                }
            }
        };

        match self.connect(config, ld).await {
            Ok(nats_bundle) => {
                let mut update_map = self.actors.write().await;
                update_map.insert(ld.actor_id.to_string(), nats_bundle);
                true
            }
            Err(e) => {
                error!(error = %e, "Failed to connect to NATS");
                false
            }
        }
    }

    /// Handle notification that a link is dropped: close the connection
    #[instrument(level = "info", skip(self))]
    async fn delete_link(&self, actor_id: &str) {
        let mut aw = self.actors.write().await;

        if let Some(bundle) = aw.remove(actor_id) {
            // Note: subscriptions will be closed via Drop on the NatsClientBundle
            debug!(
                "closing [{}] NATS subscriptions for actor [{}]...",
                &bundle.sub_handles.len(),
                actor_id,
            );
        }

        debug!("finished processing delete link for actor [{}]", actor_id);
    }

    /// Handle shutdown request by closing all connections
    async fn shutdown(&self) {
        let mut aw = self.actors.write().await;
        // empty the actor link data and stop all servers
        aw.clear();
        // dropping all connections should send unsubscribes and close the connections.
    }
}

#[cfg(test)]
mod test {
    use crate::{ConnectionConfig, NatsMessagingProvider};
    use wasmcloud_provider_sdk::{
        core::{HostData, LinkDefinition},
        ProviderHandler,
    };

    #[test]
    fn test_default_connection_serialize() {
        // test to verify that we can default a config with partial input
        let input = r#"
{
    "cluster_uris": ["nats://soyvuh"],
    "auth_jwt": "authy",
    "auth_seed": "seedy"
}
"#;

        let config: ConnectionConfig = serde_json::from_str(input).unwrap();
        assert_eq!(config.auth_jwt.unwrap(), "authy");
        assert_eq!(config.auth_seed.unwrap(), "seedy");
        assert_eq!(config.cluster_uris, ["nats://soyvuh"]);
        assert!(config.subscriptions.is_empty());
        assert!(config.ping_interval_sec.is_none());
    }

    #[test]
    fn test_generate_provider_works_with_empty_string() {
        let host_data = HostData {
            config_json: Some("".to_string()),
            ..Default::default()
        };
        let prov = NatsMessagingProvider::from_host_data(host_data);
        assert_eq!(prov.default_config, ConnectionConfig::default());
    }

    #[test]
    fn test_generate_provider_works_with_none() {
        let host_data = HostData {
            config_json: None,
            ..Default::default()
        };
        let prov = NatsMessagingProvider::from_host_data(host_data);
        assert_eq!(prov.default_config, ConnectionConfig::default());
    }

    #[test]
    fn test_connectionconfig_merge() {
        // second > original, individual vec fields are replace not extend
        let cc1 = ConnectionConfig {
            cluster_uris: vec!["old_server".to_string()],
            subscriptions: vec!["topic1".to_string()],
            ..Default::default()
        };
        let cc2 = ConnectionConfig {
            cluster_uris: vec!["server1".to_string(), "server2".to_string()],
            auth_jwt: Some("jawty".to_string()),
            ..Default::default()
        };

        let cc3 = cc1.merge(&cc2);
        assert_eq!(cc3.cluster_uris, cc2.cluster_uris);
        assert_eq!(cc3.subscriptions, cc1.subscriptions);
        assert_eq!(cc3.auth_jwt, Some("jawty".to_string()))
    }

    /// Ensure that unlink triggers subscription removal
    /// https://github.com/wasmCloud/capability-providers/issues/196
    ///
    /// NOTE: this is tested here for easy access to put_link/del_link without
    /// the fuss of loading/managing individual actors in the lattice
    #[tokio::test]
    async fn test_link_unsub() -> anyhow::Result<()> {
        // Build a nats messaging provider
        let prov = NatsMessagingProvider::default();

        // Actor should have no clients and no subs before hand
        let actor_map = prov.actors.write().await;
        assert_eq!(actor_map.len(), 0);
        drop(actor_map);

        // Add a provider
        let ld = LinkDefinition {
            actor_id: String::from("???"),
            link_name: String::from("test"),
            contract_id: String::from("test"),
            values: vec![
                (
                    String::from("SUBSCRIPTION"),
                    String::from("test.wasmcloud.unlink"),
                ),
                (String::from("URI"), String::from("127.0.0.1:4222")),
            ],
            ..Default::default()
        };
        prov.put_link(&ld).await;

        // After putting a link there should be one sub
        let actor_map = prov.actors.write().await;
        assert_eq!(actor_map.len(), 1);
        assert_eq!(actor_map.get("???").unwrap().sub_handles.len(), 1);
        drop(actor_map);

        // Remove link (this should kill the subscription)
        let _ = prov.delete_link(&ld.actor_id).await;

        // After removing a link there should be no subs
        let actor_map = prov.actors.write().await;
        assert_eq!(actor_map.len(), 0);
        drop(actor_map);

        prov.shutdown().await;
        Ok(())
    }

    /// Ensure that provided URIs are honored by NATS provider
    /// https://github.com/wasmCloud/capability-providers/issues/231
    ///
    /// NOTE: This test can't be rolled into the put_link test because
    /// NATS does not store the URL you fed it to connect -- it stores the host's view in
    /// [async_nats::ServerInfo]
    #[tokio::test]
    async fn test_link_value_uri_usage() -> anyhow::Result<()> {
        // Build a nats messaging provider
        let prov = NatsMessagingProvider::default();

        // Actor should have no clients and no subs before hand
        let actor_map = prov.actors.write().await;
        assert_eq!(actor_map.len(), 0);
        drop(actor_map);

        // Add a provider
        let ld = LinkDefinition {
            actor_id: String::from("???"),
            link_name: String::from("test"),
            contract_id: String::from("test"),
            values: vec![
                (
                    String::from("SUBSCRIPTION"),
                    String::from("test.wasmcloud.unlink"),
                ),
                (String::from("URI"), String::from("99.99.99.99:4222")),
            ],
            ..Default::default()
        };
        let result = prov.put_link(&ld).await;

        // Expect the result to fail, connecting to an IP that (should) not exist
        assert!(!result, "put_link failed");

        let _ = prov.shutdown().await;
        Ok(())
    }
}
