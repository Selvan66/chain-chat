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

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
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

    tracing::trace!("configuration environment = {}", environment);

    let settings = Config::builder()
        .add_source(File::new(
            config_base_file
                .to_str()
                .expect("Failed to find configuration file"),
            FileFormat::Yaml,
        ))
        .add_source(File::new(
            config_dir
                .join(&environment)
                .to_str()
                .expect("Failed to find configuration files: local and production"),
            FileFormat::Yaml,
        ))
        .add_source(config::Environment::with_prefix("APP").separator("__"))
        .build()?;

    settings.try_deserialize()
}
