use chain_chat::configuration::get_configuration;
use chain_chat::startup::Application;
use chain_chat::telemetry::{init_tracing_logger, LogConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _guard = init_tracing_logger(LogConfig::Stdout, "info".into());
    tracing::info!("START MAIN");

    let configuration = get_configuration().expect("Failed to read configuration");
    tracing::info!("CONFIGURATION\n{:?}", &configuration);

    let application = Application::build(configuration).await?;

    tracing::info!("START SERVER");
    application.run_until_stopped().await?;

    tracing::info!("END MAIN");
    Ok(())
}
