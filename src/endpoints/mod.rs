mod health;
mod user;

use actix_web::web::ServiceConfig;

pub fn configure_endpoints(cfg: &mut ServiceConfig) {
    cfg.configure(health::configure_endpoints);
    cfg.configure(user::configure_endpoints);
}