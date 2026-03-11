use anyhow::Result;
use tokio_postgres::{Client, NoTls};

#[derive(Debug, Clone, Copy)]
pub enum Backend {
    Postgres,
}

impl Backend {
    pub fn as_str(self) -> &'static str {
        match self {
            Backend::Postgres => "postgres",
        }
    }
}

pub async fn connect(dsn: &str) -> Result<Client> {
    let (client, connection) = tokio_postgres::connect(dsn, NoTls).await?;

    tokio::spawn(async move {
        let _ = connection.await;
    });

    Ok(client)
}
