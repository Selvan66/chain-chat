use chain_chat::domain::messages::{MESSAGE_404, MESSAGE_500};

use crate::helpers::app::spawn_app;
use crate::helpers::database::drop_table;
use crate::helpers::user::TestUser;

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
async fn error_500_works() {
    let app = spawn_app().await;
    drop_table(&app.db_pool, "users").await.unwrap();

    let user = TestUser::generate();
    let response = user.register(&app).await;

    assert_eq!(response.status().as_u16(), 500);

    let text = response.text().await.unwrap();
    assert!(text.contains(MESSAGE_500));
}
