use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct RabbitMQConfig {
    pub host: String,
    pub port: u16,
    pub v_host: String,
}

impl RabbitMQConfig {
    pub fn get_url(&self) -> String {
        Self::create_url(&self.host, self.port, &self.v_host)
    }

    pub fn create_url(host: &str, port: u16, v_host: &str) -> String {
        format!("amqp://{host}:{port}/{v_host}")
    }
}
