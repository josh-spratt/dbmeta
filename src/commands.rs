use anyhow::Result;
use tokio_postgres::Client;

use crate::db::Backend;
use crate::models::{ColumnsResponse, SchemasResponse, TablesResponse};
use crate::postgres;

pub async fn schemas(client: &Client) -> Result<()> {
    let schemas = postgres::list_schemas(client).await?;
    let response = SchemasResponse {
        backend: Backend::Postgres.as_str().to_string(),
        schemas,
    };

    println!("{}", serde_json::to_string(&response)?);
    Ok(())
}

pub async fn tables(client: &Client, schema: &str) -> Result<()> {
    let tables = postgres::list_tables(client, schema).await?;
    let response = TablesResponse {
        backend: Backend::Postgres.as_str().to_string(),
        schema: schema.to_string(),
        tables,
    };

    println!("{}", serde_json::to_string(&response)?);
    Ok(())
}

pub async fn columns(client: &Client, schema: &str, table: &str) -> Result<()> {
    let columns = postgres::list_columns(client, schema, table).await?;
    let response = ColumnsResponse {
        backend: Backend::Postgres.as_str().to_string(),
        schema: schema.to_string(),
        table: table.to_string(),
        columns,
    };

    println!("{}", serde_json::to_string(&response)?);
    Ok(())
}
