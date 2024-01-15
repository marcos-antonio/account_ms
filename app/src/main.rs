use actix_web::{web, App, HttpServer};
use client::rabbit_mq::RabbitMQExt;
use configure::AppConfig;
use dotenv::dotenv;
use error::AppResult;
use lapin::{
    options::{BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties,
};
use serde_json::json;
use state::AppState;

#[actix_web::main]
async fn main() -> AppResult<()> {
    dotenv().ok();
    let config = AppConfig::read()?;
    let app_state = AppState::new(config).await?;
    // let addr = std::env::var("AMQP_ADDR").unwrap_or_else(|_| "amqp://127.0.0.1:5672/%2f".into());

    // let conn = Connection::connect(&addr, ConnectionProperties::default())
    //     .await
    //     .unwrap();

    let channel_a = app_state.rabbit_mq.get_channel();

    channel_a
        .queue_declare(
            "transactions",
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    let jsontest = json!({
        "name": "test",
        "prop": "value"
    });

    channel_a
        .basic_publish(
            "",
            "transactions",
            BasicPublishOptions::default(),
            jsontest.to_string().as_bytes(),
            BasicProperties::default(),
        )
        .await
        .unwrap()
        .await
        .unwrap();

    let shared_date = web::Data::new(app_state);
    // let listener = TcpListener::bind(config.server.get_addr())?;

    HttpServer::new(move || {
        App::new()
            .app_data(shared_date.clone())
            .configure(router::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await?;

    Ok(())
}
