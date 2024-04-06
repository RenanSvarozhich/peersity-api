use actix_web::{get, Responder};
use crate::models::health::HealthCheck;

#[get("/health")]
pub async fn get_health() -> impl Responder { 
    let health_check = HealthCheck { 
        endpoint: true 
    };

    return actix_web::web::Json(health_check);
}