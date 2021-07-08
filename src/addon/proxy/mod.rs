pub mod handler;

use http::header::HeaderName;
use http::request::Builder as HttpRequestBuilder;
use http::{Request, Response, StatusCode};
use hyper::server::conn::Http;
use hyper::Body;
use std::sync::Arc;

/// End-to-end and Hop-by-hop Headers
///
/// For the purpose of defining the behavior of caches and non-caching proxies, we divide HTTP headers into two categories:
///
///       - End-to-end headers, which are  transmitted to the ultimate
///         recipient of a request or response. End-to-end headers in
///         responses MUST be stored as part of a cache entry and MUST be
///         transmitted in any response formed from a cache entry.
///       - Hop-by-hop headers, which are meaningful only for a single
///         transport-level connection, and are not stored by caches or
///         forwarded by proxies.
/// The following HTTP/1.1 headers are hop-by-hop headers:
///
///       - Connection
///       - Keep-Alive
///       - Proxy-Authenticate
///       - Proxy-Authorization
///       - TE
///       - Trailers
///       - Transfer-Encoding
///       - Upgrade
/// All other headers defined by HTTP/1.1 are end-to-end headers.
///
/// Refer: https://www.w3.org/Protocols/rfc2616/rfc2616-sec13.html (13.5.1)
const HOP_BY_HOP_HEADERS: [&str; 8] = [
    "Connection",
    "Keep-Alive",
    "Proxy-Authentication",
    "Proxy-Authorization",
    "Te",
    "Trailers",
    "Transfer-Encoding",
    "Upgrade",
];

pub struct Proxy;

impl Proxy {
    pub async fn handle(&self, request: Arc<Request<Body>>) -> Response<Body> {
        todo!()
    }

    /// Creates a new HTTP Request with the same defintion as
    /// the original one by with Hop-by-Hop headers removed
    fn remove_hop_by_hop_headers(og_request: Request<Body>) -> Request<Body> {
        let (mut parts, body) = og_request.into_parts();
        let mut request = HttpRequestBuilder::new();
        let headers = request.headers_mut().unwrap();

        for header in HOP_BY_HOP_HEADERS.iter() {
            let header: HeaderName = header.parse().unwrap();

            if headers.contains_key::<HeaderName>(header.clone()) {
                headers.remove(header);
            }
        }

        request.body(body).unwrap()
    }
}
