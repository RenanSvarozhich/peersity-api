use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::{ready, Ready, LocalBoxFuture};
use jsonwebtoken::{decode, DecodingKey, Validation};
use crate::models::auth::Claims; // Ensure this path matches the location of your Claims struct

pub struct Authorization;

impl<S, B> Transform<S, ServiceRequest> for Authorization
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthorizationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthorizationMiddleware { service }))
    }
}

pub struct AuthorizationMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthorizationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let auth_header = req.headers().get("Authorization");
        match auth_header {
            Some(header_value) => {
                if let Ok(header_str) = header_value.to_str() {
                    let parts: Vec<&str> = header_str.split_whitespace().collect();
                    if parts.len() == 2 && parts[0] == "Bearer" {
                        let token = parts[1];
                        if verify_token(token) {
                            let fut = self.service.call(req);
                            return Box::pin(async move {
                                let res = fut.await?;
                                Ok(res)
                            });
                        }
                    }
                }
                Box::pin(async {
                    Err(actix_web::error::ErrorUnauthorized("Invalid or missing token"))
                })
            },
            None => Box::pin(async {
                Err(actix_web::error::ErrorUnauthorized("Missing Authorization header"))
            }),
        }
    }
}

// Token verification function
fn verify_token(token: &str) -> bool {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    ).is_ok()
}