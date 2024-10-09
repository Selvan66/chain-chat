use actix_web::{web, HttpResponse};
use secrecy::Secret;
use sqlx::MySqlPool;

use crate::{
    cryptografic::validate_login,
    domain::messages::AUTHENTICATION_FAILED,
    session::UserSession,
    utils::{e500, see_other, see_other_with_flash},
};

#[derive(serde::Deserialize, Debug)]
struct FormData {
    username: String,
    password: Secret<String>,
}

#[tracing::instrument(
    skip_all,
    fields(username = form.username)
)]
#[actix_web::post("/login")]
pub async fn login_post(
    form: web::Form<FormData>,
    pool: web::Data<MySqlPool>,
    session: UserSession,
) -> Result<HttpResponse, actix_web::Error> {
    match validate_login(form.0.username, form.0.password, &pool).await {
        Ok(user_id) => {
            tracing::info!("User {} login!", user_id);
            session.renew();
            session.insert_user_id(user_id).map_err(e500)?;
            Ok(see_other("/user/info"))
        }
        Err(e) => {
            tracing::error!("Error while login: {}", e.to_string());
            Ok(see_other_with_flash("/auth/login", AUTHENTICATION_FAILED))
        }
    }
}
