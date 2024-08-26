use crate::helpers::spawn_app;

#[tokio::test]
async fn home_works() {
    let app = spawn_app().await;

    let response = app.get_response("/").await;

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());

    let html = app.get_html("/").await;
    assert!(!html.contain("Welcome"));
}
