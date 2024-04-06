use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
pub struct EnvConfig {
    pub server_address: String,
}