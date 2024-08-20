use chain_chat::configuration::get_configuration;
use chain_chat::startup::Application;

pub struct TestApp {
    pub address: String,
    pub api_client: reqwest::Client,
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

    TestApp {
        address: format!("http://localhost:{}", application_port),
        api_client: client,
    }
}
