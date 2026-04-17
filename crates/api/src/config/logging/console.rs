use crate::config::application::Logging;
use tracing::level_filters::LevelFilter;
use tracing::{Level, Subscriber};
use tracing_subscriber::fmt::time::OffsetTime;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::{fmt, Layer};

pub(crate) fn init<S>(config: Option<Logging>) -> Box<dyn Layer<S> + Send + Sync>
    where
        S: Subscriber + Send + Sync + for<'a> LookupSpan<'a>
{
    let level = config.map(|c| c.get_level_with_console()).unwrap_or(Level::TRACE);
    let layer = fmt::layer()
        .with_level(true)
        .with_writer(std::io::stdout)
        .with_timer(OffsetTime::local_rfc_3339().expect("could not get local offset!"))
        .with_filter(LevelFilter::from_level(level));
    Box::new(layer)
}