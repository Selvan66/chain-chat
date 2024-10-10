use sqlx::mysql::MySqlPool;
use tracing_appender::non_blocking::WorkerGuard;

use chain_chat::configuration::get_configuration;
use chain_chat::database::init::connection_without_db;
use chain_chat::startup::Application;
use chain_chat::telemetry::{init_tracing_logger, LogConfig};
use uuid::Uuid;

use crate::helpers::database::configure_database;

pub struct TestApp {
    pub address: String,
    pub api_client: reqwest::Client,
    pub db_pool: MySqlPool,

    _log_guard: WorkerGuard,
}

impl TestApp {
    pub async fn get_response(&self, path: &str) -> reqwest::Response {
        self.api_client
            .get(&format!("{}{}", self.address, path))
            .send()
            .await
            .expect("Failed to GET")
    }

    pub async fn get_html(&self, path: &str) -> String {
        self.get_response(path).await.text().await.unwrap()
    }

    pub async fn post_body<Body>(&self, body: &Body, path: &str) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.api_client
            .post(&format!("{}{}", self.address, path))
            .form(body)
            .send()
            .await
            .expect("Failed to POST with body")
    }

    pub async fn post(&self, path: &str) -> reqwest::Response {
        self.api_client
            .post(&format!("{}{}", self.address, path))
            .send()
            .await
            .expect("Failed to POST")
    }
}

pub async fn spawn_app() -> TestApp {
    let database_name = Uuid::new_v4().to_string();
    let mut configuration = get_configuration().expect("Failed to read configuration");
    configuration.application.port = 0;
    configuration.database.database_name = database_name.clone();

    let connection = connection_without_db(&configuration.database);
    let db_pool = configure_database(connection, database_name).await;

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

    let guard = init_tracing_logger(LogConfig::File("log/test_log.txt".into()), "info".into());

    TestApp {
        address: format!("http://localhost:{}", application_port),
        api_client: client,
        db_pool,

        _log_guard: guard,
    }
}
