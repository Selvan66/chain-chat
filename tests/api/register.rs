use fake::Fake;

use chain_chat::database::users::check_if_email_exist;
use chain_chat::domain::messages::*;

use crate::helpers::assert::*;
use crate::helpers::database::drop_table;
use crate::helpers::spawn_app;
use crate::helpers::user::TestUser;

#[tokio::test]
async fn register_get_works() {
    let app = spawn_app().await;

    let response = app.get_response("/auth/register").await;

    assert!(response.status().is_success());

    let html = app.get_html("/auth/register").await;
    assert!(html.contains("Email"));
    assert!(html.contains("Password"));
    assert!(html.contains("Confirm Password"));
}

#[tokio::test]
async fn register_post_works() {
    let app = spawn_app().await;

    let user = TestUser::generate();

    let response = user.register(&app).await;
    assert_is_redirect_to(&response, "/");
    assert_flash_message(&app, "/", REGISTRATION_SUCCESSFUL).await;

    assert!(check_if_email_exist(&app.db_pool, user.email.as_str())
        .await
        .expect("Cannot query database"));
}

#[tokio::test]
async fn email_is_too_long() {
    let app = spawn_app().await;

    let mut user = TestUser::generate();
    user.email = 251.fake::<String>() + "@test.com";

    let response = user.register(&app).await;
    assert_is_redirect_to(&response, "/auth/register");

    assert_flash_message(&app, "/auth/register", FAILED_EMAIL_TOO_LONG).await;
}

#[tokio::test]
async fn validate_email() {
    let app = spawn_app().await;
    let mut user = TestUser::generate();

    let test_cases_wrong = vec![
        "plainaddress",
        "#@%^%#@#@#.com",
        "@example.com",
        "Joe Smith email@example.com",
        "email.example.com",
        "email@example@example.com",
        ".email@example.com",
        "email.@example.com",
        "email..email@example.com",
        "email@example.com (Joe Smith)",
        "email@example",
        "email@-example.com",
        "email@example.c@m",
        "email@example..com",
        "Abc..123@example.com",
        "just\"not\"right@example.com",
        "this\\ is\"really\"not\\allowed@example.com",
        "Simon <simon@example.com>",
        "<simon@example.com>",
    ];

    for case in test_cases_wrong {
        user.email = case.to_string();
        let response = user.register(&app).await;
        assert_is_redirect_to_with_assert_message(&response, "/auth/register", case);
        assert_flash_message(&app, "/auth/register", FAILED_WRONG_EMAIL).await;
    }

    let test_cases_correct = vec![
        "tom@example.com",
        "a@a.com",
        "name+surname@mail.net",
        "name.surname@mail.net",
        "user.name@mail.example.co.uk",
        "user123@domain.com",
        "user.name+tag+sorting@example.com",
        "4@gmail.com",
        "A@gmail.com",
    ];

    for case in test_cases_correct {
        user.email = case.to_string();
        let response = user.register(&app).await;
        assert_is_redirect_to_with_assert_message(&response, "/", case);
        assert_flash_message(&app, "/auth/register", REGISTRATION_SUCCESSFUL).await;
    }
}

#[tokio::test]
async fn register_short_password() {
    let app = spawn_app().await;

    let mut user = TestUser::generate();
    user.password = "a".to_string();

    let response = user.register(&app).await;
    assert_is_redirect_to(&response, "/auth/register");

    assert_flash_message(&app, "/auth/register", FAILED_PASSWORD_TOO_SHORT).await;
}

#[tokio::test]
async fn username_is_already_used() {
    let app = spawn_app().await;

    let user = TestUser::generate();

    let response = user.register(&app).await;
    assert_is_redirect_to(&response, "/");

    // Register second time
    let response = user.register(&app).await;
    assert_is_redirect_to(&response, "/auth/register");

    assert_flash_message(&app, "/auth/register", FAILED_EMAIL_USED).await;
}

#[tokio::test]
async fn password_and_confirm_password_is_not_equal() {
    let app = spawn_app().await;

    let user = TestUser::generate();

    let register_body = serde_json::json!({
        "email": user.email,
        "password": user.password,
        "confirm_password": uuid::Uuid::new_v4().to_string(),
    });

    let response = app.post_body(&register_body, "/auth/register").await;
    assert_is_redirect_to(&response, "/auth/register");

    assert_flash_message(&app, "/auth/register", FAILED_PASSWORD_NOT_EQ_CONFIRM).await;
}

#[tokio::test]
async fn cannot_register_if_you_are_login() {
    let app = spawn_app().await;

    let user = TestUser::generate();

    // Register new user
    let response = user.register(&app).await;
    assert_is_redirect_to(&response, "/");

    // Login new user
    let response = user.login(&app).await;
    assert_is_redirect_to(&response, "/user/info");

    // Cannot register if you login
    let response = app.get_response("/auth/register").await;
    assert_is_redirect_to(&response, "/user/info");
    assert_flash_message(&app, "/user/info", USER_LOGIN).await;

    let new_user = TestUser::generate();
    let response = new_user.register(&app).await;
    assert_is_redirect_to(&response, "/user/info");
    assert_flash_message(&app, "/user/info", USER_LOGIN).await;
}

#[tokio::test]
async fn error_500_if_register_while_database_down() {
    let app = spawn_app().await;
    drop_table(&app.db_pool, "users").await.unwrap();

    let user = TestUser::generate();
    let response = user.register(&app).await;

    assert_eq!(response.status().as_u16(), 500);

    let text = response.text().await.unwrap();
    assert!(text.contains(MESSAGE_500));
}
