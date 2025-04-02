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

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password,
            self.host,
            self.port,
            self.database_name
        )
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    // Initialise our configuration reader
    let builder = config::Config::builder();

    // Add configuration values from a file name `configuration`.
    // It will look for any top-level file with an extension
    // that `config` knows to parse: yaml, json, etc.
    let settings = builder
        .add_source(config::File::with_name("configuration"))
        .build()?;
    // settings.merge(config::File::with_name("configuration"))?;

    // Try to convert the configuration values ir tead into
    // our Settings type
    settings.try_deserialize()
}
