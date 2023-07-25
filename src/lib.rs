#![cfg_attr(docsrs, feature(doc_cfg))]

use generic_api_client::{http::{self, *}, websocket::*};
use serde::Serialize;
use traits::*;

pub use generic_api_client;
pub use exchanges::*;

mod exchanges;
pub mod traits;

// very long type, make it a macro
macro_rules! request_return_type {
    ($lt:lifetime, $Response:ty, $Options:ty,  $Body:ty) => {
        Result<
            <<$Options as HttpOption<$lt, $Response, $Body>>::RequestHandler as RequestHandler<$Body>>::Successful,
            RequestError<
                <<$Options as HttpOption<$lt, $Response, $Body>>::RequestHandler as RequestHandler<$Body>>::BuildError,
                <<$Options as HttpOption<$lt, $Response, $Body>>::RequestHandler as RequestHandler<$Body>>::Unsuccessful,
            >,
        >
    };
}

#[derive(Default, Debug, Clone)]
pub struct Client {
    client: http::Client,
    #[cfg(feature = "binance")]
    binance: binance::BinanceOptions,
    #[cfg(feature = "bitbank")]
    bitbank: bitbank::BitbankOptions,
    #[cfg(feature = "bitflyer")]
    bitflyer: bitflyer::BitFlyerOptions,
    #[cfg(feature = "bybit")]
    bybit: bybit::BybitOptions,
    #[cfg(feature = "coincheck")]
    coincheck: coincheck::CoincheckOptions,
}

impl Client {
    /// Creates a new [Client].
    pub fn new() -> Self {
        Self::default()
    }

    /// Update the default options for this [Client]
    #[inline(always)]
    pub fn update_default_option<O>(&mut self, option: O)
    where
        O: HandlerOption,
        Self: GetOptions<O::Options>,
    {
        self.default_options_mut().update(option);
    }

    #[inline]
    fn merged_options<O>(&self, options: impl IntoIterator<Item=O>) -> O::Options
    where
        O: HandlerOption,
        Self:GetOptions<O::Options>,
    {
        let mut default_options = self.default_options().clone();
        for option in options {
            default_options.update(option);
        }
        default_options
    }

    /// see [http::Client::request()]
    #[inline(always)]
    pub async fn request<'a, R, O, Q, B>(&self, method: Method, url: &str, query: Option<&Q>, body: Option<B>, options: impl IntoIterator<Item=O>)
        -> request_return_type!('a, R, O, B)
    where
        O: HttpOption<'a, R, B>,
        O::RequestHandler: RequestHandler<B>,
        Self: GetOptions<O::Options>,
        Q: Serialize + ?Sized,
    {
        self.client.request(method, url, query, body, &O::request_handler(self.merged_options(options))).await
    }

    /// see [http::Client::get()]
    #[inline(always)]
    pub async fn get<'a, R, O, Q>(&self, url: &str, query: Option<&Q>, options: impl IntoIterator<Item=O>) -> request_return_type!('a, R, O, ())
    where
        O: HttpOption<'a, R, ()>,
        O::RequestHandler: RequestHandler<()>,
        Self: GetOptions<O::Options>,
        Q: Serialize + ?Sized,
    {
        self.client.get(url, query, &O::request_handler(self.merged_options(options))).await
    }

    /// see [http::Client::get_no_query()]
    #[inline(always)]
    pub async fn get_no_query<'a, R, O>(&self, url: &str, options: impl IntoIterator<Item=O>) -> request_return_type!('a, R, O, ())
    where
        O: HttpOption<'a, R, ()>,
        O::RequestHandler: RequestHandler<()>,
        Self: GetOptions<O::Options>,
    {
        self.client.get_no_query(url, &O::request_handler(self.merged_options(options))).await
    }

    /// see [http::Client::post()]
    #[inline(always)]
    pub async fn post<'a, R, O, B>(&self, url: &str, body: Option<B>, options: impl IntoIterator<Item=O>)
        -> request_return_type!('a, R, O, B)
    where
        O: HttpOption<'a, R, B>,
        O::RequestHandler: RequestHandler<B>,
        Self: GetOptions<O::Options>,
    {
        self.client.post(url, body, &O::request_handler(self.merged_options(options))).await
    }

    /// see [http::Client::post_no_body()]
    #[inline(always)]
    pub async fn post_no_body<'a, R, O>(&self, url: &str, options: impl IntoIterator<Item=O>)
        -> request_return_type!('a, R, O, ())
    where
        O: HttpOption<'a, R, ()>,
        O::RequestHandler: RequestHandler<()>,
        Self: GetOptions<O::Options>,
    {
        self.client.post_no_body(url, &O::request_handler(self.merged_options(options))).await
    }

    /// see [http::Client::put()]
    #[inline(always)]
    pub async fn put<'a, R, O, B>(&self, url: &str, body: Option<B>, options: impl IntoIterator<Item=O>)
        -> request_return_type!('a, R, O, B)
    where
        O: HttpOption<'a, R, B>,
        O::RequestHandler: RequestHandler<B>,
        Self: GetOptions<O::Options>,
    {
        self.client.put(url, body, &O::request_handler(self.merged_options(options))).await
    }

    /// see [http::Client::put_no_body()]
    #[inline(always)]
    pub async fn put_no_body<'a, R, O>(&self, url: &str, options: impl IntoIterator<Item=O>)
        -> request_return_type!('a, R, O, ())
    where
        O: HttpOption<'a, R, ()>,
        O::RequestHandler: RequestHandler<()>,
        Self: GetOptions<O::Options>,
    {
        self.client.put_no_body(url, &O::request_handler(self.merged_options(options))).await
    }

    /// see [http::Client::delete()]
    #[inline(always)]
    pub async fn delete<'a, R, O, Q>(&self, url: &str, query: Option<&Q>, options: impl IntoIterator<Item=O>) -> request_return_type!('a, R, O, ())
    where
        O: HttpOption<'a, R, ()>,
        O::RequestHandler: RequestHandler<()>,
        Self: GetOptions<O::Options>,
        Q: Serialize + ?Sized,
    {
        self.client.delete(url, query, &O::request_handler(self.merged_options(options))).await
    }

    /// see [http::Client::delete_no_query()]
    #[inline(always)]
    pub async fn delete_no_query<'a, R, O>(&self, url: &str, options: impl IntoIterator<Item=O>) -> request_return_type!('a, R, O, ())
    where
        O: HttpOption<'a, R, ()>,
        O::RequestHandler: RequestHandler<()>,
        Self: GetOptions<O::Options>,
    {
        self.client.delete_no_query(url, &O::request_handler(self.merged_options(options))).await
    }

    #[inline(always)]
    pub async fn websocket<O, H>(&self, url: &str, handler: H, options: impl IntoIterator<Item=O>) -> Result<WebSocketConnection<O::WebSocketHandler>, TungsteniteError>
    where
        O: WebSocketOption<H>,
        O::WebSocketHandler: WebSocketHandler,
        Self: GetOptions<O::Options>,
    {
        WebSocketConnection::new(url, O::websocket_handler(handler, self.merged_options(options))).await
    }
}

pub trait GetOptions<O: HandlerOptions> {
    fn default_options(&self) -> &O;
    fn default_options_mut(&mut self) -> &mut O;
}

#[cfg(feature = "binance")]
#[cfg_attr(docsrs, doc(cfg(feature = "binance")))]
impl GetOptions<binance::BinanceOptions> for Client {
    #[inline(always)]
    fn default_options(&self) -> &binance::BinanceOptions {
        &self.binance
    }

    #[inline(always)]
    fn default_options_mut(&mut self) -> &mut binance::BinanceOptions {
        &mut self.binance
    }
}

#[cfg(feature = "bitbank")]
#[cfg_attr(docsrs, doc(cfg(feature = "bitbank")))]
impl GetOptions<bitbank::BitbankOptions> for Client {
    #[inline(always)]
    fn default_options(&self) -> &bitbank::BitbankOptions {
        &self.bitbank
    }

    #[inline(always)]
    fn default_options_mut(&mut self) -> &mut bitbank::BitbankOptions {
        &mut self.bitbank
    }
}

#[cfg(feature = "bitflyer")]
#[cfg_attr(docsrs, doc(cfg(feature = "bitflyer")))]
impl GetOptions<bitflyer::BitFlyerOptions> for Client {
    #[inline(always)]
    fn default_options(&self) -> &bitflyer::BitFlyerOptions {
        &self.bitflyer
    }

    #[inline(always)]
    fn default_options_mut(&mut self) -> &mut bitflyer::BitFlyerOptions {
        &mut self.bitflyer
    }
}

#[cfg(feature = "bybit")]
#[cfg_attr(docsrs, doc(cfg(feature = "bybit")))]
impl GetOptions<bybit::BybitOptions> for Client {
    #[inline(always)]
    fn default_options(&self) -> &bybit::BybitOptions {
        &self.bybit
    }

    #[inline(always)]
    fn default_options_mut(&mut self) -> &mut bybit::BybitOptions {
        &mut self.bybit
    }
}

#[cfg(feature = "coincheck")]
#[cfg_attr(docsrs, doc(cfg(feature = "coincheck")))]
impl GetOptions<coincheck::CoincheckOptions> for Client {
    #[inline(always)]
    fn default_options(&self) -> &coincheck::CoincheckOptions {
        &self.coincheck
    }

    #[inline(always)]
    fn default_options_mut(&mut self) -> &mut coincheck::CoincheckOptions {
        &mut self.coincheck
    }
}
