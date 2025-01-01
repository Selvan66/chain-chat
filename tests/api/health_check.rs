use crate::helpers::app::spawn_app;

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;

    let response = app.get_response("/health_check").await;

    assert!(response.status().is_success());
}
