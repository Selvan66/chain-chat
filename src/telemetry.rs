use std::{fs::File, io, path::PathBuf};

use tracing::{subscriber::set_global_default, Subscriber};
use tracing_appender::non_blocking;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_log::LogTracer;
use tracing_subscriber::{
    fmt, layer::SubscriberExt, registry::LookupSpan, EnvFilter, Layer, Registry,
};

pub enum LogConfig {
    File(PathBuf),
    Stdout,
    Stderr,
}

impl LogConfig {
    pub fn layer<S>(self) -> (Box<dyn Layer<S> + Send + Sync + 'static>, WorkerGuard)
    where
        S: Subscriber,
        for<'a> S: LookupSpan<'a>,
    {
        let fmt = fmt::layer().with_thread_names(true).pretty();

        let (non_blocking, guard) = match self {
            LogConfig::File(path) => {
                let file = File::create(path).expect("Failed to create log file");
                non_blocking(file)
            }
            LogConfig::Stdout => non_blocking(io::stdout()),
            LogConfig::Stderr => non_blocking(io::stderr()),
        };

        (Box::new(fmt.with_writer(non_blocking)), guard)
    }
}

pub fn init_tracing_logger(log_config: LogConfig, env_filter: String) -> WorkerGuard {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let (layer, guard) = log_config.layer();

    let subscriber = Registry::default().with(env_filter).with(layer);
    LogTracer::init().expect("Failed to set logger");
    // Ignore error - tests call init_tracing_logger multiple times.
    match set_global_default(subscriber) {
        Err(e) => tracing::error!("Logger set_global_default. Ignored {}", e),
        _ => (),
    }
    guard
}
