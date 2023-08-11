wit_bindgen_wasmcloud_provider_binary::generate!(
    KeyvalueProvider,
    "wasmcloud:keyvalue",
    "keyvalue"
);

use async_trait::async_trait;
use wasmcloud_provider_sdk::{Context, core::LinkDefinition};

/// Implementation for wasmcloud:keyvalue
#[derive(Default, Clone)]
pub struct KeyvalueProvider;

#[async_trait]
impl WasmcloudKeyvalueReadwrite for KeyvalueProvider {
    // NOTE: these functions have to be filled out by hand instead of auto completed
    // due to async_trait mucking things up.
    //
    // You'll get helpful compiler warnings about the methods being missing, but can't auto-complete
    // the missing methods.

    async fn get(&self, _ctx: Context, _key: String) -> Result<Option<String>, String> {
        todo!()
    }
    async fn set(&self, _ctx: Context, _key: String, _value: String) -> Result<(), String> {
        todo!()
    }

    async fn delete(&self, _ctx: Context, _key: String) -> Result<(), String> {
        todo!()
    }

    async fn exists(&self, _ctx: Context, _key: String) -> Result<bool, String> {
        todo!()
    }
}

#[async_trait]
impl WasmcloudKeyvalueAtomic for KeyvalueProvider {
    // NOTE: these functions have to be filled out by hand instead of auto completed
    // due to async_trait mucking things up.
    //
    // You'll get helpful compiler warnings about the methods being missing, but can't auto-complete
    // the missing methods.

    async fn increment(&self, _ctx: Context, _key: String, _amount: u32) -> Result<u32, String> {
        todo!()
    }
}

#[async_trait]
impl WasmcloudCapabilityProvider for KeyvalueProvider {
    // NOTE: these functions have to be filled out by hand instead of auto completed
    // due to async_trait mucking things up.
    //
    // You'll get helpful compiler warnings about the methods being missing, but can't auto-complete
    // the missing methods.

    async fn put_link(&self, _ld: &LinkDefinition) -> bool {
        todo!()
    }

    async fn delete_link(&self, _actor_id: &str) {
        todo!()
    }

    async fn shutdown(&self) {
        todo!()
    }
}
