use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse};
use anyhow::Context;
use secrecy::Secret;
use sqlx::MySqlPool;
use tera::Tera;

use crate::{
    auth::{change_password, validate_credentials, validate_password_and_confirm},
    database::users::get_email,
    domain::messages::*,
    error::{e500, ValidationError},
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
) -> Result<HttpResponse, ValidationError> {
    let user_id = user_id.into_inner();

    if let Err(ValidationError::ValidationError(e)) =
        validate_password_and_confirm(&form.new_password, &form.confirm_new_password)
    {
        tracing::info!("{}", e);
        return Ok(see_other_with_flash("/user/password", &e.to_string()));
    }

    let email = get_email(&pool, &user_id)
        .await
        .map_err(ValidationError::UnexpectedError)?;

    let current_password = form.old_password.clone();

    if let Err(e) = validate_credentials(email, current_password, &pool).await {
        match e {
            ValidationError::ValidationError(e) => {
                tracing::info!("{}", e);
                return Ok(see_other_with_flash("/user/password", &e.to_string()));
            }
            error => {
                tracing::error!("{}", error);
                return Err(error);
            }
        }
    }

    change_password(user_id.0, form.0.new_password, &pool)
        .await
        .map_err(ValidationError::UnexpectedError)?;

    tracing::info!(CHANGE_PASSWORD_SUCCESSFUL);
    Ok(see_other_with_flash(
        "/user/info",
        CHANGE_PASSWORD_SUCCESSFUL,
    ))
}
