use secrecy::Secret;

pub struct User {
    pub user_id: uuid::Uuid,
    pub email: String,
    pub password_hash: Secret<String>,
}
