//! A module for communicating with the [Bybit API](https://bybit-exchange.github.io/docs/spot/v3/#t-introduction).
//! For example usages, see files in the examples/ directory.

use std::{time::SystemTime, borrow::Cow, marker::PhantomData, vec};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::json;
use generic_api_client::{http::{*, header::HeaderValue}, websocket::*};
use crate::traits::*;

/// The type returned by [Client::request()].
pub type BybitRequestResult<T> = Result<T, BybitRequestError>;
pub type BybitRequestError = RequestError<&'static str, BybitHandlerError>;

/// Options that can be set when creating handlers
pub enum BybitOption {
    /// [Default] variant, does nothing
    Default,
    /// API key
    Key(String),
    /// Api secret
    Secret(String),
    /// Base url for HTTP requests
    HttpUrl(BybitHttpUrl),
    /// Type of authentication used for HTTP requests.
    HttpAuth(BybitHttpAuth),
    /// receive window parameter used for requests
    RecvWindow(i32),
    /// [RequestConfig] used when sending requests.
    /// `url_prefix` will be overridden by [HttpUrl](Self::HttpUrl) unless `HttpUrl` is [BybitHttpUrl::None].
    RequestConfig(RequestConfig),
    /// Base url for WebSocket connections
    WebSocketUrl(BybitWebSocketUrl),
    /// Whether [BybitWebSocketHandler] should perform authentication
    WebSocketAuth(bool),
    /// The topics to subscribe to.
    WebSocketTopics(Vec<String>),
    /// [WebSocketConfig] used for creating [WebSocketConnection]s
    /// `url_prefix` will be overridden by [WebSocketUrl](Self::WebSocketUrl) unless `WebSocketUrl` is [BybitWebSocketUrl::None].
    /// By default, `ignore_duplicate_during_reconnection` is set to `true`.
    WebSocketConfig(WebSocketConfig),
}

/// A `struct` that represents a set of [BybitOption] s.
#[derive(Clone, Debug)]
pub struct BybitOptions {
    /// see [BybitOption::Key]
    pub key: Option<String>,
    /// see [BybitOption::Secret]
    pub secret: Option<String>,
    /// see [BybitOption::HttpUrl]
    pub http_url: BybitHttpUrl,
    /// see [BybitOption::HttpAuth]
    pub http_auth: BybitHttpAuth,
    /// see [BybitOption::RecvWindow]
    pub recv_window: Option<i32>,
    /// see [BybitOption::RequestConfig]
    pub request_config: RequestConfig,
    /// see [BybitOption::WebSocketUrl]
    pub websocket_url: BybitWebSocketUrl,
    /// see [BybitOption::WebSocketAuth]
    pub websocket_auth: bool,
    /// see [BybitOption::WebSocketTopics]
    pub websocket_topics: Vec<String>,
    /// see [BybitOption::WebSocketConfig]
    pub websocket_config: WebSocketConfig,
}

/// A `enum` that represents the base url of the Bybit REST API.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum BybitHttpUrl {
    /// `https://api.bybit.com`
    Bybit,
    /// `https://api.bytick.com`
    Bytick,
    /// `https://api-testnet.bybit.com`
    Test,
    /// The url will not be modified by [BybitRequestHandler]
    None,
}

/// A `enum` that represents the base url of the Bybit WebSocket API.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum BybitWebSocketUrl {
    /// `wss://stream.bybit.com`
    Bybit,
    /// `wss://stream.bytick.com`
    Bytick,
    /// `wss://stream-testnet.bybit.com`
    Test,
    /// The url will not be modified by [BybitWebSocketHandler]
    None,
}

/// Represents the auth type.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum BybitHttpAuth {
    /// [Spot V1](https://bybit-exchange.github.io/docs-legacy/spot/v1/#t-introduction)
    SpotV1,
    /// "Previous Version" APIs except for [Spot V1](https://bybit-exchange.github.io/docs-legacy/spot/v1/#t-introduction),
    /// [USDC Option](https://bybit-exchange.github.io/docs-legacy/usdc/option/#t-introduction), and
    /// [USDC Perpetual](https://bybit-exchange.github.io/docs-legacy/usdc/perpetual/#t-introduction)
    BelowV3,
    /// [USDC Option](https://bybit-exchange.github.io/docs-legacy/usdc/option/#t-introduction) and
    /// [USDC Perpetual](https://bybit-exchange.github.io/docs-legacy/usdc/perpetual/#t-introduction)
    UsdcContractV1,
    /// [V3](https://bybit-exchange.github.io/docs/v3/intro) and [V5](https://bybit-exchange.github.io/docs/v5/intro)
    V3AndAbove,
    /// No authentication (for public APIs)
    None,
}

#[derive(Debug)]
pub enum BybitHandlerError {
    ApiError(serde_json::Value),
    IpBan(serde_json::Value),
    ParseError,
}

/// A `struct` that implements [RequestHandler]
pub struct BybitRequestHandler<'a, R: DeserializeOwned> {
    options: BybitOptions,
    _phantom: PhantomData<&'a R>,
}

pub struct BybitWebSocketHandler {
    message_handler: Box<dyn FnMut(serde_json::Value) + Send>,
    options: BybitOptions,
}

impl<'a, B, R> RequestHandler<B> for BybitRequestHandler<'a, R>
where
    B: Serialize,
    R: DeserializeOwned,
{
    type Successful = R;
    type Unsuccessful = BybitHandlerError;
    type BuildError = &'static str;

    fn request_config(&self) -> RequestConfig {
        let mut config = self.options.request_config.clone();
        if self.options.http_url != BybitHttpUrl::None {
            config.url_prefix = self.options.http_url.as_str().to_owned();
        }
        config
    }

    fn build_request(&self, mut builder: RequestBuilder, request_body: &Option<B>, _: u8) -> Result<Request, Self::BuildError> {
        if self.options.http_auth == BybitHttpAuth::None {
            if let Some(body) = request_body {
                let json = serde_json::to_string(body).or(Err("could not serialize body as application/json"))?;
                builder = builder
                    .header(header::CONTENT_TYPE, "application/json")
                    .body(json);
            }
            return builder.build().or(Err("failed to build request"));
        }

        let key = self.options.key.as_deref().ok_or("API key not set")?;
        let secret = self.options.secret.as_deref().ok_or("API secret not set")?;

        let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap(); // always after the epoch
        let timestamp = time.as_millis();

        let hmac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap(); // hmac accepts key of any length

        match self.options.http_auth {
            BybitHttpAuth::SpotV1 => Self::v1_auth(builder, request_body, key, timestamp, hmac, true, self.options.recv_window),
            BybitHttpAuth::BelowV3 => Self::v1_auth(builder, request_body, key, timestamp, hmac, false, self.options.recv_window),
            BybitHttpAuth::UsdcContractV1 => Self::v3_auth(builder, request_body, key, timestamp, hmac, true, self.options.recv_window),
            BybitHttpAuth::V3AndAbove => Self::v3_auth(builder, request_body, key, timestamp, hmac, false, self.options.recv_window),
            BybitHttpAuth::None => unreachable!(), // we've already handled this case
        }
    }

    fn handle_response(&self, status: StatusCode, _: HeaderMap, response_body: Bytes) -> Result<Self::Successful, Self::Unsuccessful> {
        if status.is_success() {
            serde_json::from_slice(&response_body).map_err(|error| {
                log::debug!("Failed to parse response due to an error: {}", error);
                BybitHandlerError::ParseError
            })
        } else {
            // https://bybit-exchange.github.io/docs/spot/v3/#t-ratelimits
            let error = match serde_json::from_slice(&response_body) {
                Ok(parsed) => {
                    if status == 403 {
                        BybitHandlerError::IpBan(parsed)
                    } else {
                        BybitHandlerError::ApiError(parsed)
                    }
                }
                Err(error) => {
                    log::debug!("Failed to parse error response due to an error: {}", error);
                    BybitHandlerError::ParseError
                },
            };
            Err(error)
        }
    }
}

impl<'a, R> BybitRequestHandler<'a, R> where R: DeserializeOwned {
    fn v1_auth<B>(builder: RequestBuilder, request_body: &Option<B>, key: &str, timestamp: u128, mut hmac: Hmac<Sha256>, spot: bool, window: Option<i32>)
        -> Result<Request, <BybitRequestHandler<'a, R> as RequestHandler<B>>::BuildError>
    where
        B: Serialize,
    {
        fn sort_and_add<'a>(mut pairs: Vec<(Cow<str>, Cow<'a, str>)>, key: &'a str, timestamp: u128) -> String {
            pairs.push((Cow::Borrowed("api_key"), Cow::Borrowed(key)));
            pairs.push((Cow::Borrowed("timestamp"), Cow::Owned(timestamp.to_string())));
            pairs.sort_unstable();

            let mut urlencoded = String::new();
            for (key, value) in pairs {
                urlencoded.push_str(&key);
                if !value.is_empty() {
                    urlencoded.push('=');
                    urlencoded.push_str(&value);
                }
                urlencoded.push('&');
            }
            urlencoded.pop(); // the last '&'
            urlencoded
        }

        let mut request = builder.build().or(Err("failed to build request"))?;
        if matches!(*request.method(), Method::GET | Method::DELETE) {
            let mut queries: Vec<_> = request.url().query_pairs().collect();
            if let Some(window) = window {
                if spot {
                    queries.push((Cow::Borrowed("recvWindow"), Cow::Owned(window.to_string())));
                } else {
                    queries.push((Cow::Borrowed("recv_window"), Cow::Owned(window.to_string())));
                }
            }
            let query = sort_and_add(queries, key, timestamp);
            request.url_mut().set_query(Some(&query));

            hmac.update(query.as_bytes());
            let signature = hex::encode(hmac.finalize().into_bytes());

            request.url_mut().query_pairs_mut().append_pair("sign", &signature);

            if let Some(body) = request_body {
                if spot {
                    let body_string = serde_urlencoded::to_string(body).or(Err("could not serialize body as application/x-www-form-urlencoded"))?;
                    *request.body_mut() = Some(body_string.into());
                    request.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
                } else {
                    let body_string = serde_json::to_string(body).or(Err("could not serialize body as application/json"))?;
                    *request.body_mut() = Some(body_string.into());
                    request.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_static("application/json"));
                }
            }
        } else {
            let mut body = if let Some(body) = request_body {
                serde_urlencoded::to_string(body).or(Err("could not serialize body as application/x-www-form-urlencoded"))?
            } else {
                String::new()
            };
            if let Some(window) = window {
                if !body.is_empty() {
                    body.push('&');
                }
                if spot {
                    body.push_str("recvWindow=");
                } else {
                    body.push_str("recv_window=");
                }
                body.push_str(&window.to_string());
            }

            let pairs: Vec<_> = body.split('&')
                .map(|pair| pair.split_once('=').unwrap_or((pair, "")))
                .map(|(k, v)| (Cow::Borrowed(k), Cow::Borrowed(v)))
                .collect();
            let mut sorted_query_string = sort_and_add(pairs, key, timestamp);

            hmac.update(sorted_query_string.as_bytes());
            let signature = hex::encode(hmac.finalize().into_bytes());

            sorted_query_string.push_str(&format!("&sign={signature}"));

            if spot {
                *request.body_mut() = Some(sorted_query_string.into());
                request.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
            } else {
                let body: serde_json::Value = serde_urlencoded::from_str(&sorted_query_string).unwrap(); // sorted_query_string is always in urlencoded format
                *request.body_mut() = Some(body.to_string().into());
                request.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_static("application/json"));
            }
        }
        Ok(request)
    }

    fn v3_auth<B>(mut builder: RequestBuilder, request_body: &Option<B>, key: &str, timestamp: u128, mut hmac: Hmac<Sha256>, version_header: bool, window: Option<i32>)
        -> Result<Request, <BybitRequestHandler<'a, R> as RequestHandler<B>>::BuildError>
    where
        B: Serialize,
    {
        let body = if let Some(body) = request_body {
            let json = serde_json::to_value(body).or(Err("could not serialize body as application/json"))?;
            builder = builder
                .header(header::CONTENT_TYPE, "application/json")
                .body(json.to_string());
            Some(json)
        } else {
            None
        };

        let mut request = builder.build().or(Err("failed to build request"))?;

        let mut sign_contents = format!("{timestamp}{key}");
        if let Some(window) = window {
            sign_contents.push_str(&window.to_string());
        }

        if matches!(*request.method(), Method::GET | Method::DELETE) {
            if let Some(query) = request.url().query() {
                sign_contents.push_str(query);
            }
        } else {
            let body = body.unwrap_or_else(|| {
                *request.body_mut() = Some("{}".into());
                request.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_static("application/json"));
                json!({})
            });
            sign_contents.push_str(&body.to_string());
        }

        hmac.update(sign_contents.as_bytes());
        let signature = hex::encode(hmac.finalize().into_bytes());

        let headers = request.headers_mut();
        if version_header {
            headers.insert("X-BAPI-SIGN-TYPE", HeaderValue::from(2));
        }
        headers.insert("X-BAPI-SIGN", HeaderValue::from_str(&signature).unwrap()); // hex digits are valid
        headers.insert("X-BAPI-API-KEY", HeaderValue::from_str(key).or(Err("invalid character in API key"))?);
        headers.insert("X-BAPI-TIMESTAMP", HeaderValue::from(timestamp as u64));
        if let Some(window) = window {
            headers.insert("X-BAPI-RECV-WINDOW", HeaderValue::from(window));
        }
        Ok(request)
    }
}

impl WebSocketHandler for BybitWebSocketHandler {
    fn websocket_config(&self) -> WebSocketConfig {
        let mut config = self.options.websocket_config.clone();
        if self.options.websocket_url != BybitWebSocketUrl::None {
            config.url_prefix = self.options.websocket_url.as_str().to_owned();
        }
        config
    }

    fn handle_start(&mut self) -> Vec<WebSocketMessage> {
        if self.options.websocket_auth {
            if let Some(key) = self.options.key.as_deref() {
                if let Some(secret) = self.options.secret.as_deref() {
                    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap(); // always after the epoch
                    let expires = time.as_millis() as u64 + 1000;

                    let mut hmac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap(); // hmac accepts key of any length

                    hmac.update(format!("GET/realtime{expires}").as_bytes());
                    let signature = hex::encode(hmac.finalize().into_bytes());

                    return vec![
                        WebSocketMessage::Text(json!({
                            "op": "auth",
                            "args": [key, expires, signature],
                        }).to_string()),
                    ];
                } else {
                    log::debug!("API secret not set.");
                };
            } else {
                log::debug!("API key not set.");
            };
        }
        self.message_subscribe()
    }

    fn handle_message(&mut self, message: WebSocketMessage) -> Vec<WebSocketMessage> {
        match message {
            WebSocketMessage::Text(message) => {
                let message: serde_json::Value = match serde_json::from_str(&message) {
                    Ok(message) => message,
                    Err(_) => {
                        log::debug!("Invalid JSON received");
                        return vec![];
                    },
                };
                match message["op"].as_str() {
                    Some("auth") => {
                        if message["success"].as_bool() == Some(true) {
                            log::debug!("WebSocket authentication successful");
                        } else {
                            log::debug!("WebSocket authentication unsuccessful; message: {}", message["ret_msg"]);
                        }
                        return self.message_subscribe();
                    },
                    Some("subscribe") => {
                        if message["success"].as_bool() == Some(true) {
                            log::debug!("WebSocket topics subscription successful");
                        } else {
                            log::debug!("WebSocket topics subscription unsuccessful; message: {}", message["ret_msg"]);
                        }
                    },
                    _ => (self.message_handler)(message),
                }
            },
            WebSocketMessage::Binary(_) => log::debug!("Unexpected binary message received"),
            WebSocketMessage::Ping(_) | WebSocketMessage::Pong(_) => (),
        }
        vec![]
    }
}

impl BybitWebSocketHandler {
    #[inline(always)]
    fn message_subscribe(&self) -> Vec<WebSocketMessage> {
        vec![WebSocketMessage::Text(
            json!({ "op": "subscribe", "args": self.options.websocket_topics }).to_string(),
        )]
    }
}

impl BybitHttpUrl {
    /// The URL that this variant represents.
    #[inline(always)]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Bybit => "https://api.bybit.com",
            Self::Bytick => "https://api.bytick.com",
            Self::Test => "https://api-testnet.bybit.com",
            Self::None => "",
        }
    }
}

impl BybitWebSocketUrl {
    /// The URL that this variant represents.
    #[inline(always)]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Bybit => "wss://stream.bybit.com",
            Self::Bytick => "wss://stream.bytick.com",
            Self::Test => "wss://stream-testnet.bybit.com",
            Self::None => "",
        }
    }
}

impl HandlerOptions for BybitOptions {
    type OptionItem = BybitOption;

    fn update(&mut self, option: Self::OptionItem) {
        match option {
            BybitOption::Default => (),
            BybitOption::Key(v) => self.key = Some(v),
            BybitOption::Secret(v) => self.secret = Some(v),
            BybitOption::HttpUrl(v) => self.http_url = v,
            BybitOption::HttpAuth(v) => self.http_auth = v,
            BybitOption::RecvWindow(v) => self.recv_window = Some(v),
            BybitOption::RequestConfig(v) => self.request_config = v,
            BybitOption::WebSocketUrl(v) => self.websocket_url = v,
            BybitOption::WebSocketAuth(v) => self.websocket_auth = v,
            BybitOption::WebSocketTopics(v) => self.websocket_topics = v,
            BybitOption::WebSocketConfig(v) => self.websocket_config = v,
        }
    }
}

impl Default for BybitOptions {
    fn default() -> Self {
        let mut websocket_config = WebSocketConfig::new();
        websocket_config.ignore_duplicate_during_reconnection = true;
        Self {
            key: None,
            secret: None,
            http_url: BybitHttpUrl::Bybit,
            http_auth: BybitHttpAuth::None,
            recv_window: None,
            request_config: RequestConfig::default(),
            websocket_url: BybitWebSocketUrl::Bybit,
            websocket_auth: false,
            websocket_topics: vec![],
            websocket_config,
        }
    }
}

impl<'a, R, B> HttpOption<'a, R, B> for BybitOption
where
    R: DeserializeOwned + 'a,
    B: Serialize,
{
    type RequestHandler = BybitRequestHandler<'a, R>;

    #[inline(always)]
    fn request_handler(options: Self::Options) -> Self::RequestHandler {
        BybitRequestHandler::<'a, R> {
            options,
            _phantom: PhantomData,
        }
    }
}

impl <H: FnMut(serde_json::Value) + Send + 'static> WebSocketOption<H> for BybitOption {
    type WebSocketHandler = BybitWebSocketHandler;

    #[inline(always)]
    fn websocket_handler(handler: H, options: Self::Options) -> Self::WebSocketHandler {
        BybitWebSocketHandler {
            message_handler: Box::new(handler),
            options,
        }
    }
}

impl HandlerOption for BybitOption {
    type Options = BybitOptions;
}

impl Default for BybitOption {
    fn default() -> Self {
        Self::Default
    }
}
