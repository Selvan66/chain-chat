use anyhow::Context;
use argon2::{password_hash::SaltString, Algorithm, Argon2, Params, PasswordHasher, Version};
use secrecy::{ExposeSecret, Secret};
use sqlx::MySqlPool;
use uuid::Uuid;

pub fn compute_password_hash(password: Secret<String>) -> Result<Secret<String>, anyhow::Error> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    let password_hash = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(15000, 2, 1, None).unwrap(),
    )
    .hash_password(password.expose_secret().as_bytes(), &salt)
    .context("Cannot hash password")?
    .to_string();

    Ok(Secret::new(password_hash))
}

pub async fn validate_login(
    username: String,
    password: Secret<String>,
    pool: &MySqlPool,
) -> Result<Uuid, anyhow::Error> {
    let mut user_id = None;
    let mut expected_password_hash = Secret::new("$argon2id$v=19$m=15000,t=2,p=1$JDJiJDEyJHhIU3A5MlpmSUl0RUlRemFldTZUYy4$ucZ8s1uXegdnt6wAaIu8+/b+64j2bp10djXEgIuhZm0".to_string());

    todo!()
}
