use actix_web::cookie::{Cookie, SameSite};
use actix_web::http::header::LOCATION;
use actix_web::HttpResponse;

pub fn see_other(location: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, location))
        .finish()
}

pub fn see_other_with_flash(location: &str, flash_message: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, location))
        .cookie(create_flash_cookie(flash_message))
        .finish()
}

pub fn create_flash_cookie(value: &str) -> Cookie {
    Cookie::build("_flash", value)
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict)
        .finish()
}

pub fn delete_flash_cookie<'a>() -> Cookie<'a> {
    let mut cookie = Cookie::build("_flash", "")
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict)
        .finish();

    cookie.make_removal();
    cookie
}
