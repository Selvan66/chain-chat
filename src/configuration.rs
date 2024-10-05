use anyhow::Context;
use config::{Config, File, FileFormat};
use secrecy::Secret;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
    pub redis_uri: Secret<String>,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct ApplicationSettings {
    pub port: u16,
    pub key: Secret<String>,
    pub host: String,
}

#[derive(serde::Deserialize, Clone, Debug)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

pub fn get_configuration() -> Result<Settings, anyhow::Error> {
    dotenv::dotenv().context("Failed to load dotenv")?;

    let base_path = std::env::current_dir().context("Failed to determine the current directory")?;
    let config_dir = base_path.join("configuration");
    let config_base_file = config_dir.join("base");
    let environment = match std::env::var("APP_ENVIRONMENT") {
        Ok(s) => {
            if s == "production" {
                s
            } else {
                "local".into()
            }
        }
        Err(_) => "local".into(),
    };

    tracing::trace!("Configuration environment = {}", environment);

    let settings = Config::builder()
        .add_source(
            File::new(
                config_base_file
                    .to_str()
                    .context("Failed to find configuration file")?,
                FileFormat::Yaml,
            )
            .required(true),
        )
        .add_source(
            File::new(
                config_dir
                    .join(&environment)
                    .to_str()
                    .context("Failed to find configuration files: local and production")?,
                FileFormat::Yaml,
            )
            .required(true),
        )
        .add_source(
            config::Environment::with_prefix("app")
                .prefix_separator("__")
                .separator("__"),
        )
        .build()?;

    settings
        .try_deserialize()
        .context("Cannot deserialize settings")
}
