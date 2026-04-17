mod console;
mod file;

use crate::config::application::Logging;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

pub fn init(config: Option<Logging>) -> Option<WorkerGuard> {
    let console_layer = console::init(config.clone());
    let (file_layer, guard) = file::init(config.clone());
    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .init();
    guard
}
