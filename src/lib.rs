// NOTE: all this code is borrowed from the WIT-ified provider done by thomastaylor312
// https://github.com/thomastaylor312/nats-messaging-wit

// FORMAT: Struct, contract, ...wit_bindgen optionss
wasmcloud_provider_macros::generate!(MessagingProvider, "wasmcloud:messaging", "messaging");

/// Messaging provider
struct MessagingProvider;

// impl MessagingProvider {
//     fn handle_message(
//         &self,
//         msg: crate::exports::wasmcloud::messaging::handler::BrokerMessage,
//     ) -> Result<(), String> {
//         Ok(())
//     }
// }

