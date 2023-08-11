#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
pub struct Messaging {
    interface0: exports::wasmcloud::messaging::handler::Handler,
}
const _: () = {
    use wasmtime::component::__internal::anyhow;
    impl Messaging {
        pub fn add_to_linker<T, U>(
            linker: &mut wasmtime::component::Linker<T>,
            get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
        ) -> wasmtime::Result<()>
        where
            U: wasmcloud::messaging::types::Host + wasmcloud::messaging::consumer::Host,
        {
            wasmcloud::messaging::types::add_to_linker(linker, get)?;
            wasmcloud::messaging::consumer::add_to_linker(linker, get)?;
            Ok(())
        }
        /// Instantiates the provided `module` using the specified
        /// parameters, wrapping up the result in a structure that
        /// translates between wasm and the host.
        pub fn instantiate<T>(
            mut store: impl wasmtime::AsContextMut<Data = T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<T>,
        ) -> wasmtime::Result<(Self, wasmtime::component::Instance)> {
            let instance = linker.instantiate(&mut store, component)?;
            Ok((Self::new(store, &instance)?, instance))
        }
        /// Instantiates a pre-instantiated module using the specified
        /// parameters, wrapping up the result in a structure that
        /// translates between wasm and the host.
        pub fn instantiate_pre<T>(
            mut store: impl wasmtime::AsContextMut<Data = T>,
            instance_pre: &wasmtime::component::InstancePre<T>,
        ) -> wasmtime::Result<(Self, wasmtime::component::Instance)> {
            let instance = instance_pre.instantiate(&mut store)?;
            Ok((Self::new(store, &instance)?, instance))
        }
        /// Low-level creation wrapper for wrapping up the exports
        /// of the `instance` provided in this structure of wasm
        /// exports.
        ///
        /// This function will extract exports from the `instance`
        /// defined within `store` and wrap them all up in the
        /// returned structure which can be used to interact with
        /// the wasm module.
        pub fn new(
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<Self> {
            let mut store = store.as_context_mut();
            let mut exports = instance.exports(&mut store);
            let mut __exports = exports.root();
            let interface0 = exports::wasmcloud::messaging::handler::Handler::new(
                &mut __exports
                    .instance("wasmcloud:messaging/handler@0.1.0")
                    .ok_or_else(|| ::anyhow::__private::must_use({
                        let error = ::anyhow::__private::format_err(
                            format_args!(
                                "exported instance `wasmcloud:messaging/handler@0.1.0` not present",
                            ),
                        );
                        error
                    }))?,
            )?;
            Ok(Messaging { interface0 })
        }
        pub fn wasmcloud_messaging_handler(
            &self,
        ) -> &exports::wasmcloud::messaging::handler::Handler {
            &self.interface0
        }
    }
};

pub mod wasmcloud {
    pub mod messaging {
        #[allow(clippy::all)]
        pub mod types {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;
            #[component(record)]
            pub struct BrokerMessage {
                #[component(name = "subject")]
                pub subject: String,
                #[component(name = "body")]
                pub body: Option<Vec<u8>>,
                #[component(name = "reply-to")]
                pub reply_to: Option<String>,
            }
            #[automatically_derived]
            impl ::core::clone::Clone for BrokerMessage {
                #[inline]
                fn clone(&self) -> BrokerMessage {
                    BrokerMessage {
                        subject: ::core::clone::Clone::clone(&self.subject),
                        body: ::core::clone::Clone::clone(&self.body),
                        reply_to: ::core::clone::Clone::clone(&self.reply_to),
                    }
                }
            }
            unsafe impl wasmtime::component::Lower for BrokerMessage {
                #[inline]
                fn lower<T>(
                    &self,
                    cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    dst: &mut std::mem::MaybeUninit<Self::Lower>,
                ) -> wasmtime::component::__internal::anyhow::Result<()> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    wasmtime::component::Lower::lower(
                        &self.subject,
                        cx,
                        ty.fields[0usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).subject)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.body,
                        cx,
                        ty.fields[1usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).body)
                                }
                            }
                        },
                    )?;
                    wasmtime::component::Lower::lower(
                        &self.reply_to,
                        cx,
                        ty.fields[2usize].ty,
                        {
                            #[allow(unused_unsafe)]
                            {
                                unsafe {
                                    use ::wasmtime::component::__internal::MaybeUninitExt;
                                    let m: &mut std::mem::MaybeUninit<_> = dst;
                                    m.map(|p| &raw mut (*p).reply_to)
                                }
                            }
                        },
                    )?;
                    Ok(())
                }
                #[inline]
                fn store<T>(
                    &self,
                    cx: &mut wasmtime::component::__internal::LowerContext<'_, T>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    mut offset: usize,
                ) -> wasmtime::component::__internal::anyhow::Result<()> {
                    if true {
                        if !(offset
                            % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                as usize) == 0)
                        {
                            ::core::panicking::panic(
                                "assertion failed: offset % (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                            )
                        }
                    }
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    wasmtime::component::Lower::store(
                        &self.subject,
                        cx,
                        ty.fields[0usize].ty,
                        <String as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.body,
                        cx,
                        ty.fields[1usize].ty,
                        <Option<Vec<u8>> as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    wasmtime::component::Lower::store(
                        &self.reply_to,
                        cx,
                        ty.fields[2usize].ty,
                        <Option<String> as wasmtime::component::ComponentType>::ABI
                            .next_field32_size(&mut offset),
                    )?;
                    Ok(())
                }
            }
            unsafe impl wasmtime::component::Lift for BrokerMessage {
                #[inline]
                fn lift(
                    cx: &wasmtime::component::__internal::LiftContext<'_>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    src: &Self::Lower,
                ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    Ok(Self {
                        subject: <String as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[0usize].ty,
                            &src.subject,
                        )?,
                        body: <Option<
                            Vec<u8>,
                        > as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[1usize].ty,
                            &src.body,
                        )?,
                        reply_to: <Option<
                            String,
                        > as wasmtime::component::Lift>::lift(
                            cx,
                            ty.fields[2usize].ty,
                            &src.reply_to,
                        )?,
                    })
                }
                #[inline]
                fn load(
                    cx: &wasmtime::component::__internal::LiftContext<'_>,
                    ty: wasmtime::component::__internal::InterfaceType,
                    bytes: &[u8],
                ) -> wasmtime::component::__internal::anyhow::Result<Self> {
                    let ty = match ty {
                        wasmtime::component::__internal::InterfaceType::Record(i) => {
                            &cx.types[i]
                        }
                        _ => wasmtime::component::__internal::bad_type_info(),
                    };
                    if true {
                        if !((bytes.as_ptr() as usize)
                            % (<Self as wasmtime::component::ComponentType>::ALIGN32
                                as usize) == 0)
                        {
                            ::core::panicking::panic(
                                "assertion failed: (bytes.as_ptr() as usize) %\\n        (<Self as wasmtime::component::ComponentType>::ALIGN32 as usize) == 0",
                            )
                        }
                    }
                    let mut offset = 0;
                    Ok(Self {
                        subject: <String as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[0usize].ty,
                            &bytes[<String as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<String as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        body: <Option<
                            Vec<u8>,
                        > as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[1usize].ty,
                            &bytes[<Option<
                                Vec<u8>,
                            > as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<Option<
                                Vec<u8>,
                            > as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                        reply_to: <Option<
                            String,
                        > as wasmtime::component::Lift>::load(
                            cx,
                            ty.fields[2usize].ty,
                            &bytes[<Option<
                                String,
                            > as wasmtime::component::ComponentType>::ABI
                                .next_field32_size(
                                    &mut offset,
                                )..][..<Option<
                                String,
                            > as wasmtime::component::ComponentType>::SIZE32],
                        )?,
                    })
                }
            }
            const _: () = {
                #[doc(hidden)]
                #[repr(C)]
                pub struct LowerBrokerMessage<T0: Copy, T1: Copy, T2: Copy> {
                    subject: T0,
                    body: T1,
                    reply_to: T2,
                    _align: [wasmtime::ValRaw; 0],
                }
                #[automatically_derived]
                impl<
                    T0: ::core::clone::Clone + Copy,
                    T1: ::core::clone::Clone + Copy,
                    T2: ::core::clone::Clone + Copy,
                > ::core::clone::Clone for LowerBrokerMessage<T0, T1, T2> {
                    #[inline]
                    fn clone(&self) -> LowerBrokerMessage<T0, T1, T2> {
                        LowerBrokerMessage {
                            subject: ::core::clone::Clone::clone(&self.subject),
                            body: ::core::clone::Clone::clone(&self.body),
                            reply_to: ::core::clone::Clone::clone(&self.reply_to),
                            _align: ::core::clone::Clone::clone(&self._align),
                        }
                    }
                }
                #[automatically_derived]
                impl<
                    T0: ::core::marker::Copy + Copy,
                    T1: ::core::marker::Copy + Copy,
                    T2: ::core::marker::Copy + Copy,
                > ::core::marker::Copy for LowerBrokerMessage<T0, T1, T2> {}
                unsafe impl wasmtime::component::ComponentType for BrokerMessage {
                    type Lower = LowerBrokerMessage<
                        <String as wasmtime::component::ComponentType>::Lower,
                        <Option<Vec<u8>> as wasmtime::component::ComponentType>::Lower,
                        <Option<String> as wasmtime::component::ComponentType>::Lower,
                    >;
                    const ABI: wasmtime::component::__internal::CanonicalAbiInfo = wasmtime::component::__internal::CanonicalAbiInfo::record_static(
                        &[
                            <String as wasmtime::component::ComponentType>::ABI,
                            <Option<Vec<u8>> as wasmtime::component::ComponentType>::ABI,
                            <Option<String> as wasmtime::component::ComponentType>::ABI,
                        ],
                    );
                    #[inline]
                    fn typecheck(
                        ty: &wasmtime::component::__internal::InterfaceType,
                        types: &wasmtime::component::__internal::ComponentTypes,
                    ) -> wasmtime::component::__internal::anyhow::Result<()> {
                        wasmtime::component::__internal::typecheck_record(
                            ty,
                            types,
                            &[
                                (
                                    "subject",
                                    <String as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "body",
                                    <Option<
                                        Vec<u8>,
                                    > as wasmtime::component::ComponentType>::typecheck,
                                ),
                                (
                                    "reply-to",
                                    <Option<
                                        String,
                                    > as wasmtime::component::ComponentType>::typecheck,
                                ),
                            ],
                        )
                    }
                }
            };
            impl core::fmt::Debug for BrokerMessage {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("BrokerMessage")
                        .field("subject", &self.subject)
                        .field("body", &self.body)
                        .field("reply-to", &self.reply_to)
                        .finish()
                }
            }
            const _: () = {
                if !(32 == <BrokerMessage as wasmtime::component::ComponentType>::SIZE32)
                {
                    ::core::panicking::panic(
                        "assertion failed: 32 == <BrokerMessage as wasmtime::component::ComponentType>::SIZE32",
                    )
                }
                if !(4 == <BrokerMessage as wasmtime::component::ComponentType>::ALIGN32)
                {
                    ::core::panicking::panic(
                        "assertion failed: 4 == <BrokerMessage as wasmtime::component::ComponentType>::ALIGN32",
                    )
                }
            };
            pub trait Host {}
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                U: Host,
            {
                let mut inst = linker.instance("wasmcloud:messaging/types@0.1.0")?;
                Ok(())
            }
        }
        #[allow(clippy::all)]
        pub mod consumer {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;
            pub type BrokerMessage = super::super::super::wasmcloud::messaging::types::BrokerMessage;
            const _: () = {
                if !(32 == <BrokerMessage as wasmtime::component::ComponentType>::SIZE32)
                {
                    ::core::panicking::panic(
                        "assertion failed: 32 == <BrokerMessage as wasmtime::component::ComponentType>::SIZE32",
                    )
                }
                if !(4 == <BrokerMessage as wasmtime::component::ComponentType>::ALIGN32)
                {
                    ::core::panicking::panic(
                        "assertion failed: 4 == <BrokerMessage as wasmtime::component::ComponentType>::ALIGN32",
                    )
                }
            };
            pub trait Host {
                fn request(
                    &mut self,
                    subject: String,
                    body: Option<Vec<u8>>,
                    timeout_ms: u32,
                ) -> wasmtime::Result<Result<BrokerMessage, String>>;
                fn request_multi(
                    &mut self,
                    subject: String,
                    body: Option<Vec<u8>>,
                    timeout_ms: u32,
                    max_results: u32,
                ) -> wasmtime::Result<Result<Vec<BrokerMessage>, String>>;
                fn publish(
                    &mut self,
                    msg: BrokerMessage,
                ) -> wasmtime::Result<Result<(), String>>;
            }
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                U: Host,
            {
                let mut inst = linker.instance("wasmcloud:messaging/consumer@0.1.0")?;
                inst.func_wrap(
                    "request",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0, arg1, arg2): (String, Option<Vec<u8>>, u32)|
                    {
                        let host = get(caller.data_mut());
                        let r = host.request(arg0, arg1, arg2);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "request-multi",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0, arg1, arg2, arg3): (String, Option<Vec<u8>>, u32, u32)|
                    {
                        let host = get(caller.data_mut());
                        let r = host.request_multi(arg0, arg1, arg2, arg3);
                        Ok((r?,))
                    },
                )?;
                inst.func_wrap(
                    "publish",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (BrokerMessage,)|
                    {
                        let host = get(caller.data_mut());
                        let r = host.publish(arg0);
                        Ok((r?,))
                    },
                )?;
                Ok(())
            }
        }
    }
}

pub mod exports {
    pub mod wasmcloud {
        pub mod messaging {
            #[allow(clippy::all)]
            pub mod handler {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::anyhow;
                pub type BrokerMessage = super::super::super::super::wasmcloud::messaging::types::BrokerMessage;
                const _: () = {
                    if !(32
                        == <BrokerMessage as wasmtime::component::ComponentType>::SIZE32)
                    {
                        ::core::panicking::panic(
                            "assertion failed: 32 == <BrokerMessage as wasmtime::component::ComponentType>::SIZE32",
                        )
                    }
                    if !(4
                        == <BrokerMessage as wasmtime::component::ComponentType>::ALIGN32)
                    {
                        ::core::panicking::panic(
                            "assertion failed: 4 == <BrokerMessage as wasmtime::component::ComponentType>::ALIGN32",
                        )
                    }
                };
                pub struct Handler {
                    handle_message: wasmtime::component::Func,
                }
                impl Handler {
                    pub fn new(
                        __exports: &mut wasmtime::component::ExportInstance<'_, '_>,
                    ) -> wasmtime::Result<Handler> {
                        let handle_message = *__exports
                            .typed_func::<
                                (&BrokerMessage,),
                                (Result<(), String>,),
                            >("handle-message")?
                            .func();
                        Ok(Handler { handle_message })
                    }
                    pub fn call_handle_message<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: &BrokerMessage,
                    ) -> wasmtime::Result<Result<(), String>> {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (&BrokerMessage,),
                                (Result<(), String>,),
                            >::new_unchecked(self.handle_message)
                        };
                        let (ret0,) = callee.call(store.as_context_mut(), (arg0,))?;
                        callee.post_return(store.as_context_mut())?;
                        Ok(ret0)
                    }
                }
            }
        }
    }
}
const _: &str = "// Message broker interface\n// This is a phase 1 interface, and is subject to change\n// This interface is used to send and receive messages from a message broker\n\n// Note that in this phase 1 interface, subscriptions are defined out of band from this\n// component. Components themselves cannot establish or terminate subscriptions. This may\n// change for phase 2.\npackage wasmcloud:messaging@0.1.0\n\n// Types common to message broker interactions\ninterface types {\n    // A message sent to or received from a broker\n    record broker-message {\n        subject: string,\n        body: option<list<u8>>,\n        reply-to: option<string>,\n    }\n}\n\ninterface handler {\n    use types.{broker-message}\n\n    // Callback handled to invoke a function when a message is received from a subscription\n    handle-message: func(msg: broker-message) -> result<_, string>\n}\n\ninterface consumer {\n    use types.{broker-message}\n\n    // Perform a request operation on a subject\n    request: func(subject: string, body: option<list<u8>>, timeout-ms: u32) -> result<broker-message, string>\n\n    // Performs a request and collects multiple responses. If a non-zero timeout is supplied, this will finish a collection at that time, unless\n    // maximum results is reached first. If both timeout-ms and max-results are 0, the provider will choose when to terminate\n    request-multi: func(subject: string, body: option<list<u8>>, timeout-ms: u32, max-results: u32) -> result<list<broker-message>, string>\n\n    // Publish a message to a subject without awaiting a response\n    publish: func(msg: broker-message) -> result<_, string>\n}\n\nworld messaging {\n    import consumer\n    export handler\n}\n";
use wasmcloud_provider_sdk::core::LinkDefinition;
use wasmcloud::messaging::types::BrokerMessage;
/// Messaging provider
struct MessagingProvider;
