

use std::path::PathBuf;
use hydroconf::{Hydroconf, HydroSettings};
use serde::Deserialize;
use crate::arguments::Arguments;


#[derive(Debug, Deserialize)]
pub struct Config {
    pub elastic: ElasticConfig,
    pub proxy: Option<Proxy>,
}

#[derive(Debug, Deserialize)]
pub struct ElasticConfig {
    pub host: String,
    pub port: u16,
    pub protocol: String,
    pub username: String,
    pub password: String,
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct Proxy {
    pub enabled: bool,
    pub host: String,
    pub port: u16,
    pub protocol: String,
    pub key: String,
    pub user: String,
    pub remote_user: String,
    pub timeout: u8,
}

pub fn get_configuration(arguments: &Arguments) -> Config {
    match &arguments.config {
        None => {
            // Default Config
            Config {
                elastic: ElasticConfig {
                    protocol: "http".to_string(),
                    host: "127.0.0.1".to_string(),
                    port: 9200,
                    version: "8.8.0".to_string(),
                    username: "elastic".to_string(),
                    password: "changeme".to_string(),
                },
                proxy: None,
            }
        }
        Some(f) => {
            let root = PathBuf::from(&f);
            //let settings = PathBuf::from(format!("{}/settings.toml", &f));
            //let secrets = PathBuf::from(format!("{}/.secrets.toml", &f));
            if root.exists() {
                let hydroconf = Hydroconf::new(
                    HydroSettings::default().set_root_path(root)
                ).hydrate();
                match hydroconf {
                    Ok(c) => c,
                    Err(e) => {
                        println!("could not read configuration: {:#?}", e);
                        std::process::exit(1);
                    }
                }
            } else {
                println!("settings.toml and .secrets.toml not found in path {}", &f);
                std::process::exit(1);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::arguments::{AddArgs, Api, Arguments};
    use crate::arguments::Method::Get;

    #[test]
    fn test_get_configuration_use_defaults_when_no_config_basedir_provided() {

        // config None
        let arguments: Arguments = Arguments {
            config: None,
            api: Api::Info(AddArgs {
                index_name: None,
                method: Get,
                body: None,
                page: None,
                id: None,
            }
            ),
        };

        let config = get_configuration(&arguments);

        assert_eq!(config.elastic.host, "127.0.0.1".to_string());
        assert_eq!(config.elastic.port, 9200);
        assert_eq!(config.elastic.version, "8.8.0".to_string(),);
        assert_eq!(config.elastic.protocol, "http".to_string());
        assert_eq!(config.elastic.username, "elastic".to_string());
        assert_eq!(config.elastic.password, "changeme".to_string());
    }

    #[test]
    fn test_get_configuration_use_files_when_config_basedir_provided() {

        // config None
        let arguments: Arguments = Arguments {
            config: Some("./samples/default".to_string()),
            api: Api::Info(AddArgs {
                index_name: None,
                method: Get,
                body: None,
                page: None,
                id: None,
            }
            ),
        };

        let config = get_configuration(&arguments);

        assert_eq!(config.elastic.host, "otherhost".to_string());
        assert_eq!(config.elastic.port, 9200);
        assert_eq!(config.elastic.protocol, "http".to_string());
        assert_eq!(config.elastic.version, "8.8.0".to_string(),);
        assert_eq!(config.elastic.username, "elastic".to_string());
        assert_eq!(config.elastic.password, "secure_password".to_string());
    }
}