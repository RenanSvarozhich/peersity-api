use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject
    pub exp: usize,  // Expiry
}

#[derive(Deserialize)]
pub struct AuthData {
    email: String,
    password: String,
}