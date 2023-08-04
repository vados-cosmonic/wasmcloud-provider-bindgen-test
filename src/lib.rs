// NOTE: all this code is borrowed from the WIT-ified provider done by thomastaylor312
// https://github.com/thomastaylor312/nats-messaging-wit

use provider_sdk::core::LinkDefinition;
use wasmcloud::messaging::types::BrokerMessage;

// FORMAT: Struct, contract, ...wit_bindgen optionss
wasmcloud_provider_macros::generate!(MessagingProvider, "wasmcloud:messaging", "messaging");

/// Messaging provider
struct MessagingProvider;

impl MessagingProvider {
    async fn _put_link(&self, ld: &LinkDefinition) {}

    async fn _delete_link(&self, actor_id: &str) {}

    async fn _shutdown(&self) {}

    async fn request(
        &self,
        ctx: provider_sdk::Context,
        subject: String,
        body: Option<Vec<u8>>,
        timeout_ms: u32,
    ) {
    }

    async fn request_multi(
        &self,
        ctx: provider_sdk::Context,
        subject: String,
        body: Option<Vec<u8>>,
        timeout_ms: u32,
        max_results: u32,
    ) {
    }

    async fn publish(&self, ctx: provider_sdk::Context, msg: BrokerMessage) {}

    // fn handle_message(
    //     &self,
    //     msg: crate::exports::wasmcloud::messaging::handler::BrokerMessage,
    // ) -> Result<(), String> {
    //     Ok(())
    // }
}
