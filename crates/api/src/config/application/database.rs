use std::fmt::Display;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Database {
    pub path: String,
    pub max_connections: u32
}

impl Default for Database {
    fn default() -> Self {
        Self {
            path: String::from("auth-sphere.db"),
            max_connections: 10
        }
    }
}

impl Display for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "database.path: {}", self.path)
    }
}