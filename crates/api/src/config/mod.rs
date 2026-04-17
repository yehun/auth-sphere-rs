
mod application;
mod logging;
pub mod middle;
mod state;
pub use state::AppState;

use std::path::PathBuf;

pub use application::get as get_application;

pub fn init(config_path: PathBuf) -> Option<tracing_appender::non_blocking::WorkerGuard> {
    let config = application::init(config_path);
    let guard = logging::init(config.logging);
    guard
}
