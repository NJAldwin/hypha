use std::fmt;
use std::str;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum HttpVersion {
    Http10,
    Http11,
}

impl Default for HttpVersion {
    fn default() -> HttpVersion {
        HttpVersion::Http11
    }
}

impl AsRef<str> for HttpVersion {
    fn as_ref(&self) -> &str {
        match *self {
            HttpVersion::Http10 => "HTTP/1.0",
            HttpVersion::Http11 => "HTTP/1.1",
        }
    }
}

impl fmt::Display for HttpVersion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.as_ref())
    }
}

impl str::FromStr for HttpVersion {
    type Err = ();

    fn from_str(s: &str) -> Result<HttpVersion, ()> {
        match s.to_string().to_uppercase().as_ref() {
            "HTTP/1.0" => Ok(HttpVersion::Http10),
            "HTTP/1.1" => Ok(HttpVersion::Http11),
            _ => Err(()),
        }
    }
}
