# dbmeta
A command-line tool for reading schema, table, and column metadata from relational databases.

## MVP (v0.0.1)

PostgreSQL only. Outputs machine-readable JSON to stdout.

## Usage

```bash
dbmeta --dsn <dsn> schemas
dbmeta --dsn <dsn> tables --schema <schema>
dbmeta --dsn <dsn> columns <table> --schema <schema>
```

Default schema is `public`.

### Environment variable

You can avoid passing `--dsn` every time by setting `DBMETA_DSN`:

```bash
export DBMETA_DSN='postgres://postgres:pass@localhost:5432/postgres'
dbmeta schemas
dbmeta tables --schema public
dbmeta columns orders --schema public
```

## Quick test with Docker (Postgres)

Start Postgres:

```bash
docker run --rm --name dbmeta-postgres \
  -e POSTGRES_PASSWORD=pass \
  -p 5432:5432 \
  postgres:16
```

In another terminal, run:

```bash
cargo run -- --dsn postgres://postgres:pass@localhost:5432/postgres schemas
```

Optional: create a table so `tables`/`columns` have output:

```bash
docker exec -i dbmeta-postgres psql -U postgres -d postgres <<'SQL'
CREATE TABLE IF NOT EXISTS public.orders (
  id integer PRIMARY KEY,
  created_at timestamptz NOT NULL DEFAULT now()
);

COMMENT ON TABLE public.orders IS 'Orders placed by customers';
COMMENT ON COLUMN public.orders.id IS 'Primary key';
SQL
```

Then:

```bash
cargo run -- --dsn postgres://postgres:pass@localhost:5432/postgres tables --schema public
cargo run -- --dsn postgres://postgres:pass@localhost:5432/postgres columns orders --schema public
```

### Example

```bash
dbmeta \
  --dsn postgres://user:pass@localhost:5432/mydb \
  tables --schema public
```

## Output shapes (stable)

### Schemas

```json
{
  "backend": "postgres",
  "schemas": [
    { "name": "public", "owner": "postgres" }
  ]
}
```

### Tables

```json
{
  "backend": "postgres",
  "schema": "public",
  "tables": [
    { "name": "orders", "type": "BASE TABLE", "comment": "…" }
  ]
}
```

### Columns

```json
{
  "backend": "postgres",
  "schema": "public",
  "table": "orders",
  "columns": [
    { "name": "id", "ordinal": 1, "data_type": "integer", "nullable": false, "comment": "…" }
  ]
}
```
