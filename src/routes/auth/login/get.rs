use actix_web::{http::header::ContentType, HttpRequest, HttpResponse};
use anyhow::Context;

use crate::utils::delete_flash_cookie;

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
pub async fn login_get(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .cookie(delete_flash_cookie())
        .body(render_login_page(&req).expect("Cannot render login page"))
}
