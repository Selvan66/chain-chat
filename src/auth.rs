use anyhow::Context;
use argon2::{
    password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
    PasswordVerifier, Version,
};
use secrecy::{ExposeSecret, Secret};
use sqlx::MySqlPool;

use crate::{
    database::users::{change_user_password, check_if_email_exist, get_user_id_and_password},
    domain::messages::*,
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
    email: String,
    password: Secret<String>,
    pool: &MySqlPool,
) -> Result<String, ValidationError> {
    let mut user_id: Option<String> = None;
    let mut expected_password_hash = Secret::new("$argon2id$v=19$m=15000,t=2,p=1$JDJiJDEyJHhIU3A5MlpmSUl0RUlRemFldTZUYy4$ucZ8s1uXegdnt6wAaIu8+/b+64j2bp10djXEgIuhZm0".to_string());

    if let Some((stored_user_id, stored_password_hash)) =
        get_user_id_and_password(pool, &email).await?
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

pub async fn validate_register(
    email: &str,
    password: &Secret<String>,
    confirm_password: &Secret<String>,
    pool: &MySqlPool,
) -> Result<(), ValidationError> {
    validate_email(email)?;

    if check_if_email_exist(pool, email)
        .await
        .map_err(ValidationError::UnexpectedError)?
    {
        return Err(ValidationError::ValidationError(anyhow::anyhow!(
            FAILED_EMAIL_USED
        )));
    }

    validate_password_and_confirm(password, confirm_password)
}

pub fn validate_email(email: &str) -> Result<(), ValidationError> {
    if email.len() > 250 {
        return Err(ValidationError::ValidationError(anyhow::anyhow!(
            FAILED_EMAIL_TOO_LONG
        )));
    }

    let re = regex::Regex::new(
        r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})",
    )
    .context("Failed to compile regex")
    .map_err(ValidationError::UnexpectedError)?;

    if !re.is_match(email) {
        return Err(ValidationError::ValidationError(anyhow::anyhow!(
            FAILED_WRONG_EMAIL
        )));
    }

    Ok(())
}

pub fn validate_password_and_confirm(
    password: &Secret<String>,
    confirm_password: &Secret<String>,
) -> Result<(), ValidationError> {
    if password.expose_secret() != confirm_password.expose_secret() {
        return Err(ValidationError::ValidationError(anyhow::anyhow!(
            FAILED_PASSWORD_NOT_EQ_CONFIRM
        )));
    }

    if password.expose_secret().len() < 4 {
        return Err(ValidationError::ValidationError(anyhow::anyhow!(
            FAILED_PASSWORD_TOO_SHORT
        )));
    }

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
        .context(FAILED_CURRENT_PASSWORD_WRONG)
        .map_err(ValidationError::ValidationError)
}
