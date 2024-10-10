use actix_web::HttpResponse;

use crate::domain::messages::*;
use crate::session::UserSession;
use crate::utils::see_other_with_flash;

#[actix_web::post("/logout")]
pub async fn logout_post(session: UserSession) -> HttpResponse {
    session.log_out();
    see_other_with_flash("/", LOGOUT_MESSAGE)
}
