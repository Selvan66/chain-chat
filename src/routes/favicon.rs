use actix_files::NamedFile;

#[actix_web::get("/favicon.ico")]
pub async fn favicon_get() -> impl actix_web::Responder {
    NamedFile::open_async("static/favicon.ico").await
}
