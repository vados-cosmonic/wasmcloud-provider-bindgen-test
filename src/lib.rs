// FORMAT: generate!(impl Struct, ...wit_bindgen options)
wasmcloud_provider_macros::generate!(MessagingProvider, "messaging");

use wasmcloud_provider_sdk::core::LinkDefinition;
use wasmcloud::messaging::types::BrokerMessage;

/// Messaging provider
struct MessagingProvider;

impl MessagingProvider {
    ////////////////////////////////
    // Wasmcloud-internal methods //
    ////////////////////////////////

    async fn _put_link(&self, _ld: &LinkDefinition) -> bool {
        true
    }

    async fn _delete_link(&self, _actor_id: &str) {}

    async fn _shutdown(&self) {}

    /////////////////////////
    // Related to Consumer //
    /////////////////////////

    async fn request(
        &self,
        _ctx: wasmcloud_provider_sdk::Context,
        _subject: String,
        _body: Option<Vec<u8>>,
        _timeout_ms: u32,
    ) -> Result<BrokerMessage, String> {
        Err("Not Implemented".into())
    }

    async fn request_multi(
        &self,
        _ctx: wasmcloud_provider_sdk::Context,
        _subject: String,
        _body: Option<Vec<u8>>,
        _timeout_ms: u32,
        _max_results: u32,
    ) -> Result<Vec<BrokerMessage>, String> {
        Err("Not Implemented".into())
    }

    async fn publish(
        &self,
        _ctx: wasmcloud_provider_sdk::Context,
        _msg: BrokerMessage,
    ) -> Result<(), String> {
        Err("Not Implemented".into())
    }
}

impl crate::exports::wasmcloud::messaging::handler::Handler for MessagingProvider {
    fn handle_message(
        _msg: exports::wasmcloud::messaging::handler::BrokerMessage,
    ) -> Result<(), wit_bindgen::rt::string::String> {
        todo!()
    }
}

export_messaging!(MessagingProvider);
