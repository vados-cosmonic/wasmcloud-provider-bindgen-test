use base64::Engine;
use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;
use wasmcloud_provider_sdk::error::{ProviderError, ProviderResult};

const DEFAULT_NATS_URI: &str = "0.0.0.0:4222";
const ENV_NATS_SUBSCRIPTION: &str = "SUBSCRIPTION";
const ENV_NATS_URI: &str = "URI";
const ENV_NATS_CLIENT_JWT: &str = "CLIENT_JWT";
const ENV_NATS_CLIENT_SEED: &str = "CLIENT_SEED";

/// Configuration for connecting a nats client.
/// More options are available if you use the json than variables in the values string map.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub(crate) struct ConnectionConfig {
    /// list of topics to subscribe to
    #[serde(default)]
    pub(crate) subscriptions: Vec<String>,
    #[serde(default)]
    pub(crate) cluster_uris: Vec<String>,
    #[serde(default)]
    pub(crate) auth_jwt: Option<String>,
    #[serde(default)]
    pub(crate) auth_seed: Option<String>,

    /// ping interval in seconds
    #[serde(default)]
    pub(crate) ping_interval_sec: Option<u16>,
}

impl ConnectionConfig {
    pub(crate) fn merge(&self, extra: &ConnectionConfig) -> ConnectionConfig {
        let mut out = self.clone();
        if !extra.subscriptions.is_empty() {
            out.subscriptions = extra.subscriptions.clone();
        }
        // If the default configuration has a URL in it, and then the link definition
        // also provides a URL, the assumption is to replace/override rather than combine
        // the two into a potentially incompatible set of URIs
        if !extra.cluster_uris.is_empty() {
            out.cluster_uris = extra.cluster_uris.clone();
        }
        if extra.auth_jwt.is_some() {
            out.auth_jwt = extra.auth_jwt.clone()
        }
        if extra.auth_seed.is_some() {
            out.auth_seed = extra.auth_seed.clone()
        }
        if extra.ping_interval_sec.is_some() {
            out.ping_interval_sec = extra.ping_interval_sec
        }
        out
    }
}

impl Default for ConnectionConfig {
    fn default() -> ConnectionConfig {
        ConnectionConfig {
            subscriptions: vec![],
            cluster_uris: vec![DEFAULT_NATS_URI.to_string()],
            auth_jwt: None,
            auth_seed: None,
            ping_interval_sec: None,
        }
    }
}

impl ConnectionConfig {
    pub(crate) fn new_from(values: &[(String, String)]) -> ProviderResult<ConnectionConfig> {
        let mut config = if let Some(config_b64) = get_from_wit_map(values, "config_b64") {
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(config_b64.as_bytes())
                .map_err(|e| {
                    ProviderError::Initialization(format!("invalid base64 encoding: {}", e))
                })?;
            serde_json::from_slice::<ConnectionConfig>(&bytes)
                .map_err(|e| ProviderError::Initialization(format!("corrupt config_b64: {}", e)))?
        } else if let Some(config) = get_from_wit_map(values, "config_json") {
            serde_json::from_str::<ConnectionConfig>(config)
                .map_err(|e| ProviderError::Initialization(format!("corrupt config_json: {}", e)))?
        } else {
            ConnectionConfig::default()
        };

        if let Some(sub) = get_from_wit_map(values, ENV_NATS_SUBSCRIPTION) {
            config
                .subscriptions
                .extend(sub.split(',').map(|s| s.to_string()));
        }
        if let Some(url) = get_from_wit_map(values, ENV_NATS_URI) {
            config.cluster_uris = url.split(',').map(String::from).collect();
        }
        if let Some(jwt) = get_from_wit_map(values, ENV_NATS_CLIENT_JWT) {
            config.auth_jwt = Some(jwt.clone());
        }
        if let Some(seed) = get_from_wit_map(values, ENV_NATS_CLIENT_SEED) {
            config.auth_seed = Some(seed.clone());
        }
        if config.auth_jwt.is_some() && config.auth_seed.is_none() {
            return Err(ProviderError::Initialization(
                "if you specify jwt, you must also specify a seed".to_string(),
            ));
        }
        if config.cluster_uris.is_empty() {
            config.cluster_uris.push(DEFAULT_NATS_URI.to_string());
        }
        Ok(config)
    }
}

/// NatsClientBundles hold a NATS client and information (subscriptions)
/// related to it.
///
/// This struct is necssary because subscriptions are *not* automatically removed on client drop,
/// meaning that we must keep track of all subscriptions to close once the client is done
#[derive(Debug)]
pub(crate) struct NatsClientBundle {
    pub client: async_nats::Client,
    pub sub_handles: Vec<(String, JoinHandle<()>)>,
}

impl Drop for NatsClientBundle {
    fn drop(&mut self) {
        for handle in &self.sub_handles {
            handle.1.abort()
        }
    }
}

/// Get a value from a WIT-ified map
fn get_from_wit_map<'a>(values: &'a [(String, String)], key: &str) -> Option<&'a String> {
    values.iter().find_map(|(k, v)| (k == key).then_some(v))
}
