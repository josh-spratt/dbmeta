use anyhow::Result;
use tokio_postgres::Client;

use crate::models::{Column, Schema, Table};

pub async fn list_schemas(client: &Client) -> Result<Vec<Schema>> {
    let rows = client
        .query(
            r#"
SELECT
  schema_name,
  schema_owner
FROM information_schema.schemata
ORDER BY schema_name
"#,
            &[],
        )
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| Schema {
            name: row.get(0),
            owner: row.get(1),
        })
        .collect())
}

pub async fn list_tables(client: &Client, schema: &str) -> Result<Vec<Table>> {
    let rows = client
        .query(
            r#"
SELECT
  t.table_name,
  t.table_type,
  obj_description((quote_ident(t.table_schema) || '.' || quote_ident(t.table_name))::regclass, 'pg_class') AS comment
FROM information_schema.tables t
WHERE t.table_schema = $1
ORDER BY table_name
"#,
            &[&schema],
        )
        .await?;

    Ok(rows
        .into_iter()
        .map(|row| Table {
            name: row.get(0),
            r#type: row.get(1),
            comment: row.get(2),
        })
        .collect())
}

pub async fn list_columns(client: &Client, schema: &str, table: &str) -> Result<Vec<Column>> {
    let rows = client
        .query(
            r#"
SELECT
  c.column_name,
  c.ordinal_position,
  c.data_type,
  c.is_nullable,
  pgd.description AS comment
FROM information_schema.columns c
JOIN pg_catalog.pg_class pc
  ON pc.oid = (quote_ident(c.table_schema) || '.' || quote_ident(c.table_name))::regclass
JOIN pg_catalog.pg_attribute pa
  ON pa.attrelid = pc.oid
 AND pa.attname = c.column_name
 AND pa.attnum > 0
 AND NOT pa.attisdropped
LEFT JOIN pg_catalog.pg_description pgd
  ON pgd.objoid = pc.oid
 AND pgd.objsubid = pa.attnum
WHERE c.table_schema = $1
AND c.table_name = $2
ORDER BY ordinal_position
"#,
            &[&schema, &table],
        )
        .await?;

    let columns: Vec<Column> = rows
        .into_iter()
        .map(|row| Column {
            name: row.get(0),
            ordinal: row.get(1),
            data_type: row.get(2),
            nullable: row.get::<_, String>(3) == "YES",
            comment: row.get(4),
        })
        .collect();

    if columns.is_empty() {
        let exists = client
            .query_opt(
                r#"
SELECT 1
FROM information_schema.tables
WHERE table_schema = $1
AND table_name = $2
LIMIT 1
"#,
                &[&schema, &table],
            )
            .await?
            .is_some();

        if !exists {
            anyhow::bail!("table not found: {}.{}", schema, table);
        }
    }

    Ok(columns)
}
