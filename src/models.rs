use serde::Serialize;

#[derive(Serialize)]
pub struct Schema {
    pub name: String,
    pub owner: String,
}

#[derive(Serialize)]
pub struct Table {
    pub name: String,
    pub r#type: String,
    pub comment: Option<String>,
}

#[derive(Serialize)]
pub struct Column {
    pub name: String,
    pub ordinal: i32,
    pub data_type: String,
    pub nullable: bool,
    pub comment: Option<String>,
}

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
