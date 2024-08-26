use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{App, HttpServer};
use tracing_actix_web::TracingLogger;

use crate::configuration::Settings;
use crate::routes::{health_check, home};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        let listener = TcpListener::bind(address).expect("Failed to bind port");
        let port = listener.local_addr().unwrap().port();

        let server = run(listener).await?;

        Ok(Self { port, server })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }

    pub fn port(&self) -> u16 {
        self.port
    }
}

async fn run(listener: TcpListener) -> Result<Server, anyhow::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(home)
            .service(health_check)
    })
    .listen(listener)?
    .run();

    Ok(server)
}
