use crate::helpers::TestApp;

pub fn assert_is_redirect_to(response: &reqwest::Response, location: &str) {
    assert_eq!(response.status().as_u16(), 303);
    assert_eq!(response.headers().get("Location").unwrap(), location);
}

pub fn assert_is_redirect_to_with_assert_message(
    response: &reqwest::Response,
    location: &str,
    message: &str,
) {
    assert_eq!(response.status().as_u16(), 303, "{}", message);
    assert_eq!(
        response.headers().get("Location").unwrap(),
        location,
        "{}",
        message
    );
}

pub async fn assert_flash_message(app: &TestApp, path: &str, message: &str) {
    let html = app.get_html(path).await;
    assert!(html.contains(message), "{}", message);
    let html = app.get_html(path).await;
    assert!(!html.contains(message), "{}", message);
}
