//! Deserializes a .toml config into a `FuzzConfig` object that can be used to configure a
//! `ByteMutator`. See tests for examples.

use crate::mutators::{Mutation, MutatorType};
use crate::{Iterations, Stage};

use serde_derive::Deserialize;
use std::fs;
use std::io::{self, Error};
use toml;

#[derive(Deserialize, Debug, Clone)]
/// A struct that can be deserialized from .toml.
/// Creates and configures `Stage` and `Mutator` objects.
pub struct FuzzConfig {
    pub stages: Vec<Stage>,
}

impl FuzzConfig {
    pub fn default() -> Self {
        Self {
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

    pub fn from_file(path: &str) -> std::io::Result<Self> {
        match fs::read_to_string(path) {
            Ok(s) => match toml::from_str(&s) {
                Ok(c) => Ok(c),
                Err(_) => Err(Error::new(
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
