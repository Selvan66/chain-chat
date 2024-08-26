use tracing_appender::non_blocking::WorkerGuard;

use chain_chat::configuration::get_configuration;
use chain_chat::startup::Application;
use chain_chat::telemetry::{init_tracing_logger, LogConfig};

pub struct TestApp {
    pub address: String,
    pub api_client: reqwest::Client,

    _log_guard: WorkerGuard,
}

impl TestApp {
    pub async fn get_response(&self, path: &str) -> reqwest::Response {
        self.api_client
            .get(&format!("{}{}", &app.address, path))
            .send()
            .await
            .expect("Failed to execute request")
    }

    pub async fn get_html(&self, path: &str) -> String {
        self.get_response(path).await.text().await.unwrap()
    }
}

pub async fn spawn_app() -> TestApp {
    let configuration = get_configuration().expect("Failed to read configuration");

    let application = Application::build(configuration)
        .await
        .expect("Failed to build application");
    let application_port = application.port();
    let _ = tokio::spawn(application.run_until_stopped());

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .cookie_store(true)
        .build()
        .expect("Failed to build client");

    let guard = init_tracing_logger(LogConfig::Stdout, "info".into());

    TestApp {
        address: format!("http://localhost:{}", application_port),
        api_client: client,

        _log_guard: guard,
    }
}
