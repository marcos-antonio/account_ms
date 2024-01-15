use client::{
    postgres::{PgClient, PgPoolExt},
    rabbit_mq::{RabbitMQClient, RabbitMQExt},
};
use configure::AppConfig;

pub struct AppState {
    pub db: PgClient,
    pub rabbit_mq: RabbitMQClient,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self, std::io::Error> {
        let postgres = PgClient::new(&config.postgres).await.unwrap();
        let rabbit_mq = RabbitMQClient::new(&config.rabbit_mq).await.unwrap();
        Ok(Self {
            db: postgres,
            rabbit_mq,
        })
    }
}
