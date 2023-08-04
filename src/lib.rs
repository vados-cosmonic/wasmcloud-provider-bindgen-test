use provider_sdk::core::LinkDefinition;

// TODO(FIX): the import below clashes with a `use ...` call from codegen
// use wasmcloud::messaging::types::BrokerMessage;

// FORMAT: Struct, contract, ...wit_bindgen optionss
wasmcloud_provider_macros::generate!(MessagingProvider, "wasmcloud:messaging", "messaging");

/// Messaging provider
struct MessagingProvider;

impl MessagingProvider {
    ////////////////////////////////
    // Wasmcloud-internal methods //
    ////////////////////////////////

    async fn _put_link(&self, ld: &LinkDefinition) -> bool {
        true
    }

    async fn _delete_link(&self, actor_id: &str) {}

    async fn _shutdown(&self) {}

    /////////////////////////
    // Related to Consumer //
    /////////////////////////

    async fn request(
        &self,
        ctx: provider_sdk::Context,
        subject: String,
        body: Option<Vec<u8>>,
        timeout_ms: u32,
    ) -> Result<BrokerMessage, String> {
        Err("Not Implemented".into())
    }

    async fn request_multi(
        &self,
        ctx: provider_sdk::Context,
        subject: String,
        body: Option<Vec<u8>>,
        timeout_ms: u32,
        max_results: u32,
    ) -> Result<Vec<BrokerMessage>, String> {
        Err("Not Implemented".into())
    }

    async fn publish(&self, ctx: provider_sdk::Context, msg: BrokerMessage) -> Result<(), String> {
        Err("Not Implemented".into())
    }
}
