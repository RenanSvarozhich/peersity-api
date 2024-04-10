use actix_web::web::ServiceConfig;
use actix_web::{post, web, HttpResponse, Responder};
use deadpool_postgres::Pool;
use crate::models::auth::{AuthData, Claims};
use std::env;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, errors::Result as JwtResult};
use chrono::{Utc, Duration};

pub fn configure_endpoints(cfg: &mut ServiceConfig) {
    cfg.service(post_login);
}

#[post("/auth/login")]
async fn post_login(_auth_data: web::Json<AuthData>, _db_pool: web::Data<Pool>) -> impl Responder {
    // Here you should add logic to verify the email and password with the database
    // For demonstration, let's assume the user is authenticated successfully

    let user_id = "some_user_id"; // Replace with actual user ID from DB
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    match create_token(user_id, secret.as_bytes()) {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

fn create_token(user_id: &str, secret: &[u8]) -> JwtResult<String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(60))
        .expect("valid timestamp")
        .timestamp();
    
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration as usize,
    };
    p
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret))
}