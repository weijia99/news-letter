


#[derive(serde::Deserialize)]
// application configuration settings
pub struct Settings{
    pub database: DatabaseSettings,
    pub application_port: u16,

}
#[derive(serde::Deserialize)]
pub struct DatabaseSettings{
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings,config::ConfigError>{
    let mut settings = config::Config::default();
    // Read the file with the configuration
    settings.merge(config::File::with_name("configuration"))?;
    // Try to convert the configuration values it read into our Settings type
    settings.try_into()
}
impl DatabaseSettings{
    pub fn connection_string(&self) -> String{
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password,
            self.host,
            self.port,
            self.database_name
        )
    }
    pub fn connection_string_without_db(&self) -> String {
        format!(
        "postgres://{}:{}@{}:{}",
        self.username, self.password, self.host, self.port
        )
        }
        
}