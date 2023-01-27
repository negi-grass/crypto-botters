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
use crypto_botters_api::*;
use generic_api_client::{http::*, websocket::*};

/// The type returned by [Client::request()].
pub type BinanceRequestResult<T> = Result<T, RequestError<&'static str, BinanceHandlerError>>;

/// Options that can be set when creating handlers
pub enum BinanceOption {
    /// [Default] variant, does nothing
    Default,
    /// API key
    Key(String),
    /// Api secret
    Secret(String),
    /// Base url for HTTP requests
    HttpUrl(BinanceHttpUrl),
    /// Authentication type for HTTP requests
    HttpAuth(BinanceAuth),
    /// [RequestConfig] used when sending requests.
    /// `url_prefix` will be overridden by [HttpUrl](Self::HttpUrl) unless `HttpUrl` is [BinanceHttpUrl::None].
    RequestConfig(RequestConfig),
    /// Base url for WebSocket connections
    WebSocketUrl(BinanceWebSocketUrl),
    /// [WebSocketConfig] used for creating [WebSocketConnection]s
    /// `url_prefix` will be overridden by [WebSocketUrl](Self::WebSocketUrl) unless `WebSocketUrl` is [BinanceWebSocketUrl::None].
    /// By default, `refresh_after` is set to 12 hours and `ignore_duplicate_during_reconnection` is set to `true`.
    WebSocketConfig(WebSocketConfig),
}

/// A `struct` that represents a set of [BinanceOption] s.
#[derive(Clone, Debug)]
pub struct BinanceOptions {
    /// see [BinanceOption::Key]
    pub key: Option<String>,
    /// see [BinanceOption::Secret]
    pub secret: Option<String>,
    /// see [BinanceOption::HttpUrl]
    pub http_url: BinanceHttpUrl,
    /// see [BinanceOption::HttpAuth]
    pub http_auth: BinanceAuth,
    /// see [BinanceOption::RequestConfig]
    pub request_config: RequestConfig,
    /// see [BinanceOption::WebSocketUrl]
    pub websocket_url: BinanceWebSocketUrl,
    /// see [BinanceOption::WebSocketConfig]
    pub websocket_config: WebSocketConfig,
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
    /// https://api4.binance.com
    Spot4,
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
pub enum BinanceAuth {
    Sign,
    Key,
    None,
}

#[derive(Debug)]
pub enum BinanceHandlerError {
    ApiError(BinanceError),
    RateLimitError { retry_after: Option<u32> },
    ParseError,
}

#[derive(Deserialize, Debug)]
pub struct BinanceError {
    pub code: i32,
    pub msg: String,
}

/// A `struct` that implements [RequestHandler]
pub struct BinanceRequestHandler<'a, R: DeserializeOwned> {
    options: BinanceOptions,
    _phantom: PhantomData<&'a R>,
}

/// A `struct` that implements [WebSocketHandler]
pub struct BinanceWebSocketHandler<H: FnMut(serde_json::Value) + Send + 'static> {
    message_handler: H,
    options: BinanceOptions,
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
        let mut config = self.options.request_config.clone();
        if self.options.http_url != BinanceHttpUrl::None {
            config.url_prefix = self.options.http_url.as_str().to_owned();
        }
        config
    }

    fn build_request(&self, mut builder: RequestBuilder, request_body: &Option<B>, _: u8) -> Result<Request, Self::BuildError> {
        if let Some(body) = request_body {
            let encoded = serde_urlencoded::to_string(body).or(
                Err("could not serialize body as application/x-www-form-urlencoded"),
            )?;
            builder = builder
                .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
                .body(encoded);
        }

        if self.options.http_auth != BinanceAuth::None {
            // https://binance-docs.github.io/apidocs/spot/en/#signed-trade-user_data-and-margin-endpoint-security
            let key = self.options.key.as_deref().ok_or("API key not set")?;
            builder = builder.header("X-MBX-APIKEY", key);

            if self.options.http_auth == BinanceAuth::Sign {
                let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap(); // always after the epoch
                let timestamp = time.as_millis();

                builder = builder.query(&[("timestamp", timestamp)]);

                let secret = self.options.secret.as_deref().ok_or("API secret not set")?;
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
        let mut config = self.options.websocket_config.clone();
        if self.options.websocket_url != BinanceWebSocketUrl::None {
            config.url_prefix = self.options.websocket_url.as_str().to_owned();
        }
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
        vec![]
    }
}

impl BinanceHttpUrl {
    /// The URL that this variant represents.
    #[inline(always)]
    fn as_str(&self) -> &'static str {
        match self {
            Self::Spot => "https://api.binance.com",
            Self::Spot1 => "https://api1.binance.com",
            Self::Spot2 => "https://api2.binance.com",
            Self::Spot3 => "https://api3.binance.com",
            Self::Spot4 => "https://api4.binance.com",
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
    /// The URL that this variant represents.
    #[inline(always)]
    pub fn as_str(&self) -> &'static str {
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

impl HandlerOptions for BinanceOptions {
    type OptionItem = BinanceOption;

    fn update(&mut self, option: Self::OptionItem) {
        match option {
            BinanceOption::Default => (),
            BinanceOption::Key(v) => self.key = Some(v),
            BinanceOption::Secret(v) => self.secret = Some(v),
            BinanceOption::HttpUrl(v) => self.http_url = v,
            BinanceOption::HttpAuth(v) => self.http_auth = v,
            BinanceOption::RequestConfig(v) => self.request_config = v,
            BinanceOption::WebSocketUrl(v) => self.websocket_url = v,
            BinanceOption::WebSocketConfig(v) => self.websocket_config = v,
        }
    }
}

impl Default for BinanceOptions {
    fn default() -> Self {
        let mut websocket_config = WebSocketConfig::new();
        websocket_config.refresh_after = Duration::from_secs(60 * 60 * 12);
        websocket_config.ignore_duplicate_during_reconnection = true;
        Self {
            key: None,
            secret: None,
            http_url: BinanceHttpUrl::None,
            http_auth: BinanceAuth::None,
            request_config: RequestConfig::default(),
            websocket_url: BinanceWebSocketUrl::None,
            websocket_config,
        }
    }
}

impl<'a, R: DeserializeOwned + 'a> HttpOption<'a, R> for BinanceOption {
    type RequestHandler = BinanceRequestHandler<'a, R>;

    #[inline(always)]
    fn request_handler(options: Self::Options) -> Self::RequestHandler {
        BinanceRequestHandler::<'a, R> {
            options,
            _phantom: PhantomData,
        }
    }
}

impl<H: FnMut(serde_json::Value) + Send + 'static> WebSocketOption<H> for BinanceOption {
    type WebSocketHandler = BinanceWebSocketHandler<H>;

    #[inline(always)]
    fn websocket_handler(handler: H, options: Self::Options) -> Self::WebSocketHandler {
        BinanceWebSocketHandler {
            message_handler: handler,
            options,
        }
    }
}

impl HandlerOption for BinanceOption {
    type Options = BinanceOptions;
}

impl Default for BinanceOption {
    fn default() -> Self {
        Self::Default
    }
}
