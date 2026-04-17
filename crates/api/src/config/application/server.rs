use std::fmt::Display;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Server {
    pub server: String,
    pub port: Option<u16>,
}

impl Server {
    pub fn get_addr(&self) -> String {
        let port = self.port.unwrap_or(8080);
        format!("{}:{}", self.server, port)
    }
}

impl Default for Server {
    fn default() -> Self {
        Self {
            server: "0.0.0.0".into(),
            port: Some(8080),
        }
    }
}

impl Display for Server {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "server.host: {}\n", &self.server)?;
        write!(f, "server.port: {}", &self.port.unwrap_or(8080u16))
    }
}
