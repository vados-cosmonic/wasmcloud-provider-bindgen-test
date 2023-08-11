//! Nats implementation for wasmcloud:messaging.
//!
use wasmcloud_provider_sdk::start_provider;

use keyvalue_binary::KeyvalueProvider;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // handle lattice control messages and forward rpc to the provider dispatch
    // returns when provider receives a shutdown control message
    let provider = KeyvalueProvider;
    start_provider(provider, Some("Keyvalue Provider".to_string()))?;

    eprintln!("keyvalue provider exiting");
    Ok(())
}
