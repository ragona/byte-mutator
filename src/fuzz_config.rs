use crate::mutators::{Mutation, MutatorType};
use crate::{ByteMutator, Iterations, Stage};
use serde_derive::Deserialize;
use std::fs;
use std::io::{self, Error};
use std::path::Path;
use toml;

#[derive(Deserialize, Debug, Clone)]
pub struct FuzzConfig {
    pub stages: Vec<Stage>,
}

impl FuzzConfig {
    pub fn default() -> FuzzConfig {
        FuzzConfig {
            stages: vec![Stage {
                count: 0,
                iterations: Iterations::Unlimited,
                mutations: vec![Mutation {
                    range: None,
                    mutation: MutatorType::BitFlipper { width: 1 },
                }],
            }],
        }
    }
    pub fn from_file(path: &str) -> std::io::Result<FuzzConfig> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_from_string() {
        let config = toml::from_str::<FuzzConfig>(
            r#"
                [[stages]]
                    # todo: Reconsider exposing the state like this
                    count = 0
                    iterations = { "Limited" = 10 }
                    
                    # a list of mutations to perform on this stage
                    [[stages.mutations]]
                        mutation = {"BitFlipper" = {width = 1}}
                        range = [0, 10]
                        width = 1
        "#,
        );

        let error = toml::from_str::<FuzzConfig>("foo");

        assert!(config.is_ok());
        assert!(error.is_err());
    }

    #[test]
    fn config_from_file() {
        assert!(FuzzConfig::from_file("tests/fuzz_config.toml").is_ok());
        assert!(FuzzConfig::from_file("").is_err());
    }
}
