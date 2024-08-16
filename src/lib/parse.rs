use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// TODO: Define a TOLS spec once
// https://github.com/toml-lang/toml/pull/116 merges
//
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repo {
    pub url: String,
    pub branch: Option<String>,
    pub remote_path: Option<String>,
    pub local_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub repos: std::collections::HashMap<String, Repo>,
}

pub fn parse_config<P: Into<PathBuf>>(p: P) {
    let config_string: String = std::fs::read_to_string(p.into()).expect("Could not read from file");
    let config: Config = toml::from_str(&config_string).expect("Could not parse config");

    let t = toml::to_string_pretty(&config).unwrap();
    println!("{t}");
}

#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use super::*;
    use tempdir::TempDir;

    #[test]
    fn test_serialize() -> Result<(), std::io::Error> {
        let config: Config = Config {
            repos: std::collections::HashMap::from([
                (
                    "personal".into(),
                    Repo {
                        url: "foo".into(),
                        branch: None,
                        remote_path: None,
                        local_path: None,
                    }),
            ])
        };

        assert_eq!(toml::to_string(&config).unwrap(), r#"[repos.personal]
url = "foo"
"#);

        Ok(())
    }
}
