use actix_web::{http::header::ContentType, HttpRequest, HttpResponse};
use anyhow::Context;
use tera::Tera;

fn render_home_page(_req: &HttpRequest) -> Result<String, anyhow::Error> {
    let tera = Tera::new("templates/**/*").context("Creating tera tamplate failed")?;
    let ctx = tera::Context::new();

    // TODO: Get cookie

    Ok(tera
        .render("home.html", &ctx)
        .context("Cannot render home page")?)
}

#[actix_web::get("/")]
pub async fn home_get(req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(render_home_page(&req).expect("Cannot render home page"))
}
