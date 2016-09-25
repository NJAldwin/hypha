use url::Url;

use http::method::Method;

#[derive(Debug)]
pub struct Request {
    pub url: Url,
    pub method: Method,
}
