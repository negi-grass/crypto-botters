//! A crate for communicating with the [Bybit API](https://bybit-exchange.github.io/docs/spot/v3/#t-introduction).
//! For example usages, see files in the examples/ directory.

use std::{
    time::SystemTime,
    borrow::Cow,
    marker::PhantomData,
};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use serde::{
    Serialize,
    de::DeserializeOwned,
};
use crypto_botters_api::{HandlerOption, HandlerOptions, HttpOption};
use generic_api_client::{http::{*, header::HeaderValue}, websocket::*};

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
    /// [RequestConfig] used when sending requests.
    /// `url_prefix` will be overridden by [HttpUrl](Self::HttpUrl) unless `HttpUrl` is [BinanceHttpUrl::None].
    RequestConfig(RequestConfig),
    /// Base url for WebSocket connections
    WebSocketUrl(BybitWebSocketUrl),
    /// Whether [BitFlyerWebSocketHandler] should perform authentication
    WebSocketAuth(bool),
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
    /// see [BybitOption::RequestConfig]
    pub request_config: RequestConfig,
    /// see [BybitOption::WebSocketUrl]
    pub websocket_url: BybitWebSocketUrl,
    /// see [BybitOption::WebSocketAuth]
    pub websocket_auth: bool,
    /// see [BybitOption::WebSocketConfig]
    pub websocket_config: WebSocketConfig,
}

/// A `enum` that represents the base url of the Bybit REST API.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum BybitHttpUrl {
    /// https://api.bybit.com
    Bybit,
    /// https://api.bytick.com
    Bytick,
    /// https://api-testnet.bybit.com
    Test,
    /// The url will not be modified by [BybitRequestHandler]
    None,
}

/// A `enum` that represents the base url of the Bybit WebSocket API.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum BybitWebSocketUrl {
    /// wss://stream.bybit.com
    Bybit,
    /// wss://stream.bytick.com
    Bytick,
    /// wss://stream-testnet.bybit.com
    Test,
    /// The url will not be modified by [BybitWebSocketHandler]
    None,
}

/// Represents the auth type.
///
/// |API|type|
/// |---|----|
/// |Derivatives v3 Unified Margin|Type2|
/// |Derivatives v3 Contract|Type2|
/// |Futures v2 Inverse Perpetual|Type1|
/// |Futures v2 USDT Perpetual|Type1|
/// |Futures v2 Inverse Futures|Type1|
/// |Spot v3|SpotType2|
/// |Spot v1|SpotType1|
/// |Account Asset v3|Type2|
/// |Account Asset v1|Type1|
/// |Copy Trading|Type2|
/// |USDC Contract Option|SpotType2|
/// |USDC Contract Perpetual|SpotType2|
/// |Tax|Type2|
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum BybitHttpAuth {
    Type1,
    SpotType1,
    Type2,
    SpotType2,
    None,
}

/// A `struct` that implements [RequestHandler]
pub struct BybitRequestHandler<'a, R: DeserializeOwned> {
    options: BybitOptions,
    _phantom: PhantomData<&'a R>,
}

impl<'a, B, R> RequestHandler<B> for BybitRequestHandler<'a, R>
where
    B: Serialize,
    R: DeserializeOwned,
{
    type Successful = R;
    type Unsuccessful = ();
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
            BybitHttpAuth::Type1 => Self::type1_auth(builder, request_body, key, timestamp, hmac, false),
            BybitHttpAuth::SpotType1 => Self::type1_auth(builder, request_body, key, timestamp, hmac, true),
            BybitHttpAuth::Type2 => Self::type2_auth(builder, request_body, key, timestamp, hmac, false),
            BybitHttpAuth::SpotType2 => Self::type2_auth(builder, request_body, key, timestamp, hmac, true),
            BybitHttpAuth::None => unreachable!(), // we've already handled this case
        }
    }

    fn handle_response(&self, status: StatusCode, headers: HeaderMap, response_body: Bytes) -> Result<Self::Successful, Self::Unsuccessful> {
        todo!()
    }
}

impl<'a, R> BybitRequestHandler<'a, R> where R: DeserializeOwned {
    fn type1_auth<B>(builder: RequestBuilder, request_body: &Option<B>, key: &str, timestamp: u128, mut hmac: Hmac<Sha256>, spot: bool)
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
            let queries: Vec<_> = request.url().query_pairs().collect();
            let query = sort_and_add(queries, key, timestamp);
            request.url_mut().set_query(Some(&query));

            hmac.update(query.as_bytes());
            let signature = hex::encode(hmac.finalize().into_bytes());

            request.url_mut().query_pairs_mut().append_pair("sign", &signature);

            if let Some(body) = request_body {
                let (body_string, content_type) = if spot {
                    (
                        serde_urlencoded::to_string(body).or(Err("could not serialize body as application/x-www-form-urlencoded"))?,
                        "application/x-www-form-urlencoded",
                    )
                } else {
                    (
                        serde_json::to_string(body).or(Err("could not serialize body as application/json"))?,
                        "application/json",
                    )
                };
                *request.body_mut() = Some(body_string.into());
                request.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_static(content_type));
            }
        } else {
            let body = if let Some(body) = request_body {
                serde_urlencoded::to_string(body).or(Err("could not serialize body as application/x-www-form-urlencoded"))?
            } else {
                String::new()
            };

            let mut pairs: Vec<_> = body.split('&')
                .map(|pair| pair.split_once('=').unwrap_or((pair, "")))
                .collect();
            pairs.push(("api_key", key));
            let timestamp = timestamp.to_string();
            pairs.push(("timestamp", &timestamp));
            pairs.sort_unstable();

            let mut body_string = String::new();
            for (key, value) in pairs {
                body_string.push_str(key);
                if !value.is_empty() {
                    body_string.push('=');
                    body_string.push_str(value);
                }
                body_string.push('&');
            }
            body_string.pop(); // the last '&'

            hmac.update(body_string.as_bytes());
            let signature = hex::encode(hmac.finalize().into_bytes());

            if spot {
                body_string.push_str(&format!("sign={}", signature));

                *request.body_mut() = Some(body_string.into());

                request.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
            } else {
                let mut json = serde_json::to_value(&body).or(Err("could not serialize body as application/json"))?;
                let Some(map) = json.as_object_mut() else {
                    return Err("body must to serializable as a JSON object");
                };
                map.insert("sign".to_owned(), serde_json::Value::String(signature));

                *request.body_mut() = Some(json.to_string().into());

                request.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_static("application/json"));
            }
        }
        Ok(request)
    }

    fn type2_auth<B>(mut builder: RequestBuilder, request_body: &Option<B>, key: &str, timestamp: u128, mut hmac: Hmac<Sha256>, spot: bool)
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

        let mut request = builder.build().or(Err("failed to build reqeust"))?;

        let mut sign_contents = format!("{}{}", timestamp, key);
        let window_str = if spot {
            "recvWindow"
        } else {
            "recv_window"
        };
        let window;

        if matches!(*request.method(), Method::GET | Method::DELETE) {
            let window_pair = request.url().query_pairs().find(|(key, _)| key == window_str);
            window = if let Some((_, window_value)) = window_pair {
                sign_contents.push_str(&window_value);
                Some(window_value.into_owned())
            } else {
                None
            };
            if let Some(query) = request.url().query() {
                sign_contents.push_str(query);
            }
        } else {
            let window_value = body.as_ref().and_then(|body| body.get(window_str));
            window = if let Some(window_value) = window_value {
                let window_string = window_value.to_string();
                sign_contents.push_str(&window_string);
                Some(window_string)
            } else {
                None
            };
            if let Some(body) = body {
                sign_contents.push_str(&body.to_string());
            }
        }

        hmac.update(sign_contents.as_bytes());
        let signature = hex::encode(hmac.finalize().into_bytes());

        let headers = request.headers_mut();
        headers.insert("X-BAPI-SIGN-TYPE", HeaderValue::from(2));
        headers.insert("X-BAPI-SIGN", HeaderValue::from_str(&signature).unwrap()); // hex digits are valid
        headers.insert("X-BAPI-API-KEY", HeaderValue::from_str(key).or(Err("invalid character in API key"))?);
        headers.insert("X-BAPI-TIMESTAMP", HeaderValue::from(timestamp as u64));
        if let Some(window) = window {
            headers.insert("X-BAPI-RECV-WINDOW", HeaderValue::from_str(&window).or(Err("invalid character in recv window"))?);
        }

        Ok(request)
    }
}

impl BybitHttpUrl {
    /// The string that this variant represents.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Bybit => "https://api.bybit.com",
            Self::Bytick => "https://api.bytick.com",
            Self::Test => "https://api-testnet.bybit.com",
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
            BybitOption::RequestConfig(v) => self.request_config = v,
            BybitOption::WebSocketUrl(v) => self.websocket_url = v,
            BybitOption::WebSocketAuth(v) => self.websocket_auth = v,
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
            request_config: RequestConfig::default(),
            websocket_url: BybitWebSocketUrl::Bybit,
            websocket_auth: false,
            websocket_config,
        }
    }
}

impl<'a, R: DeserializeOwned + 'a> HttpOption<'a, R> for BybitOption {
    type RequestHandler = BybitRequestHandler<'a, R>;

    fn request_handler(options: Self::Options) -> Self::RequestHandler {
        BybitRequestHandler::<'a, R> {
            options,
            _phantom: PhantomData,
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
