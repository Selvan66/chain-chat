use crate::helpers::app::spawn_app;

#[tokio::test]
async fn home_works() {
    let app = spawn_app().await;

    let response = app.get_response("/").await;

    assert!(response.status().is_success());

    let html = app.get_html("/").await;
    assert!(html.contains("Chain chat"));
}
