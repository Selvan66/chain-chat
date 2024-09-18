use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use anyhow::Context;
use sqlx::MySqlPool;

use crate::configuration::Settings;
use crate::database::init::{connection_with_db, get_db_pool};
use crate::routes::{health_check, home_get, login, register_get, register_post};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let mysql_connection = connection_with_db(&configuration.database);
        let mysql_pool = get_db_pool(mysql_connection);

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let listener = TcpListener::bind(address).context("Failed to bind port")?;
        let port = listener.local_addr().unwrap().port();

        let server = run(listener, mysql_pool).await?;

        Ok(Self { port, server })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

async fn run(listener: TcpListener, db_pool: MySqlPool) -> Result<Server, anyhow::Error> {
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::new("%{r}a %r %s %{Location}o"))
            .service(home_get)
            .service(health_check)
            .service(login)
            .service(register_get)
            .service(register_post)
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
