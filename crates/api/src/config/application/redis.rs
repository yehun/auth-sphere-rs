use std::fmt::Display;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Redis {
    pub host: String,
    pub port: Option<u16>,
    pub auth: Option<String>,
    pub db: Option<u8>,
    pub timeout: Option<f32>,
}

impl Default for Redis {
    fn default() -> Self {
        Self {
            host: String::from("127.0.0.1"),
            port: Some(6379),
            auth: None,
            db: Some(0),
            timeout: Some(3f32),
        }
    }
}

impl Display for Redis {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "redis.host: {}\n", &self.host)?;
        write!(f, "redis.port: {}\n", &self.port.unwrap_or(6379))?;
        write!(f, "redis.auth: {}\n", self.auth.clone().unwrap_or("".to_string()))?;
        write!(f, "redis.db: {}\n", &self.db.unwrap_or(0))?;
        write!(f, "redis.timeout: {}", &self.timeout.unwrap_or(3f32))
    }
}