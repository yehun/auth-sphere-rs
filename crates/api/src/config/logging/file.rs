use crate::config::application::Logging;
use tracing::level_filters::LevelFilter;
use tracing::Subscriber;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::{fmt, Layer};

pub(crate) fn init<S>(config: Option<Logging>) -> (
    Box<dyn Layer<S> + Send + Sync>,
    Option<WorkerGuard>
)
    where
        S: Subscriber + Send + Sync + for<'a> LookupSpan<'a>
{
    let logger_config = config.unwrap_or(Logging::default());
    match &logger_config.file {
        Some(file_config) => {
            let file_appender = tracing_appender::rolling::daily(
                file_config.path.as_path(),
                file_config.prefix.clone(),
            );
            let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);

            let level = logger_config.get_level_with_file();

            let layer = fmt::layer()
                .with_ansi(false)
                .with_writer(non_blocking)
                .with_timer(OffsetTime::local_rfc_3339().expect("could not get local offset!"))
                // .with_timer(UtcOffset::from_hms(8, 0, 0).unwrap())
                .with_level(true)
                .with_filter(LevelFilter::from_level(level));
            (Box::new(layer), Some(guard))
        },
        None => {
            let layer = fmt::layer().with_filter(LevelFilter::OFF);
            (Box::new(layer), None)
        },
    }
}