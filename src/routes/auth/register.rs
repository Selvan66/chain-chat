use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse};
use anyhow::Context;
use secrecy::{ExposeSecret, Secret};
use sqlx::MySqlPool;
use tera::Tera;

use crate::{
    cryptografic::compute_password_hash,
    database::users::{add_user, check_if_username_exist},
    domain::{messages::*, User},
    utils::{delete_flash_cookie, e500, see_other_with_flash},
};

fn render_register_page(req: &HttpRequest) -> Result<String, anyhow::Error> {
    let tera = Tera::new("templates/**/*").context("Creating tera tamplate failed")?;
    let mut ctx = tera::Context::new();

    if let Some(flash_cookie) = req.cookie("_flash") {
        ctx.insert("flash_message", flash_cookie.value());
    }

    tera.render("auth/register.html", &ctx)
        .context("Cannot render register page")
}

#[actix_web::get("/register")]
pub async fn register_get(req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .cookie(delete_flash_cookie())
        .body(render_register_page(&req).map_err(e500)?))
}

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
