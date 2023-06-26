
## Elasticli 🦀
Yet another Command Line Interface to interact with Elasticsearch (and another of my experiments in the journey to learn Rust language).  
Even if elasticsearch APIs are super easy to invoke and very well documented, I always forget the basic commands and methods to do indexing or searches, so I decided to write my own CLI.  
I hope you find it useful, it is a work in progress.  
  
### Features  
Written in Rust 🦀, it uses the following crates:  
- [clap](https://crates.io/crates/clap) for arguments parsing  
- [hydroconf](https://crates.io/crates/hydroconf) for configuration management  
- [reqwest](https://crates.io/crates/reqwest) for http requests toward elasticsearch

If configured, the command can open a SSH tunnel, invoke the api then close the tunnel.   

Currently, you can use `elasticli` to:    
- get info about the target elasticsearch version;  
- create, update, delete an index;  
- create, update a document;  
- search documents;  

### Build & Configuration
- Build it for your platform with `cargo build --release`, then go to `target/release` and run the binary `elasticli`. I would like to learn how to cross compile it, in the future.      

- `cargo test` to run unit tests.  

- `cargo run -- <your options and command here>` to run it directly from cargo.  

- Every command can use the `hydroconf` features to override default configurations.  
- For example you can override some defaults passing environment name as env var:  
`ENV_FOR_HYDRO=production elasticli info`  
- Look at the [hydroconf doc](https://github.com/rubik/hydroconf) for major details.    

- I like to pass the config root directory with `-c` option, for example. The directory must contain the configuration files:  

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
# ssh -i <proxy.key> <proxy.remote_user>@<proxy.host> <elastic.port>:<elastic.host>:<elastic.port> sleep <proxy.timeout>
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

Look at the samples in the `default` dir of the project for an overview of the possible values. 

### Showcases
Instead of boring you while trying to describe how it works, here you find a few examples of commands (and sometimes the output).  

##### - Get help about the command, options and subcommand    
`elasticli --help`   
`elasticli info | index | search --help`    

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

#### - Get info about `_all` known indexes (found `test1` index)  
`elasticli index`

```
{
  "test1": {
    "aliases": {},
    "mappings": {
      "properties": {
        "language": {
          "fields": {
            "keyword": {
              "ignore_above": 256,
              "type": "keyword"
            }
          },
          "type": "text"
        },
        "name": {
          "fields": {
            "keyword": {
              "ignore_above": 256,
              "type": "keyword"
            }
          },
          "type": "text"
        }
      }
    },
    "settings": {
      "index": {
        "creation_date": "1687816701969",
        "number_of_replicas": "1",
        "number_of_shards": "1",
        "provided_name": "test1",
        "routing": {
          "allocation": {
            "include": {
              "_tier_preference": "data_content"
            }
          }
        },
        "uuid": "VjhEfYL9QDuJiwzFcuKDTQ",
        "version": {
          "created": "8080099"
        }
      }
    }
  }
}
```

#### - Get info about `test1` index
`elasticli index test1`

#### - Create Index `test2` with no other settings (`BODY` argument is empty)  
`elasticli index -m put test2 ""`

```
{
  "acknowledged": true,
  "index": "test2",
  "shards_acknowledged": true
}
```  

#### - Delete Index `test2`    
`elasticli index -m delete test2`  

#### - Index a document into `test1`  
`elasticli index -m post test1 '{"name":"giufus", "language": "rust"}'`  

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

#### - Update an existing document  
`elasticli index -m options test1 '{"doc": { "language": "zig"} }' 5Lms-YgBu6r1vXY7vPX_`  

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

#### - Search documents (no query, just give me some hits)  
`elasticli search test1 -m post "{}"` 

```                               
{
  "_shards": {
    "failed": 0,
    "skipped": 0,
    "successful": 1,
    "total": 1
  },
  "hits": {
    "hits": [
      {
        "_id": "5Lms-YgBu6r1vXY7vPX_",
        "_index": "test1",
        "_score": 1.0,
        "_source": {
          "language": "zig",
          "name": "giufus"
        }
      }
    ],
    "max_score": 1.0,
    "total": {
      "relation": "eq",
      "value": 1
    }
  },
  "timed_out": false,
  "took": 8
}
```

The `BODY` argument can be any valid elasticsearch DSL json. 


### Defaults
There are some defaults in the code in case you don't specify them.  
- http  
- 127.0.0.1  
- 9200  
- _all  
- GET


### Todo  
- change `Method` enum to something more understandable (maybe something like classic actions of CRUD APIs)    
- write a better documentation   
- handle elasticsearch authentication (probably it can be done in reqwest)   
- handle elasticsearch versions (providing multiple implementations of the trait)  
- integration tests (maybe using something like [testcontainers](https://crates.io/crates/testcontainers))    


### Run elasticsearch locally with docker and no security  
`docker run -d --name elasticsearch -p 9200:9200 -p 9300:9300 -e "discovery.type=single-node" -e "xpack.security.enabled=false" elasticsearch:8.8.0`



