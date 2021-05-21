// See: https://github.com/rust-lang/rust/issues/44732#issuecomment-488766871
//
#![cfg_attr(nightly, feature(doc_cfg, external_doc))]
#![cfg_attr(nightly, doc(include = "../README.md"))]
#![doc = ""] // empty doc line to handle missing doc warning when the feature is missing.
#![doc(html_root_url = "https://docs.rs/ws_stream_wasm")]
#![forbid(unsafe_code)]
#![allow(clippy::suspicious_else_formatting, clippy::needless_return)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    nonstandard_style,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_qualifications,
    single_use_lifetimes,
    unreachable_pub,
    variant_size_differences
)]

mod error;
mod ws_event;
mod ws_message;
mod ws_meta;
mod ws_state;
mod ws_stream;
mod ws_stream_io;

pub use {
    error::WsErr,
    ws_event::{CloseEvent, WsEvent},
    ws_message::WsMessage,
    ws_meta::WsMeta,
    ws_state::WsState,
    ws_stream::WsStream,
    ws_stream_io::WsStreamIo,
};

mod import {
    pub(crate) use {
        async_io_stream::IoStream,
        futures::{
            prelude::{Sink, Stream},
            ready, FutureExt, StreamExt,
        },
        js_sys::Array,
        js_sys::{ArrayBuffer, Uint8Array},
        pharos::{Filter, Observable, Observe, ObserveConfig, PharErr, SharedPharos},
        send_wrapper::SendWrapper,
        std::{
            cell::RefCell,
            convert::{TryFrom, TryInto},
            pin::Pin,
            rc::Rc,
        },
        std::{
            collections::VecDeque,
            fmt,
            future::Future,
            io,
            task::{Context, Poll, Waker},
        },
        thiserror::Error,
        wasm_bindgen::{closure::Closure, JsCast, JsValue, UnwrapThrowExt},
        wasm_bindgen_futures::spawn_local,
        web_sys::{BinaryType, Blob, CloseEvent as JsCloseEvt, DomException, WebSocket, *},
    };
}

use import::*;

/// Helper function to reduce code bloat
//
pub(crate) fn notify(pharos: SharedPharos<WsEvent>, evt: WsEvent) {
    let notify = async move {
        pharos
            .notify(evt)
            .await
            .map_err(|e| unreachable!("{:?}", e))
            .unwrap(); // only happens if we closed it.
    };

    spawn_local(notify);
}
