use chain_chat::configuration::get_configuration;
use chain_chat::startup::Application;
use chain_chat::telemetry::{init_tracing_logger, LogConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _guard = init_tracing_logger(LogConfig::Stdout, "info".into());
    tracing::trace!("START MAIN");

    let configuration = get_configuration().expect("Failed to read configuration");
    tracing::trace!("CONFIGURATION\n{:?}", &configuration);

    let application = Application::build(configuration).await?;

    tracing::trace!("START SERVER");
    application.run_until_stopped().await?;

    tracing::trace!("END MAIN");
    Ok(())
}
