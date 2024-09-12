use secrecy::Secret;

pub struct User {
    pub user_id: uuid::Uuid,
    pub username: String,
    pub password_hash: Secret<String>,
}
