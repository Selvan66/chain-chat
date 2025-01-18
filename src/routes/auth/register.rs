use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse};
use anyhow::Context;
use secrecy::Secret;
use sqlx::MySqlPool;
use tera::Tera;

use crate::{
    auth::{compute_password_hash, validate_register},
    database::users::add_user,
    domain::{messages::*, User},
    error::{e500, ValidationError},
    utils::{delete_flash_cookie, see_other_with_flash},
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
    email: String,
    password: Secret<String>,
    confirm_password: Secret<String>,
}

#[tracing::instrument(skip_all, fields(email = form.email))]
#[actix_web::post("/register")]
pub async fn register_post(
    form: web::Form<FormData>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse, ValidationError> {
    if let Err(e) =
        validate_register(&form.email, &form.password, &form.confirm_password, &pool).await
    {
        match e {
            ValidationError::ValidationError(e) => {
                tracing::info!("{}", e);
                return Ok(see_other_with_flash("/auth/register", &e.to_string()));
            }
            error => {
                tracing::error!("{}", error);
                return Err(error);
            }
        }
    }

    add_user(
        &pool,
        User {
            user_id: uuid::Uuid::new_v4(),
            email: form.email.to_string(),
            password_hash: compute_password_hash(form.password.clone())
                .map_err(ValidationError::UnexpectedError)?,
        },
    )
    .await
    .context("Cannot register user")
    .map_err(ValidationError::UnexpectedError)?;

    tracing::info!("User {} registered", form.email);

    Ok(see_other_with_flash("/", REGISTRATION_SUCCESSFUL))
}
