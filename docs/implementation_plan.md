# dbmeta — MVP Implementation Plan (Thin Slice)

**Target Version:** v0.0.1
**Goal:** Implement the smallest useful version of `dbmeta` that can read metadata from **PostgreSQL** and output **machine-readable JSON**.

This plan is optimized for execution by an **AI coding agent**: steps are sequential, explicit, and minimize ambiguity.

---

# MVP Scope

## Included

* PostgreSQL backend
* Connection via DSN
* Commands:

  * `schemas`
  * `tables`
  * `columns`
* JSON output only
* Stable response structures

## Excluded (future work)

* Redshift
* MySQL
* SQLite
* `describe`
* `fks`
* `indexes`
* `search`
* filtering
* multiple output formats
* config files
* timeouts
* streaming output
* exit code mapping

---

# CLI Surface

```
dbmeta --dsn <dsn> schemas
dbmeta --dsn <dsn> tables --schema <schema>
dbmeta --dsn <dsn> columns <table> --schema <schema>
```

### Example

```
dbmeta \
  --dsn postgres://user:pass@localhost:5432/mydb \
  tables --schema public
```

Default schema:

```
public
```

---

# Expected JSON Output

These response shapes must remain **stable**.

## Schemas

```json
{
  "backend": "postgres",
  "schemas": [
    {
      "name": "public",
      "owner": "postgres"
    }
  ]
}
```

---

## Tables

```json
{
  "backend": "postgres",
  "schema": "public",
  "tables": [
    {
      "name": "orders",
      "type": "BASE TABLE"
    }
  ]
}
```

---

## Columns

```json
{
  "backend": "postgres",
  "schema": "public",
  "table": "orders",
  "columns": [
    {
      "name": "id",
      "ordinal": 1,
      "data_type": "integer",
      "nullable": false
    }
  ]
}
```

---

# Project Structure (Minimal)

```
dbmeta
│
├── Cargo.toml
└── src
    ├── main.rs
    ├── cli.rs
    ├── db.rs
    ├── postgres.rs
    ├── models.rs
    └── commands.rs
```

Target total code size:

```
600–800 lines
```

---

# Step-By-Step Implementation

---

# Step 1 — Initialize Project

Create Rust project.

```
cargo new dbmeta
cd dbmeta
```

---

# Step 2 — Add Dependencies

Edit `Cargo.toml`.

```toml
[package]
name = "dbmeta"
version = "0.0.1"
edition = "2021"

[dependencies]

clap = { version = "4", features = ["derive"] }

tokio = { version = "1", features = ["rt"] }

tokio-postgres = "0.7"

serde = { version = "1", features = ["derive"] }
serde_json = "1"

anyhow = "1"
```

Runtime requirement:

```
tokio current-thread runtime
```

---

# Step 3 — Implement CLI

Create file:

```
src/cli.rs
```

CLI structure:

```
dbmeta
 ├── schemas
 ├── tables
 └── columns
```

Example implementation:

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[arg(long)]
    pub dsn: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
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
```

---

# Step 4 — Define Data Models

Create:

```
src/models.rs
```

## Schema

```rust
#[derive(Serialize)]
pub struct Schema {
    pub name: String,
    pub owner: String,
}
```

---

## Table

```rust
#[derive(Serialize)]
pub struct Table {
    pub name: String,
    pub r#type: String,
}
```

---

## Column

```rust
#[derive(Serialize)]
pub struct Column {
    pub name: String,
    pub ordinal: i32,
    pub data_type: String,
    pub nullable: bool,
}
```

---

## Response Wrappers

```rust
#[derive(Serialize)]
pub struct SchemasResponse {
    pub backend: String,
    pub schemas: Vec<Schema>,
}

#[derive(Serialize)]
pub struct TablesResponse {
    pub backend: String,
    pub schema: String,
    pub tables: Vec<Table>,
}

#[derive(Serialize)]
pub struct ColumnsResponse {
    pub backend: String,
    pub schema: String,
    pub table: String,
    pub columns: Vec<Column>,
}
```

---

# Step 5 — Database Connection

Create:

```
src/db.rs
```

Purpose:

```
Create a tokio-postgres client from a DSN
```

Example:

```rust
use tokio_postgres::{Client, NoTls};
use anyhow::Result;

pub async fn connect(dsn: &str) -> Result<Client> {
    let (client, connection) =
        tokio_postgres::connect(dsn, NoTls).await?;

    tokio::spawn(async move {
        let _ = connection.await;
    });

    Ok(client)
}
```

---

# Step 6 — Implement Postgres Metadata Queries

Create:

```
src/postgres.rs
```

Functions:

```
list_schemas
list_tables
list_columns
```

---

## Query — Schemas

```sql
SELECT
  schema_name,
  schema_owner
FROM information_schema.schemata
ORDER BY schema_name
```

---

## Query — Tables

```sql
SELECT
  table_name,
  table_type
FROM information_schema.tables
WHERE table_schema = $1
ORDER BY table_name
```

---

## Query — Columns

```sql
SELECT
  column_name,
  ordinal_position,
  data_type,
  is_nullable
FROM information_schema.columns
WHERE table_schema = $1
AND table_name = $2
ORDER BY ordinal_position
```

---

## Mapping Rows → Models

Example:

```rust
Column {
    name: row.get(0),
    ordinal: row.get(1),
    data_type: row.get(2),
    nullable: row.get::<_, String>(3) == "YES",
}
```

---

# Step 7 — Command Execution Layer

Create:

```
src/commands.rs
```

Responsibilities:

* call postgres query functions
* construct response structs
* serialize JSON
* print to stdout

Example:

```rust
pub async fn schemas(client: &Client) -> Result<()> {
    let schemas = postgres::list_schemas(client).await?;

    let response = SchemasResponse {
        backend: "postgres".to_string(),
        schemas,
    };

    println!("{}", serde_json::to_string(&response)?);

    Ok(())
}
```

---

# Step 8 — Wire Everything Together

Create:

```
src/main.rs
```

Runtime:

```rust
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
```

Execution flow:

```
parse CLI
connect database
match command
execute command
print JSON
```

Example:

```rust
let cli = Cli::parse();

let client = db::connect(&cli.dsn).await?;

match cli.command {
    Commands::Schemas => commands::schemas(&client).await?,
    Commands::Tables { schema } =>
        commands::tables(&client, &schema).await?,
    Commands::Columns { table, schema } =>
        commands::columns(&client, &schema, &table).await?,
}
```

---

# Step 9 — Manual Testing

Start a local Postgres instance:

```
docker run -p 5432:5432 \
  -e POSTGRES_PASSWORD=pass \
  postgres
```

Run the CLI:

```
cargo run -- \
  --dsn postgres://postgres:pass@localhost:5432/postgres \
  schemas
```

Expected output:

```
{"backend":"postgres","schemas":[...]}
```

---

# Step 10 — Prepare for Redshift Extension

Before completing MVP, add a minimal backend enum.

Future file:

```
src/db.rs
```

Example:

```rust
enum Backend {
    Postgres
}
```

Later extension:

```
Backend::Postgres
Backend::Redshift
```

Detection logic (future):

```
postgres://
redshift://
```

Redshift will reuse the **same driver (`tokio-postgres`)** with different catalog queries.

This ensures the CLI interface remains **unchanged** when Redshift support is added.

---

# Release

Tag the first version.

```
git tag v0.0.1
```

---

# Next Thin Slice (v0.0.2)

Add:

```
describe
foreign keys
indexes
```

These can be implemented using the same Postgres connection without architectural changes.

---

# Redshift Support (v0.1)

Add:

```
src/redshift.rs
```

Override catalog queries to surface:

```
diststyle
distkey
sortkey
```

Expose these values through:

```
engine_extras
```

in the JSON output.

---

# Success Criteria

The MVP is complete when:

* `dbmeta schemas` returns valid JSON
* `dbmeta tables` returns tables for a schema
* `dbmeta columns` returns column metadata
* codebase is under **800 lines**
* CLI is deterministic and non-interactive
* architecture allows **Redshift to be added without CLI changes**
