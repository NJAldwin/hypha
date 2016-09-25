extern crate hypha;

use hypha::config;

fn main() {
    hypha::run(config::parse());
}
