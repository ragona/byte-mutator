use crate::mutators::{Mutation, MutatorType};
use crate::{ByteMutator, Stage};
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
        let config: FuzzConfig = toml::from_str(
            r#"
                [[stages]]
                    # todo: Reconsider exposing the state like this
                    cur_iterations = 0
                    max_iterations = 100
                    
                    # a list of mutations to perform on this stage
                    [[stages.mutations]]
                        mutation = {"BitFlipper" = {width = 1}}
                        range = [0, 10]
                        width = 1
        "#,
        )
        .unwrap();

        let mut mutator = ByteMutator::new_from_config(b"foo", config);

        dbg!(mutator);
    }
}
