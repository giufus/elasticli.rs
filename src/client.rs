
use std::error::Error;
use std::ops::Add;
use async_trait::async_trait;
use serde_json::{Value};

use crate::{arguments::Api, configuration::Config};
use crate::arguments::{AddArgs, Arguments, Method};


#[async_trait]
pub trait ElasticApi {
    async fn execute_command(&self, config: &Config, arguments: Arguments) -> Result<Value, Box<dyn Error>>;
    fn get_elasticsearch_baseurl(&self, config: &Config) -> String;
    async fn info_command(&self, config: &Config, arguments: Arguments) -> Result<Value, Box<dyn Error>>;
    async fn index_command(&self, config: &Config, arguments: AddArgs) -> Result<Value, Box<dyn Error>>;
    async fn search_command(&self, config: &Config, arguments: AddArgs) -> Result<Value, Box<dyn Error>>;
}

pub struct ElasticApiClient {}

#[async_trait]
impl ElasticApi for ElasticApiClient {
    async fn execute_command(&self, config: &Config, arguments: Arguments) -> Result<Value, Box<dyn Error>> {
        match arguments.api {
            Api::Info(_) => self.info_command(config, arguments).await,
            Api::Index(a) => self.index_command(config, a).await,
            Api::Search(a) => self.search_command(config, a).await,
        }
    }

    fn get_elasticsearch_baseurl(&self, config: &Config) -> String {
        match &config.proxy {
            Some(p) if p.enabled == true => {
                format!("{}://{}:{}", p.protocol, "127.0.0.1", p.port)
            }
            _ => format!("{}://{}:{}", config.elastic.protocol, config.elastic.host, config.elastic.port),
        }
    }

    async fn info_command(&self, config: &Config, _arguments: Arguments) -> Result<Value, Box<dyn Error>> {
        let resp = reqwest::get(self.get_elasticsearch_baseurl(&config))
            .await?
            .json::<Value>()
            .await;

        match resp {
            Ok(v) => Ok(v),
            Err(e) => Err(Box::new(e))
        }
    }

    async fn index_command(&self, config: &Config, arguments: AddArgs) -> Result<Value, Box<dyn Error>> {

        let client = reqwest::Client::new();
        let base_url = self.get_elasticsearch_baseurl(&config);
        let method = arguments.method;
        let index = arguments.index_name.unwrap_or("_all".to_string());
        let id = arguments.id.unwrap_or("".to_string());
        let resp;


        match method {

            // get index info
            Method::Get => {
                resp = reqwest::get(&base_url
                    .add("/").add(&index)
                ).await?.json::<Value>().await;
            },

            // index a document
            Method::Post => {
                resp = client.post(&base_url
                    .add("/").add(&index).add("/_doc")
                ).body(arguments.body.expect("body required"))
                    .header("Content-type", "application/json")
                .send().await?.json::<Value>().await;
            },

            // update a document
            Method::Options => {
                resp = client.post(&base_url
                    .add("/").add(&index).add("/_update/").add(&id)
                ).body(arguments.body.expect("body required"))
                    .header("Content-type", "application/json")
                .send().await?.json::<Value>().await;
            },

            // create index
            Method::Put => {
                resp = client.put(&base_url
                    .add("/").add(&index)
                ).body(arguments.body.expect("body required"))
                    .send().await?.json::<Value>().await;
            },

            // delete index
            Method::Delete => {
                resp = client.delete(&base_url
                    .add("/").add(&index)
                ).send().await?.json::<Value>().await;
            },

        }

        match resp {
            Ok(v) => Ok(v),
            Err(e) => Err(Box::new(e))
        }


    }

    async fn search_command(&self, config: &Config, arguments: AddArgs) -> Result<Value, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let base_url = self.get_elasticsearch_baseurl(&config);
        let method = arguments.method;
        let index = arguments.index_name.unwrap_or("_all".to_string());
        let resp;

        match method {
            Method::Post => {
                resp = client.post(&base_url
                    .add("/").add(&index).add("/_search")
                ).body(arguments.body.expect("body required"))
                    .header("Content-type", "application/json")
                    .send().await?.json::<Value>().await;

                match resp {
                    Ok(v) => Ok(v),
                    Err(e) => Err(Box::new(e))
                }
            },
            _ => Err("only post method is supported")?
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::arguments::Method::Get;
    use crate::configuration::{get_configuration};

    use super::*;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn test_get_elasticsearch_baseurl_when_no_config_file_provided() {
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

        let conf = get_configuration(&arguments);


        let client = ElasticApiClient {};

        let base_url = client.get_elasticsearch_baseurl(&conf);
        assert_eq!(base_url, "http://127.0.0.1:9200");
    }

    #[test]
    fn test_get_elasticsearch_baseurl_when_config_file_provided_and_proxy_disabled() {
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

        let conf = get_configuration(&arguments);


        let client = ElasticApiClient {};

        let base_url = client.get_elasticsearch_baseurl(&conf);
        assert_eq!(base_url, "http://otherhost:9200");
    }


    #[test]
    fn test_get_elasticsearch_baseurl_when_config_file_provided_and_proxy_enabled() {
        let arguments: Arguments = Arguments {
            config: Some("./samples/proxy_enabled".to_string()),
            api: Api::Info(AddArgs {
                index_name: None,
                method: Get,
                body: None,
                page: None,
                id: None,
            }
            ),
        };

        let conf = get_configuration(&arguments);


        let client = ElasticApiClient {};

        let base_url = client.get_elasticsearch_baseurl(&conf);
        assert_eq!(base_url, "http://127.0.0.1:9201");
    }
}