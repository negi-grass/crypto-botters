//! A crate for communicating with the [bitFlyer API](https://lightning.bitflyer.com/docs).
//! For example usages, see files in the examples/ directory.

use std::{
    marker::PhantomData,
    time::{SystemTime, Duration},
};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use rand::{Rng, distributions::Alphanumeric};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::json;
use generic_api_client::{http::*, websocket::*};
use generic_api_client::http::header::HeaderValue;

/// The type returned by [Client::request()].
pub type BitFlyerRequestResult<T> = Result<T, RequestError<&'static str, BitFlyerHandlerError>>;

/// A `struct` that provides the [generic_api_client]'s handlers.
#[derive(Clone)]
pub struct BitFlyer {
    api_key: Option<String>,
    api_secret: Option<String>,
    /// How many times should the request be sent if it keeps failing. Defaults to 1.
    /// See also: field `max_try` of [RequestConfig]
    pub request_max_try: u8,
    /// Whether the websocket handler should receive duplicate message. Defaults to false.
    /// See also: field `ignore_duplicate_during_reconnection` of [WebSocketConfig].
    pub websocket_allow_duplicate_message: bool,
    /// The interval of auto reconnection. Defaults to disabled.
    /// See also: field `refresh_after` of [WebSocketConfig]
    pub websocket_refresh_interval: Duration,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum BitflyerSecurity {
    None,
    Sign,
}

#[derive(Deserialize, Debug)]
pub struct BitFlyerChannelMessage {
    pub channel: String,
    pub message: serde_json::Value,
}

#[derive(Debug)]
pub enum BitFlyerHandlerError {
    ApiError(serde_json::Value),
    ParseError,
}

#[derive(Copy, Clone)]
pub struct BitFlyerRequestHandler<'a, R: DeserializeOwned> {
    api_key: Option<&'a str>,
    api_secret: Option<&'a str>,
    security: BitflyerSecurity,
    max_try: u8,
    _phantom: PhantomData<&'a R>,
}

pub struct BitFlyerWebSocketHandler<H: FnMut(BitFlyerChannelMessage) + Send + 'static> {
    api_key: Option<String>,
    api_secret: Option<String>,
    message_handler: H,
    channels: Vec<String>,
    auth: bool,
    auth_id: Option<String>,
    allow_duplicate: bool,
    refresh: Duration,
}

impl BitFlyer {
    pub fn new(api_key: Option<String>, api_secret: Option<String>) -> Self {
        Self {
            api_key,
            api_secret,
            request_max_try: 1,
            websocket_allow_duplicate_message: false,
            websocket_refresh_interval: Duration::ZERO, // disable
        }
    }

    /// Returns a `impl` [RequestHandler] to be passed to [Client::request()].
    pub fn request<R: DeserializeOwned>(&self, security: BitflyerSecurity) -> BitFlyerRequestHandler<R> {
        BitFlyerRequestHandler {
            api_key: self.api_key.as_deref(),
            api_secret: self.api_secret.as_deref(),
            security,
            max_try: self.request_max_try,
            _phantom: PhantomData,
        }
    }

    /// Returns a `impl` [WebSocketHandler] to be passed to [WebSocketConnection::new()].
    pub fn websocket<H>(&self, message_handler: H, channels: Vec<&str>, auth: bool) -> BitFlyerWebSocketHandler<H>
    where
        H: FnMut(BitFlyerChannelMessage) + Send + 'static,
    {
        let channels = channels.into_iter().map(ToOwned::to_owned).collect();
        BitFlyerWebSocketHandler {
            api_key: self.api_key.clone(),
            api_secret: self.api_secret.clone(),
            message_handler,
            channels,
            auth,
            auth_id: None,
            allow_duplicate: self.websocket_allow_duplicate_message,
            refresh: self.websocket_refresh_interval,
        }
    }
}

// https://binance-docs.github.io/apidocs/spot/en/#general-api-information
impl<'a, B, R> RequestHandler<B> for BitFlyerRequestHandler<'a, R>
where
    B: Serialize,
    R: DeserializeOwned,
{
    type Successful = R;
    type Unsuccessful = BitFlyerHandlerError;
    type BuildError = &'static str;

    fn request_config(&self) -> RequestConfig {
        let mut config = RequestConfig::new();
        config.url_prefix = "https://api.bitflyer.com".to_owned();
        config.max_try = self.max_try;
        config
    }

    fn build_request(&self, mut builder: RequestBuilder, request_body: &Option<B>, _: u8) -> Result<Request, Self::BuildError> {
        if let Some(body) = request_body {
            let json = serde_json::to_vec(body).or(Err("could not serialize body as JSON"))?;
            builder = builder
                .header(header::CONTENT_TYPE, "application/json")
                .body(json);
        }

        let mut request = builder.build().or(Err("failed to build request"))?;

        if self.security == BitflyerSecurity::Sign {
            // https://lightning.bitflyer.com/docs?lang=en#authentication
            let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap(); // always after the epoch
            let timestamp = time.as_millis() as u64;

            let mut path = request.url().path().to_owned();
            if let Some(query) = request.url().query() {
                path.push('?');
                path.push_str(query)
            }
            let body = request.body()
                .and_then(|body| body.as_bytes())
                .map(String::from_utf8_lossy)
                .unwrap_or_default();

            let sign_contents = format!("{}{}{}{}", timestamp, request.method(), path, body);

            let secret = self.api_secret.ok_or("API secret not set")?;
            let mut hmac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap(); // hmac accepts key of any length

            hmac.update(sign_contents.as_bytes());
            let signature = hex::encode(hmac.finalize().into_bytes());

            let key = HeaderValue::from_str(self.api_key.ok_or("API key not set")?).or(
                Err("invalid character in API key")
            )?;
            let headers = request.headers_mut();
            headers.insert("ACCESS-KEY", key);
            headers.insert("ACCESS-TIMESTAMP", HeaderValue::from(timestamp));
            headers.insert("ACCESS-SIGN", HeaderValue::from_str(&signature).unwrap()); // hex digits are valid
            headers.insert(header::CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap()); // only contains valid letters
        }

        Ok(request)
    }

    fn handle_response(&self, status: StatusCode, _: HeaderMap, response_body: Bytes) -> Result<Self::Successful, Self::Unsuccessful> {
        if status.is_success() {
            serde_json::from_slice(&response_body).map_err(|error| {
                log::error!("Failed to parse response due to an error: {}", error);
                BitFlyerHandlerError::ParseError
            })
        } else {
            let error = match serde_json::from_slice(&response_body) {
                Ok(parsed_error) => BitFlyerHandlerError::ApiError(parsed_error),
                Err(error) => {
                    dbg!(response_body);
                    log::error!("Failed to parse error response due to an error: {}", error);
                    BitFlyerHandlerError::ParseError
                }
            };
            Err(error)
        }
    }
}

impl<H> WebSocketHandler for BitFlyerWebSocketHandler<H> where H: FnMut(BitFlyerChannelMessage) + Send + 'static, {
    fn websocket_config(&self) -> WebSocketConfig {
        let mut config = WebSocketConfig::new();
        config.url_prefix = "wss://ws.lightstream.bitflyer.com".to_owned();
        config.ignore_duplicate_during_reconnection = !self.allow_duplicate;
        config.refresh_after = self.refresh;
        config
    }

    fn handle_start(&mut self) -> Vec<WebSocketMessage> {
        if self.auth {
            // https://bf-lightning-api.readme.io/docs/realtime-api-auth
            if let Some(key) = &self.api_key {
                if let Some(secret) = &self.api_secret {
                    let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap(); // always after the epoch
                    let timestamp = time.as_millis() as u64;
                    let nonce: String = rand::thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(16)
                        .map(char::from)
                        .collect();

                    let mut hmac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap(); // hmac accepts key of any length

                    hmac.update(format!("{}{}", timestamp, nonce).as_bytes());
                    let signature = hex::encode(hmac.finalize().into_bytes());

                    let id = format!("_auth{}", time.as_nanos());
                    self.auth_id = Some(id.clone());

                    return vec![WebSocketMessage::Text(json!({
                        "method": "auth",
                        "params": {
                            "api_key": key,
                            "timestamp": timestamp,
                            "nonce": nonce,
                            "signature": signature,
                        },
                        "id": id,
                    }).to_string())];
                } else {
                    log::error!("API secret not set.");
                };
            } else {
                log::error!("API key not set.");
            };
        }
        self.message_subscribe()
    }

    fn handle_message(&mut self, message: WebSocketMessage) -> Vec<WebSocketMessage> {
        #[derive(Deserialize)]
        struct Message {
            #[allow(dead_code)]
            jsonrpc: String, // 2.0
            method: Option<String>,
            result: Option<serde_json::Value>,
            params: Option<BitFlyerChannelMessage>,
            id: Option<String>,
        }

        match message {
            WebSocketMessage::Text(message) => {
                let message: Message = match serde_json::from_str(&message) {
                    Ok(message) => message,
                    Err(_) => {
                        log::warn!("Invalid JSON-RPC message received");
                        return Vec::new();
                    },
                };
                if self.auth && self.auth_id == message.id {
                    // result of auth
                    if message.result == Some(serde_json::Value::Bool(true)) {
                        log::debug!("WebSocket authentication successful");
                        return self.message_subscribe();
                    } else {
                        log::error!("WebSocket authentication unsuccessful");
                    }
                    self.auth_id = None;
                } else if message.method.as_deref() == Some("channelMessage") {
                    if let Some(channel_message) = message.params {
                        (self.message_handler)(channel_message);
                    }
                }
            },
            WebSocketMessage::Binary(_) => log::warn!("Unexpected binary message received"),
            WebSocketMessage::Ping(_) | WebSocketMessage::Pong(_) => (),
        }
        Vec::new()
    }
}

impl<H> BitFlyerWebSocketHandler<H> where H: FnMut(BitFlyerChannelMessage) + Send + 'static, {
    fn message_subscribe(&self) -> Vec<WebSocketMessage> {
        self.channels.clone().into_iter().map(|channel| {
            WebSocketMessage::Text(json!({ "method": "subscribe", "params": { "channel": channel } }).to_string())
        }).collect()
    }
}
