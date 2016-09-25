use clap::{App, Arg};
use url::Url;

pub struct Config {
    pub url: Url,
}

pub fn parse() -> Config {
    let matches = App::new("hypha")
        .author(crate_authors!())
        .version(crate_version!())
        .about("toy HTTP client")
        .arg(Arg::with_name("url")
            .help("The URL")
            .required(true))
        .get_matches();

    Config {
        // TODO @NJA: allow elision of scheme (default to http)
        url: value_t!(matches.value_of("url"), Url).unwrap_or_else(|e| e.exit()),
    }
}
