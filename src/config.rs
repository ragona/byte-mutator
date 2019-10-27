use serde_derive::Deserialize;
use toml;

#[derive(Deserialize, Debug)]
struct Config {
    seeds: Vec<SeedConfig>,
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
    range: [u32; 2],
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
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
