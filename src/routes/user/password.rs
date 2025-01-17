use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse};
use anyhow::Context;
use secrecy::{ExposeSecret, Secret};
use sqlx::MySqlPool;
use tera::Tera;

use crate::{
    auth::{change_password, validate_credentials},
    database::users::get_username,
    domain::messages::*,
    error::e500,
    session::UserId,
    utils::{delete_flash_cookie, see_other_with_flash},
};

fn render_password_page(req: &HttpRequest) -> Result<String, anyhow::Error> {
    let tera = Tera::new("templates/**/*").context("Creating tera tamplate failed")?;
    let mut ctx = tera::Context::new();

    if let Some(flash_cookie) = req.cookie("_flash") {
        ctx.insert("flash_message", flash_cookie.value());
    }

    tera.render("user/password.html", &ctx)
        .context("Cannot render password page")
}

#[actix_web::get("/password")]
pub async fn change_password_get(req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .cookie(delete_flash_cookie())
        .body(render_password_page(&req).map_err(e500)?))
}

#[derive(serde::Deserialize, Debug)]
struct FormData {
    old_password: Secret<String>,
    new_password: Secret<String>,
    confirm_new_password: Secret<String>,
}

#[tracing::instrument(skip_all, fields(user_id=%&*user_id))]
#[actix_web::post("/password")]
pub async fn change_password_post(
    form: web::Form<FormData>,
    user_id: web::ReqData<UserId>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse, actix_web::Error> {
    if form.new_password.expose_secret() != form.confirm_new_password.expose_secret() {
        tracing::warn!(PASSWORD_CHANGE_FAILED_NOT_EQ_CONFIRM);
        return Ok(see_other_with_flash(
            "/user/password",
            PASSWORD_CHANGE_FAILED_NOT_EQ_CONFIRM,
        ));
    }

    if form.new_password.expose_secret().len() < 4 {
        tracing::warn!(PASSWORD_CHANGE_FAILED_PASSWORD_TOO_SHORT);
        return Ok(see_other_with_flash(
            "/user/password",
            PASSWORD_CHANGE_FAILED_PASSWORD_TOO_SHORT,
        ));
    }

    let user_id = user_id.into_inner();
    let username = get_username(&pool, &user_id).await.map_err(e500)?;
    let current_password = form.old_password.clone();

    if validate_credentials(username, current_password, &pool)
        .await
        .is_err()
    {
        tracing::warn!(PASSWORD_CHANGE_FAILED_CURRENT_PASSWORD_WRONG);
        return Ok(see_other_with_flash(
            "/user/password",
            PASSWORD_CHANGE_FAILED_CURRENT_PASSWORD_WRONG,
        ));
    }

    change_password(user_id.0, form.0.new_password, &pool)
        .await
        .map_err(e500)?;

    tracing::info!(CHANGE_PASSWORD_SUCCESSFUL);
    Ok(see_other_with_flash(
        "/user/info",
        CHANGE_PASSWORD_SUCCESSFUL,
    ))
}
