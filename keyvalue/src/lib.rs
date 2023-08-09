//! Example implementation for wasmcloud keyvalue
//!
//!
//! NOTE: here's what you'd use to add this implementation to the linker:
//!
//! ```
//! use wasmcloud::keyvalue::readwrite::add_to_linker;
//! ```

wit_bindgen_wasmcloud_provider_host::generate!(KeyvalueProvider, "wasmcloud:keyvalue", "keyvalue");

use async_trait::async_trait;
use wasmcloud_provider_sdk::{core::LinkDefinition, Context};

/// Keyvalue provider
struct KeyvalueProvider;

#[async_trait]
impl WasmcloudKeyvalueReadwrite for KeyvalueProvider {
    // NOTE: these functions have to be filled out by hand instead of auto completed
    // due to async_trait mucking things up.
    //
    // You'll get helpful compiler warnings about the methods being missing, but can't auto-complete
    // the missing methods.
    //
    // It's also not immediately obvious that you have to use a Result<Result<_>>, due to the wasmtime::Error
    // that wasmtime::component bindgens

    async fn get(
        &self,
        _ctx: Context,
        _key: String,
    ) -> Result<Result<Option<String>, String>, wasmtime::Error> {
        todo!()
    }
    async fn set(
        &self,
        _ctx: Context,
        _key: String,
        _value: String,
    ) -> Result<Result<(), String>, wasmtime::Error> {
        todo!()
    }

    async fn delete(
        &self,
        _ctx: Context,
        _key: String,
    ) -> Result<Result<(), String>, wasmtime::Error> {
        todo!()
    }

    async fn exists(
        &self,
        _ctx: Context,
        _key: String,
    ) -> Result<Result<bool, String>, wasmtime::Error> {
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
    //
    // It's also not immediately obvious that you have to use a Result<Result<_>>, due to the wasmtime::Error
    // that wasmtime::component bindgens

    async fn increment(
        &self,
        _ctx: Context,
        _key: String,
        _amount: u32,
    ) -> Result<Result<u32, String>, wasmtime::Error> {
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
