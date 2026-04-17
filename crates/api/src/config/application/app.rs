use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct App {
    pub name: String,
}

impl Default for App {
    fn default() -> Self {
        App {
            name: "Auth-Sphere".to_string(),
        }
    }
}
