use actix_web::http::header::LOCATION;
use actix_web::HttpResponse;

pub fn see_other(location: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, location))
        .finish()
}
