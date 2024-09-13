use actix_web::{web, HttpResponse};
use anyhow::Context;
use secrecy::{ExposeSecret, Secret};
use sqlx::MySqlPool;

use crate::{
    database::users::{add_user, check_if_username_exist},
    domain::user::User,
    utils::{e500, see_other},
};

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
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
        // TODO: Flash message: Username too short
        return Ok(see_other("/register"));
    }

    if form.username.len() > 250 {
        // TODO: Flash message: Username too long
        return Ok(see_other("/register"));
    }

    if form.password.expose_secret() != form.confirm_password.expose_secret() {
        // TODO: Flash message: Password not equal
        return Ok(see_other("/register"));
    }

    if form.password.expose_secret().len() < 4 {
        // TODO: Flash message: Password too short
        return Ok(see_other("/register"));
    }

    if check_if_username_exist(&pool, &form.username)
        .await
        .map_err(e500)?
    {
        // TODO Flash message: username used
        return Ok(see_other("/register"));
    }

    add_user(
        &pool,
        User {
            user_id: uuid::Uuid::new_v4(),
            username: form.username.to_string(),
            password_hash: form.password.clone(),
        },
    )
    .await
    .context("Cannot register user")
    .map_err(e500)?;

    tracing::info!("User {} registered", form.username);

    // TODO: Flash message: Register successful
    Ok(see_other("/"))
}
