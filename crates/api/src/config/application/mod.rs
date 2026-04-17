mod config;
mod server;
mod logging;
mod database;
mod redis;
mod app;

pub use config::ApplicationConfig;
pub use logging::Logging;

use std::path::PathBuf;
use std::sync::Mutex;
use once_cell::sync::Lazy;
use tracing::debug;

static CONFIG: Lazy<Mutex<ApplicationConfig>> = Lazy::new(|| Mutex::new(ApplicationConfig::default()));

fn init_with_file(file_path: PathBuf) -> ApplicationConfig {
    ApplicationConfig::init(file_path)
        .unwrap_or_else(|e| panic!("Failed to initialize application config: {}", e))
}

pub fn init(file_path: PathBuf) -> ApplicationConfig {
    let config = init_with_file(file_path);

    println!("{}", config);

    // Update global config
    *CONFIG.lock().unwrap() = config.clone();

    debug!("[done] config initial");
    get()
}

pub fn get() -> ApplicationConfig {
    CONFIG.lock().unwrap().clone()
}
