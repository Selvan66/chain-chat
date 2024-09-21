use crate::helpers::spawn_app;

#[tokio::test]
async fn login_works() {
    let app = spawn_app().await;

    let response = app.get_response("/auth/login").await;

    assert!(response.status().is_success());

    let html = app.get_html("/auth/login").await;
    assert!(html.contains("Username"));
    assert!(html.contains("Password"));
}
