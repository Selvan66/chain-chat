use crate::helpers::app::spawn_app;

#[tokio::test]
async fn check_assets() {
    let app = spawn_app().await;

    let paths = vec!["/favicon.ico", "/assets/github-mark.svg"];

    for path in paths {
        let response = app.get_response(path).await;
        assert!(response.status().is_success());
    }
}
