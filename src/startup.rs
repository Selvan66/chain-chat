use std::net::TcpListener;

use actix_files::Files;
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::dev::Server;
use actix_web::middleware::{from_fn, ErrorHandlers, Logger};
use actix_web::{web, App, HttpServer};
use anyhow::Context;
use deadpool_redis::Pool;
use secrecy::ExposeSecret;
use sqlx::MySqlPool;

use crate::config::Settings;
use crate::database::init::{connection_with_db, get_db_pool};
use crate::middleware::{error_handler, reject_anonymous_users, reject_logged_users};
use crate::routes::{
    change_password_get, change_password_post, health_check, home_get, info_get, login_get,
    login_post, logout_post, register_get, register_post,
};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        tracing::debug!("SETTING UP MYSQL CONNECTION");
        let mysql_connection = connection_with_db(&configuration.database);
        let mysql_pool = get_db_pool(mysql_connection);

        tracing::debug!("SETTING UP REDIS POOL");
        let cfg = deadpool_redis::Config::from_url(configuration.redis_uri.expose_secret());
        let redis_pool = cfg
            .create_pool(Some(deadpool_redis::Runtime::Tokio1))
            .context("Cannot create deadpool redis")?;

        tracing::debug!("SETTING UP REDIS SESSION");
        let redis_store = RedisSessionStore::new_pooled(redis_pool.clone()).await?;
        let secret_key = Key::from(configuration.application.key.expose_secret().as_bytes());

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        tracing::debug!("TcpListener bind address: {}", address);
        let listener = TcpListener::bind(address).context("Failed to bind port")?;
        let port = listener.local_addr().unwrap().port();
        tracing::debug!("Get local port: {}", port);

        let server = run(listener, mysql_pool, redis_pool, redis_store, secret_key).await?;

        Ok(Self { port, server })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

async fn run(
    listener: TcpListener,
    db_pool: MySqlPool,
    redis_pool: Pool,
    redis_store: RedisSessionStore,
    secret_key: Key,
) -> Result<Server, anyhow::Error> {
    let db_pool = web::Data::new(db_pool);
    let redis_pool = web::Data::new(redis_pool);

    tracing::debug!("CREATE SERVER");
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%{r}a %r %s %{Location}o").exclude("/health_check"))
            .wrap(SessionMiddleware::new(
                redis_store.clone(),
                secret_key.clone(),
            ))
            .wrap(ErrorHandlers::new().default_handler(error_handler))
            .service(home_get)
            .service(health_check)
            .service(
                web::scope("/auth")
                    .wrap(from_fn(reject_logged_users))
                    .service(login_get)
                    .service(login_post)
                    .service(register_get)
                    .service(register_post),
            )
            .service(
                web::scope("/user")
                    .wrap(from_fn(reject_anonymous_users))
                    .service(info_get)
                    .service(logout_post)
                    .service(change_password_get)
                    .service(change_password_post),
            )
            .service(Files::new("/", "./static"))
            .app_data(db_pool.clone())
            .app_data(redis_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
