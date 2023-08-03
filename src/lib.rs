// NOTE: all this code is borrowed from the WIT-ified provider done by thomastaylor312
// https://github.com/thomastaylor312/nats-messaging-wit

// use async_trait::async_trait;
// use provider_sdk::core::LinkDefinition;
// use serde::{Deserialize, Serialize};
// use tracing::{debug, error, instrument, warn};
// use wit_bindgen_wasmcloud::provider_sdk::ProviderHandler;

#[cfg(feature = "otel")]
use tracing_futures::Instrument;

// We may not need the extra crate here
// wit_bindgen_wasmcloud::generate!(MessagingProvider, "messaging");

// FORMAT: Struct, contract, ...wit_bindgen optionss
wasmcloud_provider_macros::generate!(MessagingProvider, "wasmcloud:messaging", "messaging");

// use crate::exports::wasmcloud::messaging::handler::BrokerMessage;

/// Messaging provider
struct MessagingProvider;

impl MessagingProvider {
    fn handle_message(
        &self,
        msg: crate::exports::wasmcloud::messaging::handler::BrokerMessage,
    ) -> Result<(), String> {
        Ok(())
    }
}

// /// Configuration for connecting a nats client.
// /// More options are available if you use the json than variables in the values string map.
// #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
// struct ConnectionConfig {
//     /// list of topics to subscribe to
//     #[serde(default)]
//     subscriptions: Vec<String>,
//     #[serde(default)]
//     cluster_uris: Vec<String>,
//     #[serde(default)]
//     auth_jwt: Option<String>,
//     #[serde(default)]
//     auth_seed: Option<String>,

//     /// ping interval in seconds
//     #[serde(default)]
//     ping_interval_sec: Option<u16>,
// }

// impl ConnectionConfig {
//     fn merge(&self, extra: &ConnectionConfig) -> ConnectionConfig {
//         let mut out = self.clone();
//         if !extra.subscriptions.is_empty() {
//             out.subscriptions = extra.subscriptions.clone();
//         }
//         // If the default configuration has a URL in it, and then the link definition
//         // also provides a URL, the assumption is to replace/override rather than combine
//         // the two into a potentially incompatible set of URIs
//         if !extra.cluster_uris.is_empty() {
//             out.cluster_uris = extra.cluster_uris.clone();
//         }
//         if extra.auth_jwt.is_some() {
//             out.auth_jwt = extra.auth_jwt.clone()
//         }
//         if extra.auth_seed.is_some() {
//             out.auth_seed = extra.auth_seed.clone()
//         }
//         if extra.ping_interval_sec.is_some() {
//             out.ping_interval_sec = extra.ping_interval_sec
//         }
//         out
//     }
// }

// #[async_trait]
// impl ProviderHandler for MessagingProvider {
//     /// Provider should perform any operations needed for a new link,
//     /// including setting up per-actor resources, and checking authorization.
//     /// If the link is allowed, return true, otherwise return false to deny the link.
//     #[cfg_attr(feature = "otel", instrument(level = "debug", skip(self, ld), fields(actor_id = %ld.actor_id)))]
//     async fn put_link(&self, ld: &LinkDefinition) -> bool {
//         // If the link definition values are empty, use the default connection configuration
//         let config = if ld.values.is_empty() {
//             self.default_config.clone()
//         } else {
//             // create a config from the supplied values and merge that with the existing default
//             match ConnectionConfig::new_from(&ld.values) {
//                 Ok(cc) => self.default_config.merge(&cc),
//                 Err(e) => {
//                     error!(error = %e, "Failed to build connection configuration");
//                     return false;
//                 }
//             }
//         };

//         match self.connect(config, ld).await {
//             Ok(nats_bundle) => {
//                 let mut update_map = self.actors.write().await;
//                 update_map.insert(ld.actor_id.to_string(), nats_bundle);
//                 true
//             }
//             Err(e) => {
//                 error!(error = %e, "Failed to connect to NATS");
//                 false
//             }
//         }
//     }

//     /// Handle notification that a link is dropped: close the connection
//     #[cfg_attr(feature = "otel", instrument(level = "info", skip(self)))]
//     async fn delete_link(&self, actor_id: &str) {
//         let mut aw = self.actors.write().await;

//         if let Some(bundle) = aw.remove(actor_id) {
//             // Note: subscriptions will be closed via Drop on the NatsClientBundle
//             debug!(
//                 "closing [{}] NATS subscriptions for actor [{}]...",
//                 &bundle.sub_handles.len(),
//                 actor_id,
//             );
//         }

//         debug!("finished processing delete link for actor [{}]", actor_id);
//     }

//     /// Handle shutdown request by closing all connections
//     async fn shutdown(&self) {
//         let mut aw = self.actors.write().await;
//         // empty the actor link data and stop all servers
//         aw.clear();
//         // dropping all connections should send unsubscribes and close the connections.
//     }
// }

// #[async_trait::async_trait]
// impl ::provider_sdk::MessageDispatch for NatsMessagingProvider {
//     async fn dispatch<'a>(
//         &'a self,
//         ctx: ::provider_sdk::Context,
//         method: String,
//         body: std::borrow::Cow<'a, [u8]>,
//     ) -> Result<Vec<u8>, ::provider_sdk::error::ProviderInvocationError> {
//         match method.as_str() {
//             "Message.Request" => {
//                 let input: RequestBody = ::provider_sdk::deserialize(&body)?;
//                 let result = self
//                     .request(ctx, input.subject, input.body, input.timeout_ms)
//                     .await
//                     .map_err(|e| {
//                         ::provider_sdk::error::ProviderInvocationError::Provider(e.to_string())
//                     })?;
//                 Ok(::provider_sdk::serialize(&result)?)
//             }
//             "Message.RequestMulti" => {
//                 let input: RequestMultiBody = ::provider_sdk::deserialize(&body)?;
//                 let result = self
//                     .request_multi(
//                         ctx,
//                         input.subject,
//                         input.body,
//                         input.timeout_ms,
//                         input.max_results,
//                     )
//                     .await
//                     .map_err(|e| {
//                         ::provider_sdk::error::ProviderInvocationError::Provider(e.to_string())
//                     })?;
//                 Ok(::provider_sdk::serialize(&result)?)
//             }
//             "Message.Publish" => {
//                 let input: PublishBody = ::provider_sdk::deserialize(&body)?;
//                 let result = self.publish(ctx, input.msg).await.map_err(|e| {
//                     ::provider_sdk::error::ProviderInvocationError::Provider(e.to_string())
//                 })?;
//                 Ok(::provider_sdk::serialize(&result)?)
//             }
//             _ => Err(::provider_sdk::error::InvocationError::Malformed(format!(
//                 "Invalid method name {method}",
//             ))
//             .into()),
//         }
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct RequestBody {
//     subject: String,
//     body: Option<Vec<u8>>,
//     timeout_ms: u32,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct RequestMultiBody {
//     subject: String,
//     body: Option<Vec<u8>>,
//     timeout_ms: u32,
//     max_results: u32,
// }

// #[derive(Debug, Serialize, Deserialize)]
// struct PublishBody {
//     msg: BrokerMessage,
// }

// // // Export the messaging provider
// // export_messaging!(MessagingProvider);
