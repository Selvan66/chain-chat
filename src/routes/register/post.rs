use actix_web::{web, HttpResponse};
use secrecy::{ExposeSecret, Secret};
use sqlx::MySqlPool;

use crate::{
    database::users::check_if_username_exist,
    utils::{e500, see_other},
};

#[derive(serde::Deserialize)]
pub struct FormData {
    username: String,
    password: Secret<String>,
    confirm_password: Secret<String>,
}

#[tracing::instrument(skip_all)]
#[actix_web::post("/register")]
pub async fn register_post(
    form: web::Form<FormData>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse, actix_web::Error> {
    tracing::error!("TEST");
    if check_if_username_exist(&pool, form.username.as_str())
        .await
        .map_err(e500)?
    {
        // TODO Flash message: username used
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

    // TODO: Flash message: Register successful
    Ok(see_other("/"))
}
