//! A crate for communicating with the [Binance API](https://binance-docs.github.io/apidocs/spot/en/).
//! For example usages, see files in the examples/ directory.

use std::{
    str::FromStr,
    marker::PhantomData,
    time::{SystemTime, Duration},
};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use generic_api_client::{http::*, websocket::*};

/// The type returned by [Client::request()].
pub type BinanceRequestResult<T> = Result<T, RequestError<&'static str, BinanceHandlerError>>;

/// A `struct` that provides the [generic_api_client]'s handlers.
#[derive(Clone)]
pub struct Binance {
    api_key: Option<String>,
    api_secret: Option<String>,
    /// How many times should the request be sent if it keeps failing. Defaults to 1.
    /// See also: field `max_try` of [RequestConfig]
    pub request_max_try: u8,
    /// Whether the websocket handler should receive duplicate message. Defaults to disabled.
    /// See also: field `ignore_duplicate_during_reconnection` of [WebSocketConfig].
    pub websocket_allow_duplicate_message: bool,
    /// The interval of auto reconnection. Defaults to 12 hours.
    /// See also: field `refresh_after` of [WebSocketConfig]
    pub websocket_refresh_interval: Duration,
}

/// A `enum` that represents the base url of the Binance REST API.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[non_exhaustive]
pub enum BinanceHttpUrl {
    /// https://api.binance.com
    Spot,
    /// https://api1.binance.com
    Spot1,
    /// https://api2.binance.com
    Spot2,
    /// https://api3.binance.com
    Spot3,
    /// https://testnet.binance.vision
    SpotTest,
    /// https://data.binance.com
    SpotData,
    /// https://fapi.binance.com
    FuturesUsdM,
    /// https://dapi.binance.com
    FuturesCoinM,
    /// https://testnet.binancefuture.com
    FuturesTest,
    /// https://eapi.binance.com
    EuropeanOptions,
    /// The url will not be modified by [BinanceRequestHandler]
    None,
}

/// A `enum` that represents the base url of the Binance WebSocket API
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[non_exhaustive]
pub enum BinanceWebSocketUrl {
    /// wss://stream.binance.com:9443
    Spot9443,
    /// wss://stream.binance.com:443
    Spot443,
    /// wss://testnet.binance.vision
    SpotTest,
    /// wss://data-stream.binance.com
    SpotData,
    /// wss://ws-api.binance.com:443
    WebSocket443,
    /// wss://ws-api.binance.com:9443
    WebSocket9443,
    /// wss://fstream.binance.com
    FuturesUsdM,
    /// wss://fstream-auth.binance.com
    FuturesUsdMAuth,
    /// wss://dstream.binance.com
    FuturesCoinM,
    /// wss://stream.binancefuture.com
    FuturesUsdMTest,
    /// wss://dstream.binancefuture.com
    FuturesCoinMTest,
    /// wss://nbstream.binance.com
    EuropeanOptions,
    /// The url will not be modified by [BinanceRequestHandler]
    None,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum BinanceSecurity {
    None,
    Key,
    Sign,
}

#[derive(Debug)]
pub enum BinanceHandlerError {
    ApiError(BinanceError),
    RateLimitError { retry_after: Option<u32> },
    ParseError,
}

#[derive(Copy, Clone)]
pub struct BinanceRequestHandler<'a, R: DeserializeOwned> {
    api_key: Option<&'a str>,
    api_secret: Option<&'a str>,
    security: BinanceSecurity,
    base_url: BinanceHttpUrl,
    max_try: u8,
    _phantom: PhantomData<&'a R>,
}

pub struct BinanceWebSocketHandler<H: FnMut(serde_json::Value) + Send + 'static> {
    message_handler: H,
    base_url: BinanceWebSocketUrl,
    allow_duplicate: bool,
    refresh: Duration,
}

#[derive(Deserialize, Debug)]
pub struct BinanceError {
    pub code: i32,
    pub msg: String,
}

impl Binance {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        Self {
            api_key,
            api_secret,
            request_max_try: 1,
            websocket_allow_duplicate_message: false,
            websocket_refresh_interval: Duration::from_secs(60 * 60 * 12), // 12 hours
        }
    }

    /// Returns a `impl` [RequestHandler] to be passed to [Client::request()].
    pub fn request<R: DeserializeOwned>(&self, security: BinanceSecurity, base_url: BinanceHttpUrl) -> BinanceRequestHandler<R> {
        BinanceRequestHandler {
            api_key: self.api_key.as_deref(),
            api_secret: self.api_secret.as_deref(),
            security,
            base_url,
            max_try: self.request_max_try,
            _phantom: PhantomData,
        }
    }

    /// Returns a `impl` [RequestHandler] to be passed to [Client::request()].
    ///
    /// The difference between [request()][Self::request()] is that the `base_url` parameter is not needed.
    #[inline(always)]
    pub fn request_no_url<R: DeserializeOwned>(&self, security: BinanceSecurity) -> BinanceRequestHandler<R> {
        self.request(security, BinanceHttpUrl::None)
    }

    /// Returns a `impl` [WebSocketHandler] to be passed to [WebSocketConnection::new()].
    pub fn websocket<H>(&self, message_handler: H, base_url: BinanceWebSocketUrl) -> BinanceWebSocketHandler<H>
    where
        H: FnMut(serde_json::Value) + Send + 'static,
    {
        BinanceWebSocketHandler {
            message_handler,
            base_url,
            allow_duplicate: self.websocket_allow_duplicate_message,
            refresh: self.websocket_refresh_interval,
        }
    }

    /// Returns a `impl` [WebSocketHandler] to be passed to [WebSocketConnection::new()].
    ///
    /// The difference between [websocket()][Self::websocket()] is that the `base_url` parameter is not needed.
    #[inline(always)]
    pub fn websocket_no_url<H>(&self, message_handler: H) -> BinanceWebSocketHandler<H>
    where
        H: FnMut(serde_json::Value) + Send + 'static,
    {
        self.websocket(message_handler, BinanceWebSocketUrl::None)
    }
}

// https://binance-docs.github.io/apidocs/spot/en/#general-api-information
impl<'a, B, R> RequestHandler<B> for BinanceRequestHandler<'a, R>
where
    B: Serialize,
    R: DeserializeOwned,
{
    type Successful = R;
    type Unsuccessful = BinanceHandlerError;
    type BuildError = &'static str;

    fn request_config(&self) -> RequestConfig {
        let mut config = RequestConfig::new();
        config.url_prefix = self.base_url.to_string();
        config.max_try = self.max_try;
        config
    }

    fn build_request(&self, mut builder: RequestBuilder, request_body: &Option<B>, _: u8) -> Result<Request, Self::BuildError> {
        if let Some(body) = request_body {
            let encoded = serde_urlencoded::to_string(body).or(
                Err("could not parse body as application/x-www-form-urlencoded"),
            )?;
            builder = builder
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(encoded);
        }

        if self.security != BinanceSecurity::None {
            // https://binance-docs.github.io/apidocs/spot/en/#signed-trade-user_data-and-margin-endpoint-security
            let key = self.api_key.ok_or("API key not set")?;
            builder = builder.header("X-MBX-APIKEY", key);

            if self.security == BinanceSecurity::Sign {
                let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap(); // always after the epoch
                let timestamp = time.as_millis();

                builder = builder.query(&[("timestamp", timestamp)]);

                let secret = self.api_secret.ok_or("API secret not set")?;
                let mut hmac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap(); // hmac accepts key of any length

                let mut request = builder.build().or(Err("Failed to build request"))?;
                let query = request.url().query().unwrap(); // we added the timestamp query
                let body = request.body().and_then(|body| body.as_bytes()).unwrap_or_default();

                hmac.update(&[query.as_bytes(), body].concat());
                let signature = hex::encode(hmac.finalize().into_bytes());

                request.url_mut().query_pairs_mut().append_pair("signature", &signature);

                return Ok(request);
            }
        }
        builder.build().or(Err("failed to build request"))
    }

    fn handle_response(&self, status: StatusCode, headers: HeaderMap, response_body: Bytes) -> Result<Self::Successful, Self::Unsuccessful> {
        if status.is_success() {
            serde_json::from_slice(&response_body).map_err(|error| {
                log::error!("Failed to parse response due to an error: {}", error);
                BinanceHandlerError::ParseError
            })
        } else {
            // https://binance-docs.github.io/apidocs/spot/en/#limits
            if status == 429 || status == 418 {
                let retry_after = if let Some(value) = headers.get("Retry-After") {
                    if let Ok(string) = value.to_str() {
                        if let Ok(retry_after) = u32::from_str(string) {
                            Some(retry_after)
                        } else {
                            log::warn!("Invalid number in Retry-After header");
                            None
                        }
                    } else {
                        log::warn!("Non-ASCII character in Retry-After header");
                        None
                    }
                } else {
                    None
                };
                return Err(BinanceHandlerError::RateLimitError { retry_after });
            }

            let error = match serde_json::from_slice(&response_body) {
                Ok(parsed_error) => BinanceHandlerError::ApiError(parsed_error),
                Err(error) => {
                    log::error!("Failed to parse error response due to an error: {}", error);
                    BinanceHandlerError::ParseError
                }
            };
            Err(error)
        }
    }
}

impl<H> WebSocketHandler for BinanceWebSocketHandler<H> where H: FnMut(serde_json::Value) + Send + 'static, {
    fn websocket_config(&self) -> WebSocketConfig {
        let mut config = WebSocketConfig::new();
        config.url_prefix = self.base_url.to_string();
        config.ignore_duplicate_during_reconnection = !self.allow_duplicate;
        config.refresh_after = self.refresh;
        config
    }

    fn handle_message(&mut self, message: WebSocketMessage) -> Vec<WebSocketMessage> {
        match message {
            WebSocketMessage::Text(message) => {
                if let Ok(message) = serde_json::from_str(&message) {
                    (self.message_handler)(message);
                } else {
                    log::error!("Invalid JSON message received");
                }
            },
            WebSocketMessage::Binary(_) => log::warn!("Unexpected binary message received"),
            WebSocketMessage::Ping(_) | WebSocketMessage::Pong(_) => (),
        }
        Vec::new()
    }
}

impl BinanceHttpUrl {
    /// The string that this variant represents.
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Spot => "https://api.binance.com",
            Self::Spot1 => "https://api1.binance.com",
            Self::Spot2 => "https://api2.binance.com",
            Self::Spot3 => "https://api3.binance.com",
            Self::SpotTest => "https://testnet.binance.vision",
            Self::SpotData => "https://data.binance.com",
            Self::FuturesUsdM => "https://fapi.binance.com",
            Self::FuturesCoinM => "https://dapi.binance.com",
            Self::FuturesTest => "https://testnet.binancefuture.com",
            Self::EuropeanOptions => "https://eapi.binance.com",
            Self::None => "",
        }
    }
}

impl BinanceWebSocketUrl {
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Spot9443 => "wss://stream.binance.com:9443",
            Self::Spot443 => "wss://stream.binance.com:443",
            Self::SpotTest => "wss://testnet.binance.vision",
            Self::SpotData => "wss://data-stream.binance.com",
            Self::WebSocket443 => "wss://ws-api.binance.com:443",
            Self::WebSocket9443 => "wss://ws-api.binance.com:9443",
            Self::FuturesUsdM => "wss://fstream.binance.com",
            Self::FuturesUsdMAuth => "wss://fstream-auth.binance.com",
            Self::FuturesCoinM => "wss://dstream.binance.com",
            Self::FuturesUsdMTest => "wss://stream.binancefuture.com",
            Self::FuturesCoinMTest => "wss://dstream.binancefuture.com",
            Self::EuropeanOptions => "wss://nbstream.binance.com",
            Self::None => "",
        }
    }
}

impl ToString for BinanceHttpUrl {
    fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}

impl ToString for BinanceWebSocketUrl {
    fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}
