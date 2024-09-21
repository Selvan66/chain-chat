use actix_web::{web, HttpResponse};
use anyhow::Context;
use secrecy::{ExposeSecret, Secret};
use sqlx::MySqlPool;

use crate::{
    cryptografic::compute_password_hash,
    database::users::{add_user, check_if_username_exist},
    domain::{messages::*, User},
    utils::{e500, see_other_with_flash},
};

#[derive(serde::Deserialize, Debug)]
struct FormData {
    username: String,
    password: Secret<String>,
    confirm_password: Secret<String>,
}

#[tracing::instrument(skip_all, fields(username = form.username))]
#[actix_web::post("/register")]
pub async fn register_post(
    form: web::Form<FormData>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse, actix_web::Error> {
    if form.username.len() < 4 {
        tracing::error!(REGISTRATION_FAILED_USERNAME_TOO_SHORT);
        return Ok(see_other_with_flash(
            "/auth/register",
            REGISTRATION_FAILED_USERNAME_TOO_SHORT,
        ));
    }

    if form.username.len() > 250 {
        tracing::error!(REGISTRATION_FAILED_USERNAME_TOO_LONG);
        return Ok(see_other_with_flash(
            "/auth/register",
            REGISTRATION_FAILED_USERNAME_TOO_LONG,
        ));
    }

    if form.password.expose_secret() != form.confirm_password.expose_secret() {
        tracing::error!(REGISTRATION_FAILED_PASSWORD_NOT_EQ_CONFIRM);
        return Ok(see_other_with_flash(
            "/auth/register",
            REGISTRATION_FAILED_PASSWORD_NOT_EQ_CONFIRM,
        ));
    }

    if form.password.expose_secret().len() < 4 {
        tracing::error!(REGISTRATION_FAILED_PASSWORD_TOO_SHORT);
        return Ok(see_other_with_flash(
            "/auth/register",
            REGISTRATION_FAILED_PASSWORD_TOO_SHORT,
        ));
    }

    if check_if_username_exist(&pool, &form.username)
        .await
        .map_err(e500)?
    {
        tracing::error!(REGISTRATION_FAILED_USERNAME_USED);
        return Ok(see_other_with_flash(
            "/auth/register",
            REGISTRATION_FAILED_USERNAME_USED,
        ));
    }

    add_user(
        &pool,
        User {
            user_id: uuid::Uuid::new_v4(),
            username: form.username.to_string(),
            password_hash: compute_password_hash(form.password.clone()).map_err(e500)?,
        },
    )
    .await
    .context("Cannot register user")
    .map_err(e500)?;

    tracing::info!("User {} registered", form.username);

    Ok(see_other_with_flash("/", REGISTRATION_SUCCESSFUL))
}
