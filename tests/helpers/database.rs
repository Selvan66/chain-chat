use sqlx::mysql::{MySqlConnectOptions, MySqlPool};
use sqlx::{ConnectOptions, Executor};

pub async fn configure_database(
    connection_options: MySqlConnectOptions,
    database_name: String,
) -> MySqlPool {
    let mut connection = connection_options
        .clone()
        .connect()
        .await
        .expect("Failed to connect to MySQL");

    connection
        .execute(format!(r#"CREATE DATABASE `{}`;"#, database_name).as_str())
        .await
        .expect("Failed to create database");

    let db_pool = MySqlPool::connect_with(connection_options.database(&database_name))
        .await
        .expect("Failed to connect MySQL pool");

    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to migrate the database");

    db_pool
}
