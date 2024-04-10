pub mod health;
pub mod user;

use actix_web::web::{self, ServiceConfig};
use tokio_postgres::NoTls;

pub fn configure_database(cfg: &mut ServiceConfig, postgres_config: deadpool_postgres::Config) {
    let pool = postgres_config.create_pool(None, NoTls).unwrap();

    cfg.app_data(web::Data::new(pool.clone()));
}