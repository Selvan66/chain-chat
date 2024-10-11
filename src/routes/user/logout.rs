use actix_web::web;
use actix_web::HttpResponse;

use crate::domain::messages::*;
use crate::session::{UserId, UserSession};
use crate::utils::see_other_with_flash;

#[tracing::instrument(
    skip_all,
    fields(user_id=%&*user_id)
)]
#[actix_web::post("/logout")]
pub async fn logout_post(session: UserSession, user_id: web::ReqData<UserId>) -> HttpResponse {
    session.log_out();
    tracing::info!("Logout!");
    see_other_with_flash("/", LOGOUT_MESSAGE)
}
