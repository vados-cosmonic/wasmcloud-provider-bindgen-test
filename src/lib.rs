wit_bindgen_wasmcloud::generate!(MessagingProvider "messaging");

// TODO(FIX): add comma
// wit_bindgen_wasmcloud::generate!(MessagingProvider, "messaging");

/// Messaging provider
struct MessagingProvider;

// use crate::exports::wasmcloud::messaging::handler::{BrokerMessage, Handler};

// // TODO: Handler trait
// impl Handler for MessagingProvider {
//     fn handle_message(_msg: BrokerMessage) -> Result<(), String> {
//         Ok(())
//     }
// }

// // Export the messaging provider
// export_messaging!(MessagingProvider);
