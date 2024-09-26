use crate::helpers::{spawn_app, user::TestUser};

#[tokio::test]
async fn user_info_get_works() {
    let app = spawn_app().await;

    let user = TestUser::generate();
    user.register(&app).await;
    user.login(&app).await;

    let response = app.get_response("/user/info").await;

    assert!(response.status().is_success());

    let html = app.get_html("/user/info").await;
    assert!(html.contains(&user.username));
}
