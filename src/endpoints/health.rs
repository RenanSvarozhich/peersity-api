use actix_web::{get, Responder, web};
use crate::models::health::HealthCheck;
use actix_web::web::ServiceConfig;
use crate::database::health::get_database_health;
use deadpool_postgres::Pool;

pub fn configure_endpoints(cfg: &mut ServiceConfig) {
    cfg.service(get_health);
}

#[get("/health")]
async fn get_health(db_pool: web::Data<Pool>) -> impl Responder {
    let database_health = match db_pool.get().await {
        Ok(client) => {
            match get_database_health(&client).await {
                Ok(health) => health,
                Err(_) => false,
            }
        },
        Err(_) => false,
    };

    let health_check = HealthCheck { 
        endpoint: true,
        database: database_health,
    };

    actix_web::web::Json(health_check)
}