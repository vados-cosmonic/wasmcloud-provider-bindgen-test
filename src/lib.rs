wit_bindgen_wasmcloud::generate!("messaging");

/// Messaging provider
struct MessagingProvider;

// TODO: Handler trait

// Export the messaging provider
export_messaging!(MessagingProvider);
