use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse};
use anyhow::Context;
use secrecy::Secret;
use sqlx::MySqlPool;

use crate::{
    auth::validate_credentials,
    domain::messages::AUTHENTICATION_FAILED,
    error::e500,
    error::ValidationError,
    session::UserSession,
    utils::{delete_flash_cookie, see_other, see_other_with_flash},
};

fn render_login_page(req: &HttpRequest) -> Result<String, anyhow::Error> {
    let tera = tera::Tera::new("templates/**/*").context("Creating tera tamplate failed")?;
    let mut ctx = tera::Context::new();

    if let Some(flash_cookie) = req.cookie("_flash") {
        ctx.insert("flash_message", flash_cookie.value());
    }

    tera.render("auth/login.html", &ctx)
        .context("Cannot render login page")
}

#[actix_web::get("/login")]
pub async fn login_get(req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .cookie(delete_flash_cookie())
        .body(render_login_page(&req).map_err(e500)?))
}

#[derive(serde::Deserialize, Debug)]
struct FormData {
    username: String,
    password: Secret<String>,
}

#[tracing::instrument(
    skip_all,
    fields(username = form.username)
)]
#[actix_web::post("/login")]
pub async fn login_post(
    form: web::Form<FormData>,
    pool: web::Data<MySqlPool>,
    session: UserSession,
) -> Result<HttpResponse, ValidationError> {
    match validate_credentials(form.0.username, form.0.password, &pool).await {
        Ok(user_id) => {
            tracing::info!("User {} login!", user_id);
            session.renew();
            tracing::debug!("Insert user_id {} to session", user_id);
            session
                .insert_user_id(user_id)
                .map_err(|e| ValidationError::UnexpectedError(e.into()))?;
            Ok(see_other("/user/info"))
        }
        Err(e) => match e {
            ValidationError::ValidationError(e) => {
                tracing::info!("{}", e);
                Ok(see_other_with_flash("/auth/login", AUTHENTICATION_FAILED))
            }
            ValidationError::UnexpectedError(e) => {
                tracing::error!("Authentication error: {}", e);
                Err(ValidationError::UnexpectedError(e))
            }
        },
    }
}
