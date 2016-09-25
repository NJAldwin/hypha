#[macro_use]
extern crate clap;
extern crate hypha;

use hypha::config;

fn main() {
    hypha::run(config::parse());
}
