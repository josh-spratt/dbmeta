use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "dbmeta", version, about = "Database metadata as JSON")]
pub struct Cli {
    #[arg(long, env = "DBMETA_DSN")]
    pub dsn: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Schemas,

    Tables {
        #[arg(long, default_value = "public")]
        schema: String,
    },

    Columns {
        table: String,

        #[arg(long, default_value = "public")]
        schema: String,
    },
}
