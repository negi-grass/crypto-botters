use std::time::Duration;
use serde::Serialize;
pub use reqwest::{Request, RequestBuilder, StatusCode, Method, header::{self, HeaderMap}};
pub use bytes::Bytes;

/// Client for communicating with APIs through HTTP/HTTPS.
///
/// When making a HTTP request or starting a websocket connection with this client,
/// a handler that implements [RequestHandler] is required.
#[derive(Debug, Clone, Default)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    /// Constructs a new `Client`.
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Makes an HTTP request with the given [RequestHandler] and returns the response.
    ///
    /// It is recommended to use methods like [get()][Self::get()] because this method takes many type parameters and parameters.
    ///
    /// The request is passed to `handler` before being sent, and the response is passed to `handler` before being returned.
    /// Note, that as stated in the docs for [RequestBuilder::query()], parameter `query` only accepts a **sequence of** key-value pairs.
    pub async fn request<Q, B, H>(
        &self, method: Method, url: &str, query: Option<&Q>, body: Option<B>, handler: &H,
    ) -> Result<H::Successful, RequestError<H::BuildError, H::Unsuccessful>>
    where
        Q: Serialize + ?Sized,
        H: RequestHandler<B>,
    {
        let config = handler.request_config();
        config.verify();
        let url = config.url_prefix + url;
        let mut count = 1;
        loop {
            // create RequestBuilder
            let mut request_builder = self.client.request(method.clone(), url.clone())
                .timeout(config.timeout);
            if let Some(query) = query {
                request_builder = request_builder.query(query);
            }
            let request = handler.build_request(request_builder, &body, count)
                .map_err(|error| {
                    RequestError::BuildRequestError(error)
                })?;
            // send the request
            match self.client.execute(request).await {
                Ok(mut response) => {
                    let status = response.status();
                    let headers = std::mem::take(response.headers_mut());
                    let body = response.bytes().await.map_err(|error| {
                        RequestError::ReceiveResponse(error)
                    })?;
                    return handler.handle_response(status, headers, body).map_err(|error| {
                        RequestError::ResponseHandleError(error)
                    });
                },
                Err(error) => {
                    if count >= config.max_try {
                        // max retry count
                        return Err(RequestError::SendRequest(error));
                    }
                    log::warn!("Retrying sending reqeust");
                    // else, continue
                    count += 1;
                    tokio::time::sleep(config.retry_cooldown).await;
                },
            }
        }
    }

    /// Makes an GET request with the given [RequestHandler].
    ///
    /// This method just calls [request()][Self::request()]. It requires less typing for type parameters and parameters.
    /// This method requires that `handler` can handle a request with a body of type `()`. The actual body passed will be `None`.
    ///
    /// For more information, see [request()][Self::request()].
    #[inline(always)]
    pub async fn get<Q, H>(&self, url: &str, query: Option<&Q>, handler: &H) -> Result<H::Successful, RequestError<H::BuildError, H::Unsuccessful>>
    where
        Q: Serialize + ?Sized,
        H: RequestHandler<()>,
    {
        self.request::<Q, (), H>(Method::GET, url, query, None, handler).await
    }

    /// Makes an GET request with the given [RequestHandler], without queries.
    ///
    /// This method just calls [request()][Self::request()]. It requires less typing for type parameters and parameters.
    /// This method requires that `handler` can handle a request with a body of type `()`. The actual body passed will be `None`.
    ///
    /// For more information, see [request()][Self::request()].
    #[inline(always)]
    pub async fn get_no_query<H>(&self, url: &str, handler: &H) -> Result<H::Successful, RequestError<H::BuildError, H::Unsuccessful>>
    where
        H: RequestHandler<()>,
    {
        self.request::<&[(&str, &str)], (), H>(Method::GET, url, None, None, handler).await
    }

    /// Makes an POST request with the given [RequestHandler].
    ///
    /// This method just calls [request()][Self::request()]. It requires less typing for type parameters and parameters.
    ///
    /// For more information, see [request()][Self::request()].
    #[inline(always)]
    pub async fn post<B, H>(&self, url: &str, body: Option<B>, handler: &H) -> Result<H::Successful, RequestError<H::BuildError, H::Unsuccessful>>
    where
        H: RequestHandler<B>,
    {
        self.request::<(), B, H>(Method::POST, url, None, body, handler).await
    }

    /// Makes an POST request with the given [RequestHandler], without a body.
    ///
    /// This method just calls [request()][Self::request()]. It requires less typing for type parameters and parameters.
    /// This method requires that `handler` can handle a request with a body of type `()`. The actual body passed will be `None`.
    ///
    /// For more information, see [request()][Self::request()].
    #[inline(always)]
    pub async fn post_no_body<H>(&self, url: &str, handler: &H) -> Result<H::Successful, RequestError<H::BuildError, H::Unsuccessful>>
    where
        H: RequestHandler<()>,
    {
        self.request::<(), (), H>(Method::POST, url, None, None, handler).await
    }

    /// Makes an PUT request with the given [RequestHandler].
    ///
    /// This method just calls [request()][Self::request()]. It requires less typing for type parameters and parameters.
    ///
    /// For more information, see [request()][Self::request()].
    #[inline(always)]
    pub async fn put<B, H>(&self, url: &str, body: Option<B>, handler: &H) -> Result<H::Successful, RequestError<H::BuildError, H::Unsuccessful>>
    where
        H: RequestHandler<B>,
    {
        self.request::<(), B, H>(Method::PUT, url, None, body, handler).await
    }

    /// Makes an PUT request with the given [RequestHandler], without a body.
    ///
    /// This method just calls [request()][Self::request()]. It requires less typing for type parameters and parameters.
    /// This method requires that `handler` can handle a request with a body of type `()`. The actual body passed will be `None`.
    ///
    /// For more information, see [request()][Self::request()].
    #[inline(always)]
    pub async fn put_no_body<H>(&self, url: &str, handler: &H) -> Result<H::Successful, RequestError<H::BuildError, H::Unsuccessful>>
    where
        H: RequestHandler<()>,
    {
        self.request::<(), (), H>(Method::PUT, url, None, None, handler).await
    }

    /// Makes an DELETE request with the given [RequestHandler].
    ///
    /// This method just calls [request()][Self::request()]. It requires less typing for type parameters and parameters.
    /// This method requires that `handler` can handle a request with a body of type `()`. The actual body passed will be `None`.
    ///
    /// For more information, see [request()][Self::request()].
    #[inline(always)]
    pub async fn delete<Q, H>(&self, url: &str, query: Option<&Q>, handler: &H) -> Result<H::Successful, RequestError<H::BuildError, H::Unsuccessful>>
    where
        Q: Serialize + ?Sized,
        H: RequestHandler<()>,
    {
        self.request::<Q, (), H>(Method::DELETE, url, query, None, handler).await
    }

    /// Makes an DELETE request with the given [RequestHandler], without queries.
    ///
    /// This method just calls [request()][Self::request()]. It requires less typing for type parameters and parameters.
    /// This method requires that `handler` can handle a request with a body of type `()`. The actual body passed will be `None`.
    ///
    /// For more information, see [request()][Self::request()].
    #[inline(always)]
    pub async fn delete_no_query<H>(&self, url: &str, handler: &H) -> Result<H::Successful, RequestError<H::BuildError, H::Unsuccessful>>
    where
        H: RequestHandler<()>,
    {
        self.request::<&[(&str, &str)], (), H>(Method::DELETE, url, None, None, handler).await
    }
}

/// A `trait` which is used to process requests and responses for the [Client].
pub trait RequestHandler<B> {
    /// The type which is returned to the caller of [Client::request()] when the response was successful.
    type Successful;
    /// The type which is returned to the caller of [Client::request()] when the response was unsuccessful.
    type Unsuccessful;
    /// The type that represents an error occurred in [build_request()][Self::build_request()].
    type BuildError;

    /// Returns a [RequestConfig] that will be used to send a HTTP reqeust.
    fn request_config(&self) -> RequestConfig {
        RequestConfig::default()
    }

    /// Build a HTTP request to be sent.
    ///
    /// Implementors have to decide how to include the `request_body` into the `builder`. Implementors can
    /// also perform other operations (such as authorization) on the request.
    fn build_request(&self, builder: RequestBuilder, request_body: &Option<B>, attempt_count: u8) -> Result<Request, Self::BuildError>;

    /// Handle a HTTP response before it is returned to the caller of [Client::request()].
    ///
    /// You can verify, parse, etc... the response here before it is returned to the caller.
    ///
    /// # Examples
    /// ```
    /// # use bytes::Bytes;
    /// # use reqwest::{StatusCode, header::HeaderMap};
    /// # trait Ignore {
    /// fn handle_response(&self, status: StatusCode, _: HeaderMap, response_body: Bytes) -> Result<String, ()> {
    ///     if status.is_success() {
    ///         let body = std::str::from_utf8(&response_body).expect("body should be valid UTF-8").to_owned();
    ///         Ok(body)
    ///     } else {
    ///         Err(())
    ///     }
    /// }
    /// # }
    /// ```
    fn handle_response(&self, status: StatusCode, headers: HeaderMap, response_body: Bytes) -> Result<Self::Successful, Self::Unsuccessful>;
}

/// Configuration when sending a request using [Client].
///
/// Should be returned by [RequestHandler::request_config()].
#[derive(Debug, Clone)]
pub struct RequestConfig {
    /// [Client] will retry sending a request if it failed to send. `max_try` can be used limit the number of attempts.
    ///
    /// Do not set this to `0` or [Client::request()] will **panic**. The first attempt is counted [Default]s to `1` (which means no retry).
    pub max_try: u8,
    /// Duration that should elapse after retrying sending a request.
    ///
    /// [Default]s to 500ms. See also: `max_try`.
    pub retry_cooldown: Duration,
    /// The timeout set when sending a request. [Default]s to 3s.
    ///
    /// It is possible for the [RequestHandler] to override this in [RequestHandler::build_request()].
    /// See also: [RequestBuilder::timeout()].
    pub timeout: Duration,
    /// The prefix which will be used for requests sent using this configuration. [Default]s to `""`.
    ///
    /// Example usage: `"https://example.com"`
    pub url_prefix: String,
}

impl RequestConfig {
    /// Constructs a new `RequestConfig` with its fields set to [default][RequestConfig::default()].
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline(always)]
    fn verify(&self) {
        assert_ne!(self.max_try, 0, "RequestConfig.max_try must not be equal to 0");
    }
}

impl Default for RequestConfig {
    fn default() -> Self {
        Self {
            max_try: 1,
            retry_cooldown: Duration::from_millis(500),
            timeout: Duration::from_secs(3),
            url_prefix: String::new(),
        }
    }
}

use std::fmt::{Debug};
use thiserror::Error;

/// An `enum` that represents errors that could be returned by [Client::request()]
///
/// Type parameter `R` is [RequestHandler::Unsuccessful].
#[derive(Error, Debug)]
pub enum RequestError<E, R> {
    /// An error which occurred while sending a HTTP request.
    #[error("failed to send reqeust")]
    SendRequest(#[source] reqwest::Error),
    /// An error which occurred while receiving a HTTP response.
    #[error("failed to receive response")]
    ReceiveResponse(#[source] reqwest::Error),
    /// Error occurred in [RequestHandler::build_request()].
    #[error("the handler failed to build a request")]
    BuildRequestError(E),
    /// An error which was returned by [RequestHandler].
    #[error("the response handler returned an error")]
    ResponseHandleError(R),
}
