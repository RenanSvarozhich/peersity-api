pub mod models;
mod endpoints;

use ::config::Config;
use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use crate::models::config::EnvConfig;
use crate::endpoints::health::get_health;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build().unwrap();

    let config: EnvConfig = config_.try_deserialize().unwrap();

    return HttpServer::new(move || {
        App::new()
        .service(get_health)
    })
    .bind(config.server_address.clone()).expect("Address should be free and valid")
    .run().await;
}