#![doc = include_str!("../README.md")]

use fastrace::prelude::*;
use reqwest::header::HeaderMap;
use reqwest::header::HeaderName;
use reqwest::header::HeaderValue;

/// The standard [W3C Trace Context](https://www.w3.org/TR/trace-context/) header name for passing trace information.
///
/// This is the header key used to propagate trace context between services according to
/// the W3C Trace Context specification.
pub const TRACEPARENT_HEADER: &str = "traceparent";

/// Creates a [`HeaderMap`] containing trace context headers from the current span context.
///
/// This function extracts the current span context from the local thread and formats it
/// according to the W3C Trace Context specification as a `traceparent` header.
///
/// # Returns
///
/// * If a local parent span exists: A `HeaderMap` containing the traceparent header with the
///   encoded span context
/// * If no local parent span exists: An empty `HeaderMap`
///
/// # Example
///
/// ```
/// use fastrace::prelude::*;
/// use fastrace_reqwest::traceparent_headers;
/// use reqwest::Client;
///
/// #[fastrace::trace]
/// async fn send_request() {
///     let client = Client::new();
///     let response = client
///         .get("https://example.com")
///         .headers(traceparent_headers())
///         .send()
///         .await
///         .unwrap();
/// }
/// ```
pub fn traceparent_headers() -> HeaderMap {
    if let Some(parent) = SpanContext::current_local_parent() {
        let key = HeaderName::from_static(TRACEPARENT_HEADER);
        let value = HeaderValue::from_str(&parent.encode_w3c_traceparent()).unwrap();
        [(key, value)].into_iter().collect()
    } else {
        HeaderMap::new()
    }
}
