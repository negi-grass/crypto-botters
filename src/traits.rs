use std::fmt::Debug;
use generic_api_client::{http, websocket};

/// A `trait` that represents an option which can be set when creating handlers
pub trait HandlerOption: Default {
    type Options: HandlerOptions<OptionItem=Self>;
}

/// Set of [HandlerOption] s
pub trait HandlerOptions: Default + Clone + Debug {
    /// The element of this set
    type OptionItem: HandlerOption<Options=Self>;

    fn update(&mut self, option: Self::OptionItem);
}

/// A `trait` that shows the implementing type is able to create [http::RequestHandler]s
pub trait HttpOption<'a, R, B>: HandlerOption {
    type RequestHandler: http::RequestHandler<B>;

    fn request_handler(options: Self::Options) -> Self::RequestHandler;
}

/// A `trait` that shows the implementing type is able to create [websocket::WebSocketHandler]s
pub trait WebSocketOption<H>: HandlerOption {
    type WebSocketHandler: websocket::WebSocketHandler;

    fn websocket_handler(handler: H, options: Self::Options) -> Self::WebSocketHandler;
}
