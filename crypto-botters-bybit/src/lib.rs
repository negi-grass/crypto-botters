//! A crate for communicating with the [Bybit API](https://bybit-exchange.github.io/docs/spot/v3/#t-introduction).
//! For example usages, see files in the examples/ directory.

use std::{
    time::{SystemTime, Duration},
    borrow::Cow,
    marker::PhantomData,
};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use serde::{
    Serialize,
    de::DeserializeOwned,
};
use generic_api_client::{http::*, websocket::*};

pub struct Bybit {
    api_key: Option<String>,
    api_secret: Option<String>,
    /// The url which will used for HTTP requests. Defaults to [BybitHttpUrl::Bybit].
    pub http_url: BybitHttpUrl,
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

/// A `enum` that represents the base url of the Bybit REST API.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[non_exhaustive]
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
#[non_exhaustive]
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
/// |Environment Testnet|Type2Spot|
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum BybitHttpAuth {
    Type1,
    SpotType1,
    Type2,
    SpotType2,
    None,
}

pub struct BybitRequestHandler<'a, R: DeserializeOwned> {
    api_key: Option<&'a str>,
    api_secret: Option<&'a str>,
    auth: BybitHttpAuth,
    base_url: BybitHttpUrl,
    max_try: u8,
    _phantom: PhantomData<&'a R>,
}

impl Bybit {
    pub fn request<R: DeserializeOwned>(&self, auth: BybitHttpAuth) -> BybitRequestHandler<R> {
        BybitRequestHandler {
            api_key: self.api_key.as_deref(),
            api_secret: self.api_secret.as_deref(),
            auth,
            base_url: self.http_url,
            max_try: self.request_max_try,
            _phantom: PhantomData,
        }
    }
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
        let mut config = RequestConfig::new();
        config.url_prefix = self.base_url.to_string();
        config.max_try = self.max_try;
        config
    }

    fn build_request(&self, mut builder: RequestBuilder, request_body: &Option<B>, _: u8) -> Result<Request, Self::BuildError> {
        if self.auth == BybitHttpAuth::None {
            return builder.build().or(Err("failed to build request"));
        }

        let key = self.api_key.ok_or("API key not set")?;
        let secret = self.api_key.ok_or("API secret not set")?;

        let time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap(); // always after the epoch
        let timestamp = time.as_millis();

        let mut hmac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap(); // hmac accepts key of any length

        if self.auth.is_type1() {
            let mut request = builder.build().or(Err("failed to build request"))?;

            if matches!(request.method(), &Method::GET | &Method::DELETE) {
                let mut queries: Vec<_> = request.url().query_pairs().collect();
                queries.push((Cow::Borrowed("api_key"), Cow::Borrowed(key)));
                queries.push((Cow::Borrowed("timestamp"), Cow::Owned(timestamp.to_string())));
                queries.sort_unstable();

                request.url_mut()
                    .query_pairs_mut()
                    .clear()
                    .extend_pairs(queries.iter());

                let query = request.url().query().unwrap(); // we just added api_key and timestamp

                hmac.update(query.as_bytes());
                let signature = hex::encode(hmac.finalize().into_bytes());

                request.url_mut().query_pairs_mut().append_pair("sign", &signature);

                Ok(request)
            } else {
                let body = if let Some(body) = request_body {
                    serde_urlencoded::to_string(body)
                        .or(Err("could not parse body as application/x-www-form-urlencoded"))?
                } else {
                    String::new()
                };

                let mut pairs: Vec<_> = body.split('&')
                    .map(|pair| pair.split_once('=').unwrap_or((pair, "")))
                    .collect();
                pairs.push(("api_key", key));
                pairs.push(("timestamp", &timestamp.to_string()));
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

                hmac.update(body_string.as_bytes());
                let signature = hex::encode(hmac.finalize().into_bytes());

                body_string.push_str("sign=");
                body_string.push_str(&signature);

                let request = builder.body(body_string).build();

                request.or(Err("failed to build request"))
            }
        } else {
            assert!(self.auth.is_type2());
            Ok(builder.build().unwrap())
        }
    }

    fn handle_response(&self, status: StatusCode, headers: HeaderMap, response_body: Bytes) -> Result<Self::Successful, Self::Unsuccessful> {
        todo!()
    }
}

impl BybitHttpUrl {
    /// The string that this variant represents.
    pub fn to_str(&self) -> &'static str {
        match self {
            Self::Bybit => "https://api.bybit.com",
            Self::Bytick => "https://api.bytick.com",
            Self::Test => "https://api-testnet.bybit.com",
            Self::None => "",
        }
    }
}

impl ToString for BybitHttpUrl {
    fn to_string(&self) -> String {
        self.to_str().to_owned()
    }
}

impl Default for BybitHttpUrl {
    fn default() -> Self {
        Self::Bybit
    }
}

impl BybitHttpAuth {
    fn is_type1(&self) -> bool {
        match self {
            Self::Type1 => true,
            Self::SpotType1 => true,
            Self::Type2 => false,
            Self::SpotType2 => false,
            Self::None => false,
        }
    }

    fn is_type2(&self) -> bool {
        match self {
            Self::Type1 => false,
            Self::SpotType1 => false,
            Self::Type2 => true,
            Self::SpotType2 => true,
            Self::None => false,
        }
    }

    fn is_non_spot(&self) -> bool {
        match self {
            Self::Type1 => true,
            Self::SpotType1 => false,
            Self::Type2 => true,
            Self::SpotType2 => false,
            Self::None => false,
        }
    }

    fn is_spot(&self) -> bool {
        match self {
            Self::Type1 => false,
            Self::SpotType1 => true,
            Self::Type2 => false,
            Self::SpotType2 => true,
            Self::None => false,
        }
    }
}
