use actix_web::{web, HttpResponse};
use sqlx::MySqlPool;

use crate::database;

#[actix_web::get("/health_check")]
pub async fn health_check(pool: web::Data<MySqlPool>) -> HttpResponse {
    if !database::init::health_check(&pool).await {
        tracing::error!("Database down");
        return HttpResponse::InternalServerError().finish();
    }
    HttpResponse::Ok().finish()
}
