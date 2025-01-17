use crate::helpers::app::spawn_app;

#[tokio::test]
async fn favicon_works() {
    let app = spawn_app().await;

    let response = app.get_response("/favicon.ico").await;

    assert!(response.status().is_success());
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "image/x-icon"
    );
}
