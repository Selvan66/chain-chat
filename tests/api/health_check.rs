use chain_chat::domain::messages::*;

use crate::helpers::app::spawn_app;

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;

    let response = app.get_response("/health_check").await;

    assert!(response.status().is_success());
}

#[tokio::test]
async fn error_404_works() {
    let app = spawn_app().await;

    let tests_cases = vec![
        "/not_found_asdasdasdasdwa",
        "/dlaksjdlkas/alskdjlaskjd/lasjdlkasjd",
        "/djaslkda/ajsdasd/ajskdjaks/jkasdjkasjd/jkasjdksda",
    ];

    for case in tests_cases {
        let response = app.get_response(case).await;

        assert_eq!(response.status().as_u16(), 404);

        let html = app.get_html(case).await;
        assert!(html.contains(MESSAGE_404));
    }
}

#[tokio::test]
async fn other_errors_works() {
    let app = spawn_app().await;
    let response = app.post("/auth/register").await;
    assert_eq!(response.status().as_u16(), 415);

    let html = app.post_html("/auth/register").await;
    assert!(html.contains(MESSAGE_UNKOWN_ERROR));
}
