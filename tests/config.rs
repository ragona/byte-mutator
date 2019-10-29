extern crate byte_mutator;

use byte_mutator::config::Config;
use std::fs;

#[test]
fn from_file() {
    let config = Config::from_file("examples/config.toml").unwrap();

    dbg!(config);
}
