//! Nats implementation for wasmcloud:messaging.
//!
use wasmcloud_provider_sdk::{load_host_data, start_provider};

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
