use chain_chat::telemetry::{get_tracing_subscriber, init_tracing_subscriber, LogConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let subscriber = get_tracing_subscriber(LogConfig::Stdout, "info".into());
    init_tracing_subscriber(subscriber);
    Ok(())
}
