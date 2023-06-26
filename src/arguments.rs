
use clap::{Parser, Subcommand, Args};
use clap::builder::TypedValueParser;


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(term_width = 0)]
pub struct Arguments {
    /// dir containing settings.toml / .secrets.toml
    #[arg(short, long, value_name = "DIRECTORY", )]
    pub config: Option<String>,

    #[command(subcommand)]
    pub api: Api,

}

#[derive(Subcommand, Debug)]
pub enum Api {
    /// info about elasticsearch: only GET (it is the default) method is enabled
    Info(AddArgs),
    /// GET retrieve index info, PUT create index, POST insert document into index, OPTIONS update a document, DELETE remove index
    Index(AddArgs),
    /// search documents operations: only POST method is enabled
    Search(AddArgs),
}

#[derive(Debug, Args)]
pub struct AddArgs {
    /// index name or alias
    pub index_name: Option<String>,

    /// json body of the request
    pub body: Option<String>,

    /// id of a document
    pub id: Option<String>,

    #[arg(
    long,
    short,
    default_value_t = Method::Get,
    value_parser = clap::builder::PossibleValuesParser::new(["get", "post", "put", "delete", "options"])
    .map(| s | s.parse::< Method > ().unwrap()),
    )]
    pub method: Method,

    ///  pagination
    pub page: Option<u16>,

}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Options,
}

impl std::fmt::Display for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Get => "get",
            Self::Post => "post",
            Self::Put => "put",
            Self::Delete => "delete",
            Self::Options => "options",
        };
        s.fmt(f)
    }
}

impl std::str::FromStr for Method {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "get" => Ok(Self::Get),
            "post" => Ok(Self::Post),
            "put" => Ok(Self::Put),
            "delete" => Ok(Self::Delete),
            "options" => Ok(Self::Options),
            _ => Err(format!("Unknown method: {s}")),
        }
    }
}


pub fn parse_arguments() -> Arguments {
    Arguments::parse()
}


#[cfg(test)]
mod tests {
    
}