use url::Url;

use http::method::Method;
use http::version::HttpVersion;

#[derive(Debug)]
pub struct Request {
    pub url: Url,
    pub method: Method,
    pub version: HttpVersion,
}
