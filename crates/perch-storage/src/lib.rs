use sqlx::postgres::{PgPool, PgPoolOptions};
use thiserror::Error;
use url::Url;

#[derive(Debug, Clone)]
pub struct Database {
    pool: PgPool,
}

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("database configuration failed: {0}")]
    Configuration(#[from] sqlx::Error),
    #[error("database readiness check failed: {0}")]
    Readiness(#[source] sqlx::Error),
}

impl Database {
    pub fn new(url: &Url) -> Result<Self, StorageError> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect_lazy(url.as_str())?;

        Ok(Self { pool })
    }

    pub async fn ready(&self) -> Result<(), StorageError> {
        sqlx::query("select 1")
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(StorageError::Readiness)
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
