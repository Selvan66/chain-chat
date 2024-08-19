use std::{fs::File, io, path::PathBuf};

use tracing::{subscriber::set_global_default, Subscriber};
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
    pub fn layer<S>(self) -> Box<dyn Layer<S> + Send + Sync + 'static>
    where
        S: Subscriber,
        for<'a> S: LookupSpan<'a>,
    {
        let fmt = fmt::layer().with_thread_names(true).json();

        match self {
            LogConfig::File(path) => {
                let file = File::create(path).expect("Failed to create log file");
                Box::new(fmt.with_writer(file))
            }
            LogConfig::Stdout => Box::new(fmt.with_writer(io::stdout)),
            LogConfig::Stderr => Box::new(fmt.with_writer(io::stderr)),
        }
    }
}

pub fn get_tracing_subscriber(
    log_config: LogConfig,
    env_filter: String,
) -> impl Subscriber + Send + Sync {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    Registry::default()
        .with(env_filter)
        .with(log_config.layer())
}

pub fn init_tracing_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("Failed to set logger");
    set_global_default(subscriber).expect("Failed to set subscriber");
}
