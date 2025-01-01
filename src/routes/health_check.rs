use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse};
use deadpool_redis::redis::cmd;
use deadpool_redis::Pool;
use sqlx::MySqlPool;

use crate::database;

#[actix_web::get("/health_check")]
pub async fn health_check(
    db_pool: web::Data<MySqlPool>,
    redis_pool: web::Data<Pool>,
) -> HttpResponse {
    if !database::init::health_check(&db_pool).await {
        tracing::error!("Database down");
        return HttpResponse::InternalServerError().finish();
    }
    if redis_health_check(&redis_pool).await.is_err() {
        tracing::error!("Redis down");
        return HttpResponse::InternalServerError().finish();
    }

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body("Yep its healthy")
}

async fn redis_health_check(redis_pool: &web::Data<Pool>) -> Result<(), anyhow::Error> {
    let mut conn = redis_pool.get().await?;
    let value: String = cmd("PING").query_async(&mut conn).await?;
    if value != "PONG" {
        return Err(anyhow::anyhow!("Redis wrong response"));
    }
    Ok(())
}
