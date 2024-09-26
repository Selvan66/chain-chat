use chain_chat::domain::messages::*;

use crate::helpers::assert::{assert_flash_message, assert_is_redirect_to};
use crate::helpers::spawn_app;
use crate::helpers::user::TestUser;

#[tokio::test]
async fn login_get_works() {
    let app = spawn_app().await;

    let response = app.get_response("/auth/login").await;

    assert!(response.status().is_success());

    let html = app.get_html("/auth/login").await;
    assert!(html.contains("Username"));
    assert!(html.contains("Password"));
}

#[tokio::test]
async fn login_post_works() {
    let app = spawn_app().await;

    // Register
    let user = TestUser::generate();
    let response = user.register(&app).await;
    assert_is_redirect_to(&response, "/");

    // Login
    let response = user.login(&app).await;
    assert_is_redirect_to(&response, "/user/info");
}

#[tokio::test]
async fn an_error_message_on_failure() {
    let app = spawn_app().await;

    let mut user = TestUser::generate();
    // Register
    let response = user.register(&app).await;
    assert_is_redirect_to(&response, "/");

    // Login - wrong user name
    let username = user.username;
    user.username = "abcd".to_string();

    let response = user.login(&app).await;
    assert_is_redirect_to(&response, "/auth/login");
    assert_flash_message(&app, "/auth/login", AUTHENTICATION_FAILED).await;

    user.username = username;

    // Login - wrong password
    user.password = "abcd".to_string();
    let response = user.login(&app).await;
    assert_is_redirect_to(&response, "/auth/login");
    assert_flash_message(&app, "/auth/login", AUTHENTICATION_FAILED).await;
}

#[tokio::test]
async fn cannot_login_if_you_are_already_login() {
    let app = spawn_app().await;

    let user = TestUser::generate();

    // Register new user
    let response = user.register(&app).await;
    assert_is_redirect_to(&response, "/");

    // Login new user
    let response = user.login(&app).await;
    assert_is_redirect_to(&response, "/user/info");

    // Cannot login if you login
    let response = app.get_response("/auth/login").await;
    assert_is_redirect_to(&response, "/user/info");
    assert_flash_message(&app, "/user/info", USER_LOGIN).await;

    let new_user = TestUser::generate();
    let response = new_user.login(&app).await;
    assert_is_redirect_to(&response, "/user/info");
    assert_flash_message(&app, "/user/info", USER_LOGIN).await;
}
