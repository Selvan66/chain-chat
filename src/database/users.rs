use anyhow::Context;
use secrecy::ExposeSecret;
use sqlx::MySqlPool;

use crate::domain::User;

pub async fn check_if_username_exist(
    pool: &MySqlPool,
    username: &str,
) -> Result<bool, anyhow::Error> {
    match sqlx::query!(
        r#" 
        SELECT user_id
        FROM users
        WHERE username = ?
    "#,
        username
    )
    .fetch_optional(pool)
    .await
    .context("Failed to query database about username existence")?
    {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

pub async fn add_user(pool: &MySqlPool, user: User) -> Result<(), anyhow::Error> {
    sqlx::query!(
        r#"
    INSERT INTO users (user_id, username, password_hash)
    VALUES (?, ?, ?)
    "#,
        user.user_id.to_string(),
        user.username,
        user.password_hash.expose_secret()
    )
    .execute(pool)
    .await
    .context("Failed to insert user to database")?;

    Ok(())
}
