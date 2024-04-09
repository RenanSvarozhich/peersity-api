use actix_web::{get, Error, web::{self, ServiceConfig}, HttpResponse};
use deadpool_postgres::{Client, Pool};
use crate::{database, models::error::DBError};
use crate::middleware::auth::Authorization;

pub fn configure_endpoints(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("")
        .wrap(Authorization)
        .service(get_users));
}

#[get("/user")]
pub async fn get_users(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let client: Client = db_pool.get().await.map_err(DBError::PoolError)?;

    let users = database::user::get_users(&client).await?;

    Ok(HttpResponse::Ok().json(users))
}