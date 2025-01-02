use chain_chat::domain::messages::*;

use crate::helpers::assert::{assert_flash_message, assert_is_redirect_to};
use crate::helpers::spawn_app;
use crate::helpers::user::TestUser;

#[tokio::test]
async fn logout_post_works() {
    let app = spawn_app().await;

    // Register
    let user = TestUser::generate();
    let response = user.register(&app).await;
    assert_is_redirect_to(&response, "/");

    // Login
    let response = user.login(&app).await;
    assert_is_redirect_to(&response, "/user/info");

    // Cannot go to /auth/login
    let response = app.get_response("/auth/login").await;
    assert_is_redirect_to(&response, "/user/info");
    assert_flash_message(&app, "/user/info", USER_LOGIN).await;

    // Logout
    let response = app.post("/user/logout").await;
    assert_is_redirect_to(&response, "/");
    assert_flash_message(&app, "/", LOGOUT_MESSAGE).await;

    // Can go to /auth/login
    let response = app.get_response("/auth/login").await;
    assert_eq!(response.status().as_u16(), 200);

    // Cannot logout if not login
    let response = app.post("/user/logout").await;
    assert_is_redirect_to(&response, "/auth/login");
}
