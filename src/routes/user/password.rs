use actix_web::{http::header::ContentType, HttpRequest, HttpResponse};
// use actix_web::web;
use anyhow::Context;
// use secrecy::Secret;
// use sqlx::MySqlPool;
use tera::Tera;

use crate::{
    // session::UserId,
    utils::{delete_flash_cookie, e500},
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
pub async fn password_get(req: HttpRequest) -> Result<HttpResponse, actix_web::Error> {
    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .cookie(delete_flash_cookie())
        .body(render_password_page(&req).map_err(e500)?))
}
//
// #[derive(serde::Deserialize, Debug)]
// struct FormData {
//     old_password: Secret<String>,
//     new_password: Secret<String>,
//     confirm_new_password: Secret<String>,
// }
//
// #[tracing::instrument(skip_all, fields(user_id=%&*user_id))]
// #[actix_web::post("/password")]
// pub async fn change_password_post(
//     form: web::Form<FormData>,
//     user_id: web::ReqData<UserId>,
//     pool: web::Data<MySqlPool>,
// ) -> Result<HttpResponse, actix_web::Error> {
//     todo!()
// }
