// Warn (almost) everything. see https://doc.rust-lang.org/rustc/lints/groups.html
#![warn(
    future_incompatible,
    let_underscore,
    nonstandard_style,
    rust_2018_compatibility,
    rust_2018_idioms,
    rust_2021_compatibility,
    unused,
    missing_docs,
)]

//! # Generic-API-Client
//! This is a crate for interacting with HTTP/HTTPS/WebSocket APIs.
//! It is named "generic" because you can use the **same** client to interact with **multiple different**
//! APIs with, different authentication methods, data formats etc.
//!
//! This crate  provides
//! - [Client][http::Client] A HTTP/HTTPS client
//! - [WebSocketConnection][websocket::WebSocketConnection] A `struct` to manager WebSocket connections
//! - [RequestHandler][http::RequestHandler] A `trait` for implementing features like authentication on your requests
//! - [WebSocketHandler][websocket::WebSocketHandler] A `trait` that is used to handle messages etc.. for a WebSocket Connection.
//!
//! For a more detailed documentation, see the links above.

/// Module for interacting with HTTP/HTTPS APIs.
pub mod http;
/// Module for interacting with WebSocket APIs.
pub mod websocket;
