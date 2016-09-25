#[macro_use]
extern crate clap;

pub mod config;

use config::Config;

pub fn run(conf: Config) {
    println!("HYPHA");
}
