use fake::faker::name;
use fake::Fake;
use reqwest::Response;

use crate::helpers::TestApp;

pub struct TestUser {
    pub username: String,
    pub password: String,
}

impl TestUser {
    pub fn generate() -> Self {
        Self {
            username: name::en::Name().fake(),
            password: uuid::Uuid::new_v4().to_string(),
        }
    }

    pub async fn register(&self, app: &TestApp) -> Response {
        let register_body = serde_json::json!({
            "username": self.username,
            "password": self.password,
            "confirm_password": self.password,
        });

        app.post_body(&register_body, "/auth/register").await
    }

    pub async fn login(&self, app: &TestApp) -> Response {
        let login_body = serde_json::json!({
            "username": self.username,
            "password": self.password
        });

        app.post_body(&login_body, "/auth/login").await
    }

    pub async fn change_password(&self, app: &TestApp, new_password: &str) -> Response {
        let password_body = serde_json::json!({
            "old_password": self.password,
            "new_password": new_password,
            "confirm_new_password": new_password
        });

        app.post_body(&password_body, "/user/password").await
    }
}
