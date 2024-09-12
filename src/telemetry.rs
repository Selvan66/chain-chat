use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};

use tracing::{subscriber::set_global_default, Subscriber};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_appender::rolling::{RollingFileAppender, Rotation};
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
                let default_name = "log.txt";
                let log_appender = RollingFileAppender::builder()
                    .rotation(Rotation::DAILY)
                    .max_log_files(3)
                    .filename_suffix(
                        path.file_name()
                            .unwrap_or(OsStr::new(default_name))
                            .to_str()
                            .unwrap_or(default_name),
                    )
                    .build(path.parent().unwrap_or(Path::new("./")))
                    .expect("Failed to initialiase rolling file");
                tracing_appender::non_blocking(log_appender)
            }
            LogConfig::Stdout => tracing_appender::non_blocking(io::stdout()),
            LogConfig::Stderr => tracing_appender::non_blocking(io::stderr()),
        };

        (Box::new(fmt.with_writer(non_blocking)), guard)
    }
}

pub fn init_tracing_logger(log_config: LogConfig, env_filter: String) -> WorkerGuard {
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    let (layer, guard) = log_config.layer();

    let subscriber = Registry::default().with(env_filter).with(layer);

    // Ignore errors - tests call init_tracing_logger multiple times.
    match LogTracer::init() {
        Err(e) => tracing::warn!("Logger LogTracer::init | Ignored {}", e),
        _ => (),
    }
    match set_global_default(subscriber) {
        Err(e) => tracing::warn!("Logger set_global_default | Ignored {}", e),
        _ => (),
    }
    guard
}
