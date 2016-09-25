#[macro_use]
extern crate clap;
extern crate url;

pub mod config;
pub mod http;

use config::Config;
use http::client;
use http::request::Request;

pub fn run(conf: Config) {
    println!("HYPHA\n{} {}", conf.method, conf.url);
    client::execute(Request {
        url: conf.url,
        method: conf.method,
    });
}
