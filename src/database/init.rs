use secrecy::ExposeSecret;
use sqlx::mysql::{MySqlConnectOptions, MySqlPool, MySqlPoolOptions, MySqlSslMode};
use sqlx::ConnectOptions;

use crate::configuration::DatabaseSettings;

pub fn connection_without_db(settings: &DatabaseSettings) -> MySqlConnectOptions {
    let ssl_mode = if settings.require_ssl {
        MySqlSslMode::Required
    } else {
        MySqlSslMode::Preferred
    };

    MySqlConnectOptions::new()
        .host(&settings.host)
        .username(&settings.username)
        .password(settings.password.expose_secret())
        .port(settings.port)
        .ssl_mode(ssl_mode)
        .log_statements(tracing::log::LevelFilter::Trace)
}

pub fn connection_with_db(settings: &DatabaseSettings) -> MySqlConnectOptions {
    connection_without_db(settings).database(&settings.database_name)
}

pub fn get_db_pool(connection: MySqlConnectOptions) -> MySqlPool {
    MySqlPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(connection)
}
