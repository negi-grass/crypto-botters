//! A module for communicating with the [bitbank API](https://github.com/bitbankinc/bitbank-api-docs).
//! For example usages, see files in the examples/ directory.

use std::marker::PhantomData;
use serde::de::DeserializeOwned;
use serde::Serialize;
use generic_api_client::http::{Bytes, header, HeaderMap, Request, RequestBuilder, RequestConfig, RequestError, RequestHandler, StatusCode};
use crate::traits::{HandlerOption, HandlerOptions, HttpOption};

pub type BitbankRequestResult<T> = Result<T, BitbankRequestError>;
pub type BitbankRequestError = RequestError<&'static str, BitbankHandlerError>;

pub enum BitbankOption {
    Default,
    HttpUrl(BitbankHttpUrl),
    RequestConfig(RequestConfig),
}

#[derive(Clone, Debug)]
pub struct BitbankOptions {
    pub http_url: BitbankHttpUrl,
    pub request_config: RequestConfig,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum BitbankHttpUrl {
    /// `https://public.bitbank.cc`
    Default,
    None,
}

#[derive(Debug)]
pub enum BitbankHandlerError {
    /// https://github.com/bitbankinc/bitbank-api-docs/blob/master/errors.md
    ApiError { code: Option<u64> },
    ParseError,
}

pub struct BitbankRequestHandler<'a, R: DeserializeOwned> {
    options: BitbankOptions,
    _phantom: PhantomData<&'a R>,
}

impl<'a, B, R> RequestHandler<B> for BitbankRequestHandler<'a, R>
where
    B: Serialize,
    R: DeserializeOwned,
{
    type Successful = R;
    type Unsuccessful = BitbankHandlerError;
    type BuildError = &'static str;

    fn request_config(&self) -> RequestConfig {
        let mut config = self.options.request_config.clone();
        if self.options.http_url != BitbankHttpUrl::None {
            config.url_prefix = self.options.http_url.as_str().to_owned();
        }
        config
    }

    fn build_request(&self, mut builder: RequestBuilder, request_body: &Option<B>, _attempt_count: u8) -> Result<Request, Self::BuildError> {
        if let Some(body) = request_body {
            let json = serde_json::to_vec(body).or(Err("could not serialize body as application/json"))?;
            builder = builder
                .header(header::CONTENT_TYPE, "application/json")
                .body(json);
        }

        builder.build().or(Err("failed to build request"))
    }

    fn handle_response(&self, status: StatusCode, _headers: HeaderMap, response_body: Bytes) -> Result<Self::Successful, Self::Unsuccessful> {
        if status.is_success() {
            serde_json::from_slice(&response_body).map_err(|error| {
                log::debug!("Failed to parse response due to an error: {}", error);
                BitbankHandlerError::ParseError
            })
        } else {
            let error = match serde_json::from_slice::<serde_json::Value>(&response_body) {
                Ok(parsed_error) => {
                    let code = parsed_error["data"]["code"].as_u64();
                    BitbankHandlerError::ApiError { code }
                },
                Err(error) => {
                    log::debug!("Failed to parse error response due to an error: {}", error);
                    BitbankHandlerError::ParseError
                }
            };
            Err(error)
        }
    }
}

impl BitbankHttpUrl {
    #[inline(always)]
    fn as_str(&self) -> &'static str {
        match self {
            Self::Default => "https://public.bitbank.cc",
            Self::None => "",
        }
    }
}

impl HandlerOptions for BitbankOptions {
    type OptionItem = BitbankOption;

    fn update(&mut self, option: Self::OptionItem) {
        match option {
            BitbankOption::Default => (),
            BitbankOption::HttpUrl(v) => self.http_url = v,
            BitbankOption::RequestConfig(v) => self.request_config = v,
        }
    }
}

impl Default for BitbankOptions {
    fn default() -> Self {
        Self {
            http_url: BitbankHttpUrl::Default,
            request_config: RequestConfig::default(),
        }
    }
}

impl <'a, R, B> HttpOption<'a, R, B> for BitbankOption
where
    R: DeserializeOwned + 'a,
    B: Serialize,
{
    type RequestHandler = BitbankRequestHandler<'a, R>;

    #[inline(always)]
    fn request_handler(options: Self::Options) -> Self::RequestHandler {
        BitbankRequestHandler::<'a, R> {
            options,
            _phantom: PhantomData,
        }
    }
}

impl HandlerOption for BitbankOption {
    type Options = BitbankOptions;
}

impl Default for BitbankOption {
    fn default() -> Self {
        Self::Default
    }
}
