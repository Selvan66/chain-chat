use chain_chat::telemetry::{init_tracing_logger, LogConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _guard = init_tracing_logger(LogConfig::Stdout, "info".into());

    Ok(())
}
