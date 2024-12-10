use actix_files::NamedFile;

#[actix_web::get("/favicon.ico")]
pub async fn favicon_get() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("static/favicon.ico")?)
}
