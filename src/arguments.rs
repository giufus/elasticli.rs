use clap::{Args, Parser, Subcommand};
use clap::builder::TypedValueParser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(term_width = 0)]
pub struct Arguments {
    /// dir containing settings.toml  .secrets.toml
    #[arg(short, long, value_name = "DIRECTORY", )]
    pub config: Option<String>,

    /// the command name
    #[command(subcommand)]
    pub api: Api,

}

#[derive(Subcommand, Debug)]
pub enum Api {
    /// info commands
    Info(AddArgs),
    /// index commands
    Index(AddArgs),
    /// document commands
    Document(AddArgs),
}

#[derive(Debug, Args)]
pub struct AddArgs {


    /// operation to execute
    #[arg(
    long,
    short,
    default_value_t = Operation::Read,
    value_parser = clap::builder::PossibleValuesParser::new(["create", "read", "update", "delete"])
    .map(| s | s.parse::< Operation > ().unwrap()),
    )]
    pub operation: Operation,

    /// index name or alias
    #[arg(short, long, )]
    pub index_name: Option<String>,

    /// json body of the request
    #[arg(short, long,)]
    pub body: Option<String>,

    /// type of a document
    #[arg(short, long,)]
    pub type_name: Option<String>,

    /// id of a document
    #[arg(long,)]
    pub id: Option<String>,

    ///  page size
    #[arg(short, long,)]
    pub page: Option<u16>,

}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Operation {
    Create,
    Read,
    Update,
    Delete,
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Create => "create",
            Self::Read => "read",
            Self::Update => "update",
            Self::Delete => "delete",
        };
        s.fmt(f)
    }
}

impl std::str::FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "create" => Ok(Self::Create),
            "read" => Ok(Self::Read),
            "update" => Ok(Self::Update),
            "delete" => Ok(Self::Delete),
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