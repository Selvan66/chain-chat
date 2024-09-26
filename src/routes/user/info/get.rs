use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse};
use anyhow::Context;
use sqlx::MySqlPool;

use crate::{
    database::users::get_username,
    session::UserId,
    utils::{delete_flash_cookie, e500},
};

fn render_info_page(req: &HttpRequest, username: &str) -> Result<String, anyhow::Error> {
    let tera = tera::Tera::new("templates/**/*").context("Creating tera tamplate failed")?;
    let mut ctx = tera::Context::new();

    if let Some(flash_cookie) = req.cookie("_flash") {
        ctx.insert("flash_message", flash_cookie.value());
    }

    ctx.insert("username", username);

    Ok(tera
        .render("user/info.html", &ctx)
        .context("Cannot render login page")?)
}

#[actix_web::get("/info")]
pub async fn info_get(
    req: HttpRequest,
    user_id: web::ReqData<UserId>,
    pool: web::Data<MySqlPool>,
) -> Result<HttpResponse, actix_web::Error> {
    let username = get_username(&pool, &user_id).await.map_err(e500)?;

    Ok(HttpResponse::Ok()
        .content_type(ContentType::html())
        .cookie(delete_flash_cookie())
        .body(render_info_page(&req, &username).expect("Cannot render login page")))
}
