use actix_web::http::header::LOCATION;
use actix_web::HttpResponse;

pub fn see_other(location: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, location))
        .finish()
}

pub fn e500<T>(e: T) -> actix_web::Error
where
    T: std::fmt::Debug + std::fmt::Display + 'static,
{
    tracing::error!("{}", e);
    actix_web::error::ErrorInternalServerError(e)
}
