
pub mod trace;

use trace::LoggerTraceBuilder;
use tracing_actix_web::TracingLogger;

pub fn get_config() -> TracingLogger<LoggerTraceBuilder> {
    TracingLogger::<LoggerTraceBuilder>::new()
}
