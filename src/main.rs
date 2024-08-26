use chain_chat::configuration::get_configuration;
use chain_chat::startup::Application;
use chain_chat::telemetry::{init_tracing_logger, LogConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _guard = init_tracing_logger(LogConfig::Stdout, "info".into());

    let configuration = get_configuration().expect("Failed to read configuration");
    let application = Application::build(configuration).await?;

    application.run_until_stopped().await?;

    Ok(())
}
