
use std::process::{Child, Command, ExitStatus};

use crate::{arguments::Arguments, configuration::Config};
use crate::client::{ElasticApi, ElasticApiClient};
use crate::configuration::{get_configuration};

mod arguments;
mod configuration;
mod client;


#[tokio::main]
async fn main() {

    let arguments: Arguments = arguments::parse_arguments();

    let conf: Config = get_configuration(&arguments);

    let elastic_api: ElasticApiClient = ElasticApiClient {};

    // run tunnel eventually
    let _ssh_process: (Option<Child>, Option<ExitStatus>) = match &conf.proxy {
        Some(x) => {
            match x.enabled {
                true => {
                    let mut child = Command::new("ssh")
                        .args(["-i", format!("{}", x.key).as_str()])
                        .arg(format!("{}@{}", x.remote_user, x.host).as_str())
                        .arg("-f")
                        .arg("-L")
                        .arg(format!("{}:{}:{}", conf.elastic.port, conf.elastic.host, conf.elastic.port).as_str())
                        .arg(format!("sleep {};", x.timeout).as_str()) //autoclose tunnel in x sec;
                        .spawn().expect("ssh command panicked");

                    let exit_status = child.wait().expect("ssh command exit status panicked");
                    (Some(child), Some(exit_status))
                }
                false => (None, None)
            }
        }
        None => (None, None)
    };


    let result = elastic_api.execute_command(&conf, arguments).await;
    println!("{:#}", result.unwrap());

}