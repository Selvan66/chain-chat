use anyhow::Context;
use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};
use secrecy::{ExposeSecret, Secret};
use sqlx::MySqlPool;

use crate::{
    database::users::{change_user_password, get_user_id_and_password},
    error::ValidationError,
};

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

pub async fn validate_credentials(
    username: String,
    password: Secret<String>,
    pool: &MySqlPool,
) -> Result<String, ValidationError> {
    let mut user_id: Option<String> = None;
    let mut expected_password_hash = Secret::new("$argon2id$v=19$m=15000,t=2,p=1$JDJiJDEyJHhIU3A5MlpmSUl0RUlRemFldTZUYy4$ucZ8s1uXegdnt6wAaIu8+/b+64j2bp10djXEgIuhZm0".to_string());

    if let Some((stored_user_id, stored_password_hash)) =
        get_user_id_and_password(pool, &username).await?
    {
        user_id = Some(stored_user_id);
        expected_password_hash = stored_password_hash;
    }

    tokio::task::spawn_blocking(move || verify_password_hash(expected_password_hash, password))
        .await
        .context("Failed to spawn blocking task.")??;

    user_id
        .ok_or_else(|| anyhow::anyhow!("Unkown user"))
        .map_err(ValidationError::ValidationError)
}

pub async fn change_password(
    user_id: String,
    password: Secret<String>,
    pool: &MySqlPool,
) -> Result<(), anyhow::Error> {
    let password_hash = tokio::task::spawn_blocking(move || compute_password_hash(password))
        .await?
        .context("Failed to hash password")?;

    change_user_password(pool, &user_id, password_hash)
        .await
        .context("Failed to change password")?;

    Ok(())
}

fn verify_password_hash(
    expected_password_hash: Secret<String>,
    password_candidate: Secret<String>,
) -> Result<(), ValidationError> {
    let expected_password_hash = PasswordHash::new(expected_password_hash.expose_secret())
        .context("Failed to parse hash in PHC string format.")?;

    Argon2::default()
        .verify_password(
            password_candidate.expose_secret().as_bytes(),
            &expected_password_hash,
        )
        .context("Invalid password")
        .map_err(ValidationError::ValidationError)
}
