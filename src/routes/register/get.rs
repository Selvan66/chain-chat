use actix_web::{http::header::ContentType, HttpResponse};

#[actix_web::get("/register")]
pub async fn register() -> HttpResponse {
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(include_str!("register.html"))
}
