use config::{Config, File, FileFormat};

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let config_dir = base_path.join("configuration");
    let config_base_file = config_dir.join("base");

    let settings = Config::builder()
        .add_source(File::new(
            config_base_file
                .to_str()
                .expect("Failed to find configuration file"),
            FileFormat::Yaml,
        ))
        .build()?;

    settings.try_deserialize()
}
