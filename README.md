
# Elasticli

<img alt="Logo" height="10%" src="logo.jpeg" width="10%"/>

[![Rust](https://github.com/giufus/elasticli.rs/actions/workflows/rust.yml/badge.svg)](https://github.com/giufus/elasticli.rs/actions/workflows/rust.yml)  

The missing Command Line Interface to interact with Elasticsearch (or yet another one).  
Even if elasticsearch APIs are super easy to invoke and very well documented, I always forget the basic commands and methods to do indexing or searches, so I decided to write my own CLI.  
I hope you find it useful, it is a work in progress.  
  
### Features  
Written in Rust 🦀, it uses the following crates:  
- [clap](https://crates.io/crates/clap) for arguments parsing  
- [hydroconf](https://crates.io/crates/hydroconf) for configuration management  
- [reqwest](https://crates.io/crates/reqwest) for http requests toward elasticsearch

If opportunely configured, the command can run inside a SSH tunnel (with auto close after a number of seconds).   

Currently, you can use `elasticli` to:    
- get info about the target elasticsearch version;  
- create, read, update, delete an index;  
- create, read, update, delete a document;  

### Changelog  
0.1.1: basic authentication   
0.1.0: first release  

### Build & Configuration
- Build it for your platform with `cargo build --release`, then go to `target/release` and run the binary `elasticli`. I would like to learn how to cross compile it in the future, if possible, to provide binaries for different platforms and architectures.      

- `cargo test` to run unit tests.  

- `cargo run -- <your options and command here>` to build and run from sources directly with the great `cargo`.  

- Every command can use the `hydroconf` features to override default configurations. For example you can override some defaults passing environment name as env var to the command line:  
`ENV_FOR_HYDRO=production elasticli info`  
or per single prop, specifying it as an env var to the command line:  
`HYDRO_ELASTIC__PASSWORD="an even stronger password" elasticli info`  

Look at the [hydroconf doc](https://github.com/rubik/hydroconf) for major details.    

- You pass the config root directory with `-c` option before one of the commands (info, index, document). The directory must contain the configuration files (look into the `default` folder for the latest version):  

[.secrets.toml](samples%2Fdefault%2F.secrets.toml)  environment based _sensitive_ configurations (tipically elastic user and password).  
```
[default]
elastic.username = 'elastic'
elastic.password = 'secure_password'

[production]
elastic.username = 'elastic'
elastic.password = 'changeme'
```

[settings.toml](samples%2Fdefault%2Fsettings.toml)  environment based configurations  

```
[default]
# your target elasticsearch host, port, protocol and version
elastic.host = 'otherhost'
elastic.port = 9200
elastic.protocol = 'http'
elastic.version = '8.8.0'

# if enabled, the main command will be executed inside an ssh tunnel. It is the same as running 
###  ssh -i <proxy.key> <proxy.remote_user>@<proxy.host> <elastic.port>:<elastic.host>:<elastic.port> sleep <proxy.timeout>
###  ssh -i .ssh/some_id_rsa centos@bastion-host 9200:remote-es.server.es:9200 sleep 3
# but rust do it for you
proxy.enabled = false
proxy.host = 'proxyhost'
proxy.port = 9201
proxy.protocol = 'http'
proxy.user = 'ec2-user'
proxy.remote_user = 'ec2-remote-user'
proxy.key = 'path to ssh key'
proxy.timeout = 3
```

## Commands Showcase
Instead of boring you while trying to describe how it works, here you find a few examples of commands (and sometimes the output). 
I hope it is understandable enough.

### General

#### - Get help about the command, options and subcommand   
`elasticli --help`   
`elasticli <info | index | document> --help`    

### Info

#### - Get basic info about elasticsearch  
`elasticli info`  

```
{
  "cluster_name": "docker-cluster",
  "cluster_uuid": "b9c-xKsSRs2HQAvZ8wGsIw",
  "name": "aecf011cca00",
  "tagline": "You Know, for Search",
  "version": {
    "build_date": "2023-05-23T17:16:07.179039820Z",
    "build_flavor": "default",
    "build_hash": "c01029875a091076ed42cdb3a41c10b1a9a5a20f",
    "build_snapshot": false,
    "build_type": "docker",
    "lucene_version": "9.6.0",
    "minimum_index_compatibility_version": "7.0.0",
    "minimum_wire_compatibility_version": "7.17.0",
    "number": "8.8.0"
  }
}
```

### Index

#### - Get index, same operation, multiple ways (it is Clap's magic)
`elasticli index -i test1`  
`elasticli index -i=test1`  
`elasticli index --index-name=test1`  
`elasticli index -o read --index-name=test1`  

#### - Get indexes ('_all' and '*' are Elasticsearch wildcards)
`elasticli index -i _all`  
`elasticli index --index-name='*'`  

#### - Create index  
`elasticli index -o create --index-name='pippo'`  
`elasticli index -o create --index-name='pippo_2' -b '{"settings": { "index": {  "number_of_shards": 3,  "number_of_replicas": 2  } } }'`  

```
{
  "acknowledged": true,
  "index": "pippo",
  "shards_acknowledged": true
}
```  

#### - Update Index    
```
NOT YET IMPLEMENTED
```  

#### - Delete Index      
`elasticli index -o delete -i 'pippo2'`  
`elasticli -c ./samples/default index -o delete --index-name='pippo2'`  


### Document

#### - Create a document in `test1`  
`elasticli document -o create -i test1 -b '{"name":"giufus", "language": "rust"}'`  

``` 
{
  "_id": "5Lms-YgBu6r1vXY7vPX_",
  "_index": "test1",
  "_primary_term": 3,
  "_seq_no": 2,
  "_shards": {
    "failed": 0,
    "successful": 1,
    "total": 2
  },
  "_version": 1,
  "result": "created"
}
```

#### - Update an existing document (you need to put your updates inside 'doc')
`elasticli document -o update -i test1 -b '{"doc": { "language": "zig"} }' --id 5Lms-YgBu6r1vXY7vPX_`  

``` 
{
  "_id": "5Lms-YgBu6r1vXY7vPX_",
  "_index": "test1",
  "_primary_term": 3,
  "_seq_no": 3,
  "_shards": {
    "failed": 0,
    "successful": 1,
    "total": 2
  },
  "_version": 2,
  "result": "updated"
}
```

#### - Search all docs in test1 index  
`elasticli document -o read -i test1`  

#### - Search all docs in test1 with an Elasticsearch query   
`elasticli document -o read -i test1 -b '{ "query": { "term": { "name": "giufus" } }}'`  

#### - Delete an existing document
`elasticli document -o delete -i test1 --id 5rm3-YgBu6r1vXY7S_Xb`

``` 
{
  "_id": "5Lms-YgBu6r1vXY7vPX_",
   "_index": "test1",
  "_primary_term": 3,
  "_seq_no": 9,
  "_shards": {
    "failed": 0,
    "successful": 1,
    "total": 2
  },
  "_version": 3,
  "result": "deleted"
}
```


### Defaults
There are some defaults in the code in case you don't specify them.  
- `http` as protocol  
- `127.0.0.1` as host  
- `9200` as port   
- `_doc` as type (used in document deletion)  
- `read` as operation  
- `8.8.0` as es version


### Todo  
- write a better documentation   
- handle elasticsearch versions (providing multiple implementations of the trait)  
- integration tests (maybe using something like [testcontainers](https://crates.io/crates/testcontainers))    


### Run elasticsearch locally with docker and no security  
`docker run -d --name elasticsearch -p 9200:9200 -p 9300:9300 -e "discovery.type=single-node" -e "xpack.security.enabled=false" elasticsearch:8.8.0`


### Run elasticsearch locally with docker and basic authentication  
`docker run -d --name elasticsearch -p 9200:9200 -p 9300:9300 -e "discovery.type=single-node" -e "xpack.security.enabled=true" -e "ELASTIC_PASSWORD=changeme" elasticsearch:8.8.0`



