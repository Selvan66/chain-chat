use actix_web::cookie::Cookie;

pub fn create_flash_cookie<'a>(value: &'a str) -> Cookie<'a> {
    // TODO: crypt the value of the cookie

    Cookie::build("_flash", value)
        .secure(true)
        .http_only(true)
        .finish()
}

pub fn delete_flash_cookie<'a>() -> Cookie<'a> {
    let mut cookie = Cookie::build("_flash", "")
        .secure(true)
        .http_only(true)
        .finish();

    cookie.make_removal();
    cookie
}
