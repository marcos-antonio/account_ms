use async_trait::async_trait;
use configure::rabbit_mq::RabbitMQConfig;
use error::AppResult;
use lapin::{Channel, Connection, ConnectionProperties};

pub struct RabbitMQClient {
    connection: Connection,
    current_channel: lapin::Channel,
}

#[async_trait]
pub trait RabbitMQExt: Sized {
    async fn new(config: &RabbitMQConfig) -> AppResult<RabbitMQClient>;
    async fn create_channel(self) -> AppResult<Channel>;
    fn get_channel(&self) -> &Channel;
}

#[async_trait]
impl RabbitMQExt for RabbitMQClient {
    async fn new(config: &RabbitMQConfig) -> AppResult<RabbitMQClient> {
        let con = Connection::connect(&config.get_url(), ConnectionProperties::default()).await?;
        let channel = con.create_channel().await?;
        Ok(RabbitMQClient {
            connection: con,
            current_channel: channel,
        })
    }

    async fn create_channel(self) -> AppResult<Channel> {
        Ok(self.connection.create_channel().await?)
    }

    fn get_channel(&self) -> &Channel {
        &self.current_channel
    }
}
