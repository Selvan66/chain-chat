use actix_web::{web, HttpResponse};
use sqlx::MySqlPool;
use secrecy::Secret;



#[derive(serde::Deserialize, Debug)]
struct FormData {
    username: String,
    password: Secret<String>
}

#[tracing::instrument(
    skip_all, 
    fields(username = form.username)
)]
#[actix_web::post("/login")]
pub async fn login_post(form: web::Form<FormData>, pool: web::Data<MySqlPool>) -> Result<HttpResponse, actix_web::Error> {
todo!()
}
