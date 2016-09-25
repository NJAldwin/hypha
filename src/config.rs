extern crate clap;

use clap::App;

pub struct Config {}

pub fn parse() -> Config {
    App::new("hypha")
        .author(crate_authors!())
        .version(crate_version!())
        .about("toy HTTP client")
        .get_matches();

    Config {}
}
