#[macro_use]
extern crate clap;
extern crate url;

pub mod config;

use config::Config;

pub fn run(conf: Config) {
    println!("HYPHA\nURL: {}", conf.url);
}
