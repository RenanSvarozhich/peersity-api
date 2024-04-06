use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)] 
pub struct HealthCheck { 
    pub endpoint: bool,
    pub database: bool,
}