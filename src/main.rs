mod cli;
mod commands;
mod db;
mod models;
mod postgres;

use anyhow::Result;
use clap::Parser;

use crate::cli::{Cli, Commands};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let client = db::connect(&cli.dsn).await?;

    match cli.command {
        Commands::Schemas => commands::schemas(&client).await?,
        Commands::Tables { schema } => commands::tables(&client, &schema).await?,
        Commands::Columns { table, schema } => commands::columns(&client, &schema, &table).await?,
    }

    Ok(())
}
