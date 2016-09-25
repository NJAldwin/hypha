use std::fmt;
use std::str;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Method {
    Options,
    Get,
    Head,
    Post,
    Put,
    Delete,
    Trace,
    Connect,
}

impl Default for Method {
    fn default() -> Method {
        Method::Get
    }
}

impl AsRef<str> for Method {
    fn as_ref(&self) -> &str {
        match *self {
            Method::Options => "OPTIONS",
            Method::Get => "GET",
            Method::Head => "HEAD",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Trace => "TRACE",
            Method::Connect => "CONNECT",
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.as_ref())
    }
}

impl str::FromStr for Method {
    type Err = ();

    fn from_str(s: &str) -> Result<Method, ()> {
        match s.to_string().to_uppercase().as_ref() {
            "OPTIONS" => Ok(Method::Options),
            "GET" => Ok(Method::Get),
            "HEAD" => Ok(Method::Head),
            "POST" => Ok(Method::Post),
            "PUT" => Ok(Method::Put),
            "DELETE" => Ok(Method::Delete),
            "TRACE" => Ok(Method::Trace),
            "CONNECT" => Ok(Method::Connect),
            _ => Err(()),
        }
    }
}
