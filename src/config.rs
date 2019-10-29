use serde_derive::Deserialize;
use std::fs;
use std::io::{self, Error};
use std::path::Path;
use toml;

#[derive(Deserialize, Debug)]
pub struct Config {
    seeds: Vec<SeedConfig>,
}

impl Config {
    pub fn from_file(path: &str) -> std::io::Result<Config> {
        match fs::read_to_string(path) {
            Ok(s) => match toml::from_str(&s) {
                Ok(c) => Ok(c),
                Err(e) => Err(Error::new(
                    io::ErrorKind::InvalidInput,
                    "Failed to parse config",
                )),
            },
            Err(e) => Err(e),
        }
    }
}

#[derive(Deserialize, Debug)]
struct SeedConfig {
    path: String,
    mutations: Option<Vec<MutationConfig>>,
}

#[derive(Deserialize, Debug)]
struct MutationConfig {
    name: String,
    iterations: u32,
    range: Option<[u32; 2]>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_from_string() {
        let config: Config = toml::from_str(
            r#"
            # a list of seeds to start from
            [[seeds]]
                path = "foo/path"
                    # a list of mutations to try on this seed
                    [[seeds.mutations]]
                        name = "bitflipper"
                        iterations = 10
                        range = [0, 10]
        "#,
        )
        .unwrap();

        dbg!(config);
    }
}
