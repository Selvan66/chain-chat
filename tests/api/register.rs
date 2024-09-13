use chain_chat::database::users::check_if_username_exist;
use fake::faker::name;
use fake::Fake;

use crate::helpers::assert::assert_is_redirect_to;
use crate::helpers::spawn_app;

#[tokio::test]
async fn register_get_works() {
    let app = spawn_app().await;

    let response = app.get_response("/register").await;

    assert!(response.status().is_success());

    let html = app.get_html("/register").await;
    assert!(html.contains("Username"));
    assert!(html.contains("Password"));
    assert!(html.contains("Confirm password"));
}

#[tokio::test]
async fn register_post_works() {
    let app = spawn_app().await;

    let username: String = name::en::Name().fake();
    let password = uuid::Uuid::new_v4().to_string();

    let register_body = serde_json::json!({
        "username": username,
        "password": password,
        "confirm_password": password,
    });

    let response = app.post_body(&register_body, "/register").await;
    assert_is_redirect_to(&response, "/");

    // TODO: Check flash message for: register successful

    assert!(check_if_username_exist(&app.db_pool, username.as_str())
        .await
        .expect("Cannot query database"));
}

#[tokio::test]
async fn username_is_too_long() {
    let app = spawn_app().await;

    let username: String = 300.fake();
    let password = uuid::Uuid::new_v4().to_string();

    let register_body = serde_json::json!({
        "username": username,
        "password": password,
        "confirm_password": password,
    });

    let response = app.post_body(&register_body, "/register").await;
    assert_is_redirect_to(&response, "/register");

    // TODO: Check flash message for: register failed - username to long
}

#[tokio::test]
async fn username_is_too_short() {
    let app = spawn_app().await;

    let password = uuid::Uuid::new_v4().to_string();

    let register_body = serde_json::json!({
        "username": "a",
        "password": password,
        "confirm_password": password,
    });

    let response = app.post_body(&register_body, "/register").await;
    assert_is_redirect_to(&response, "/register");

    // TODO: Check flash message for: register failed - username field is empty
}

#[tokio::test]
async fn register_short_password() {
    let app = spawn_app().await;

    let username: String = name::en::Name().fake();
    let password = uuid::Uuid::new_v4().to_string();

    let register_body = serde_json::json!({
        "username": username,
        "password": "a",
        "confirm_password": password,
    });

    let response = app.post_body(&register_body, "/register").await;
    assert_is_redirect_to(&response, "/register");

    // TODO: Check flash message for: register failed - password field is empty

    let register_body = serde_json::json!({
        "username": username,
        "password": password,
        "confirm_password": "",
    });

    let response = app.post_body(&register_body, "/register").await;
    assert_is_redirect_to(&response, "/register");

    // TODO: Check flash message for: register failed - password field is empty
}

#[tokio::test]
async fn username_is_already_used() {
    let app = spawn_app().await;

    let username: String = name::en::Name().fake();
    let password = uuid::Uuid::new_v4().to_string();

    let register_body = serde_json::json!({
        "username": username,
        "password": password,
        "confirm_password": password,
    });

    let response = app.post_body(&register_body, "/register").await;
    assert_is_redirect_to(&response, "/");

    // TODO: Check flash message for: register successful

    let response = app.post_body(&register_body, "/register").await;
    assert_is_redirect_to(&response, "/register");

    // TODO: Check flash message for: register failed - username is already used
}

#[tokio::test]
async fn password_and_confirm_password_is_not_equal() {
    let app = spawn_app().await;

    let username: String = name::en::Name().fake();

    let register_body = serde_json::json!({
        "username": username,
        "password": uuid::Uuid::new_v4().to_string(),
        "confirm_password": uuid::Uuid::new_v4().to_string(),
    });

    let response = app.post_body(&register_body, "/register").await;
    assert_is_redirect_to(&response, "/register");

    // TODO: Check flash message for: register failed - username is already used
}
