use chain_chat::domain::messages::*;

use crate::helpers::assert::{assert_flash_message, assert_is_redirect_to};
use crate::helpers::spawn_app;
use crate::helpers::user::TestUser;

#[tokio::test]
async fn password_get_works() {
    let app = spawn_app().await;

    // Cannot go to password if not login
    let response = app.get_response("/user/logout").await;
    assert_is_redirect_to(&response, "/auth/login");

    // Create user and login
    let user = TestUser::generate();
    user.register(&app).await;
    user.login(&app).await;

    // Password
    let response = app.get_response("/user/password").await;
    assert_eq!(response.status().as_u16(), 200);

    let html = app.get_html("/user/password").await;
    assert!(html.contains("Current Password"));
}

#[tokio::test]
async fn password_post_works() {
    let app = spawn_app().await;

    // Cannot go to password if not login
    let response = app.post("/user/password").await;
    assert_is_redirect_to(&response, "/auth/login");

    // Create user and login
    let user = TestUser::generate();
    user.register(&app).await;
    user.login(&app).await;

    let new_password = uuid::Uuid::new_v4().to_string();

    // Changing password
    let response = user.change_password(&app, &new_password).await;
    assert_is_redirect_to(&response, "/user/info");
    assert_flash_message(&app, "/user/info", CHANGE_PASSWORD_SUCCESSFUL).await;

    // Logout
    app.post("/user/logout").await;

    // Try to login with old password
    let response = user.login(&app).await;
    assert_is_redirect_to(&response, "/auth/login");
    assert_flash_message(&app, "/auth/login", AUTHENTICATION_FAILED).await;

    // Try to login with new password
    let user = TestUser {
        email: user.email,
        password: new_password,
    };

    let response = user.login(&app).await;
    assert_is_redirect_to(&response, "/user/info");
}

#[tokio::test]
async fn password_too_short() {
    let app = spawn_app().await;
    let user = TestUser::generate();
    user.register(&app).await;
    user.login(&app).await;

    let new_password = "a".to_string();

    let response = user.change_password(&app, &new_password).await;
    assert_is_redirect_to(&response, "/user/password");
    assert_flash_message(&app, "/user/password", FAILED_PASSWORD_TOO_SHORT).await;
}

#[tokio::test]
async fn old_password_wrong() {
    let app = spawn_app().await;
    let mut user = TestUser::generate();
    user.register(&app).await;
    user.login(&app).await;

    user.password = "aaaaaa".to_string();
    let new_password = uuid::Uuid::new_v4().to_string();

    let response = user.change_password(&app, &new_password).await;
    assert_is_redirect_to(&response, "/user/password");
    assert_flash_message(&app, "/user/password", FAILED_CURRENT_PASSWORD_WRONG).await;
}

#[tokio::test]
async fn new_password_not_eq_confirm() {
    let app = spawn_app().await;
    let user = TestUser::generate();
    user.register(&app).await;
    user.login(&app).await;

    let new_password = uuid::Uuid::new_v4().to_string();
    let confirm_new_password = uuid::Uuid::new_v4().to_string();

    let password_body = serde_json::json!({
        "old_password": user.password,
        "new_password": new_password,
        "confirm_new_password": confirm_new_password,
    });

    let response = app.post_body(&password_body, "/user/password").await;
    assert_is_redirect_to(&response, "/user/password");
    assert_flash_message(&app, "/user/password", FAILED_PASSWORD_NOT_EQ_CONFIRM).await;
}
