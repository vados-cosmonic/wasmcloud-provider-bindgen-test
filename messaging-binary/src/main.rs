//! Nats implementation for wasmcloud:messaging.
//!
use wasmcloud_provider_sdk::{
    core::HostData,
    load_host_data, start_provider, Context, ProviderHandler,
};

use messaging_binary::NatsMessagingProvider;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // handle lattice control messages and forward rpc to the provider dispatch
    // returns when provider receives a shutdown control message
    let host_data = load_host_data()?;
    let provider = NatsMessagingProvider::from_host_data(host_data.to_owned());
    start_provider(provider, Some("NATS Messaging Provider".to_string()))?;

    eprintln!("NATS messaging provider exiting");
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{ConnectionConfig, NatsMessagingProvider};
    use provider_sdk::{
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
        let prov = generate_provider(host_data);
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
