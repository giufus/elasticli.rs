
use std::error::Error;

use async_trait::async_trait;
use serde_json::Value;

use crate::{arguments::Api, configuration::Config};
use crate::arguments::{AddArgs, Arguments, Operation};

#[async_trait]
pub trait ElasticApi {
    async fn execute_command(&self, config: &Config, arguments: Arguments) -> Result<Value, Box<dyn Error>>;
    fn get_elasticsearch_baseurl(&self, config: &Config) -> String;
    async fn info_command(&self, config: &Config, arguments: Arguments) -> Result<Value, Box<dyn Error>>;
    async fn index_command(&self, config: &Config, arguments: AddArgs) -> Result<Value, Box<dyn Error>>;
    async fn document_command(&self, config: &Config, arguments: AddArgs) -> Result<Value, Box<dyn Error>>;
}

pub struct ElasticApiClient {}

#[async_trait]
impl ElasticApi for ElasticApiClient {
    async fn execute_command(&self, config: &Config, arguments: Arguments) -> Result<Value, Box<dyn Error>> {
        match arguments.api {
            Api::Info(_) => self.info_command(config, arguments).await,
            Api::Index(a) => self.index_command(config, a).await,
            Api::Document(a) => self.document_command(config, a).await,
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
        let operation = arguments.operation;
        let index = arguments.index_name.expect("index name is required when operating with index commands");
        let resp;

        match operation {

            // create index
            Operation::Create => {
                let body = arguments.body.unwrap_or("{}".to_string());
                resp = client
                    .put(base_url + "/" + &index)
                    .body(body)
                    .header("Content-type", "application/json")
                    .send().await?
                    .json::<Value>().await;
            },

            // get index info
            Operation::Read => {
                resp = reqwest::get(base_url + "/" + &index).await?
                    .json::<Value>().await;
            },

            // update index
            Operation::Update => {
                unimplemented!("I am sorry: Index update is not yet implemented.")
            },

            // delete index
            Operation::Delete => {
                resp = client
                    .delete(base_url + "/" + &index)
                    .send().await?
                    .json::<Value>().await;
            },

        };

        match resp {
            Ok(v) => Ok(v),
            Err(e) => Err(Box::new(e))
        }


    }

    async fn document_command(&self, config: &Config, arguments: AddArgs) -> Result<Value, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let base_url = self.get_elasticsearch_baseurl(&config);
        let operation = arguments.operation;
        let index = arguments.index_name.expect("index name is required when operating with document commands");
        let resp;

        match operation {

            // index a document
            Operation::Create => {
                let body = arguments.body.expect("body is required when creating documents");
                resp = client
                    .post(base_url + "/" + &index + "/_doc")
                    .body(body)
                    .header("Content-type", "application/json")
                    .send().await?
                    .json::<Value>().await;
            },

            // search a document
            Operation::Read => {
                let body = arguments.body.unwrap_or("".to_string());
                resp = client
                    .post(base_url + "/" + &index + "/_search")
                    .body(body)
                    .header("Content-type", "application/json")
                    .send().await?
                    .json::<Value>().await;
            },

            // update a document
            Operation::Update => {
                let body = arguments.body.expect("body is required when updating documents");
                let id = arguments.id.expect("doc id is required when updating a document");
                resp = client
                    .post(base_url + "/" + &index + "/_update/" + &id)
                    .body(body)
                    .header("Content-type", "application/json")
                    .send().await?
                    .json::<Value>().await;
            },

            // delete a document
            Operation::Delete => {
                let type_name = arguments.type_name.unwrap_or("_doc".to_string());
                let id = arguments.id.expect("doc id is required when deleting a document");
                resp = client
                    .delete(base_url + "/" + &index + "/" + &type_name +  "/" + &id)
                    .header("Content-type", "application/json")
                    .send().await?
                    .json::<Value>().await;
            },


        };

        match resp {
            Ok(v) => Ok(v),
            Err(e) => Err(Box::new(e))
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::arguments::Operation::{Read};
    use crate::configuration::get_configuration;

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
                operation: Read,
                body: None,
                page: None,
                id: None,
                type_name: None,
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
                operation: Read,
                body: None,
                page: None,
                id: None,
                type_name: None,
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
                operation: Read,
                body: None,
                page: None,
                id: None,
                type_name: None,
            }
            ),
        };

        let conf = get_configuration(&arguments);


        let client = ElasticApiClient {};

        let base_url = client.get_elasticsearch_baseurl(&conf);
        assert_eq!(base_url, "http://127.0.0.1:9201");
    }
}