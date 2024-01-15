use async_trait::async_trait;
use configure::postgres::PostgresConfig;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub type PgClient = PgPool;

#[async_trait]
pub trait PgPoolExt: Sized {
    async fn new(config: &PostgresConfig) -> Result<Self, sqlx::Error>;
}

#[async_trait]
impl PgPoolExt for PgPool {
    async fn new(config: &PostgresConfig) -> Result<Self, sqlx::Error> {
        PgPoolOptions::new()
            .max_connections(config.max_connections)
            .connect(&config.get_url())
            .await
    }
}
