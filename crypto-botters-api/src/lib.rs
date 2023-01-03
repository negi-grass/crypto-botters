//! This crate is meant to be used by the `crypto-botters` crate.
//! This crate only exists to prevent cyclic dependencies.

use std::fmt::Debug;

/// A `trait` that represents an option which can be set when creating handlers
pub trait HandlerOption: Default {
    type Options: HandlerOptions<OptionItem=Self>;

    // /// Create a [generic_api_client::http::RequestHandler] from some options.
    // #[allow(unused_variables)]
    // fn request_handler<'a, R>(options: Self::Options) -> Self::RequestHandler where Self: HttpOption<'a, R> {
    //     unimplemented!()
    // }

    // /// Create a [generic_api_client::websocket::WebSocketHandler] from some options.
    // #[allow(unused_variables)]
    // fn websocket_handler<H>(handler: H, options: Self::Options) -> Self::WebSocketHandler where Self: WebSocketOption<H> {
    //     unimplemented!()
    // }
}

/// Set of [HandlerOption] s
pub trait HandlerOptions: Default + Clone + Debug {
    /// The element of this set
    type OptionItem: HandlerOption<Options=Self>;

    fn update(&mut self, option: Self::OptionItem);
}

/// A `trait` that shows the implementing type is able to create [generic_api_client::http::RequestHandler]s
pub trait HttpOption<'a, R>: HandlerOption {
    type RequestHandler;

    fn request_handler(options: Self::Options) -> Self::RequestHandler;
}

/// A `trait` that shows the implementing type is able to create [generic_api_client::websocket::WebSockethandler]s
pub trait WebSocketOption<H>: HandlerOption {
    type WebSocketHandler;

    fn websocket_handler(handler: H, options: Self::Options) -> Self::WebSocketHandler;
}
