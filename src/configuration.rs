#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // init
    let mut settings = config::Config::default();

    // get from yaml
    settings.merge(config::File::with_name("configuration"))?;
    // Try to convert the configuration values it read into
    // our Settings type
    settings.try_into()
}
