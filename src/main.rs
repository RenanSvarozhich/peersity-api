pub mod models;
pub mod database;
pub mod middleware;
mod endpoints;

use ::config::Config;
use actix_web::{App, HttpServer};
use dotenvy::dotenv;
use crate::models::config::EnvConfig;
use crate::endpoints::configure_endpoints;
use crate::database::configure_database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config: EnvConfig = Config::builder()
        .add_source(::config::Environment::default())
        .build().unwrap()
        .try_deserialize().unwrap();

    HttpServer::new(move || {
        App::new()
            .configure(|cfg| configure_database(cfg, config.pg.clone()))            
            .configure(configure_endpoints)
    })
    .bind(config.server_address.clone()).expect("Address should be free and valid")
    .run().await
}