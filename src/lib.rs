#[macro_use]
extern crate clap;
extern crate url;

pub mod config;
pub mod request;

use config::Config;

pub fn run(conf: Config) {
    println!("HYPHA\n{} {}", conf.method, conf.url);
}
