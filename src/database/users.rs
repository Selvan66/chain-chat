use anyhow::Context;
use secrecy::{ExposeSecret, Secret};
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

pub async fn get_user_id_and_password(
    pool: &MySqlPool,
    username: &str,
) -> Result<Option<(String, Secret<String>)>, anyhow::Error> {
    let row: Option<_> = sqlx::query!(
        r#"
    SELECT user_id, password_hash
    FROM users
    WHERE username = ?
    "#,
        username
    )
    .fetch_optional(pool)
    .await
    .context("Failed to preform a query to validate auth credentials.")?
    .map(|row| (row.user_id, Secret::new(row.password_hash)));
    Ok(row)
}

pub async fn get_username(pool: &MySqlPool, user_id: &str) -> Result<String, anyhow::Error> {
    let row = sqlx::query!(
        r#"
    SELECT username
    FROM users
    WHERE user_id = ?
    "#,
        user_id
    )
    .fetch_one(pool)
    .await
    .context("Failed to perform a query to retrieve a username.")?;
    Ok(row.username)
}
