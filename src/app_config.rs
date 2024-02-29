use config::{Config, ConfigError, File};

#[derive(Debug)] 
pub struct AppConfig {
    // pub enable_api: i64,
    pub neodb_enable: bool,
    pub neodb_download: bool,
    pub neodb_username: String,
    pub neodb_token: String,
    pub bangumi_enable: bool,
    pub bangumi_download: bool,
    pub bangumi_username: String,
    pub bangumi_token: String,
}

impl AppConfig {
    pub fn from_file(file_path: &str) -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(File::with_name(file_path))
            .build()?;
        
        let neodb_enable = config.get_bool("neodb_enable")?;
        let neodb_download = config.get_bool("neodb_download")?;
        let neodb_username = config.get_string("neodb_username")?;
        let neodb_token = config.get_string("neodb_token")?;
        let bangumi_enable = config.get_bool("bangumi_enable")?;
        let bangumi_download = config.get_bool("bangumi_download")?;
        let bangumi_username = config.get_string("bangumi_username")?;
        let bangumi_token = config.get_string("bangumi_token")?;

        Ok(Self {
            neodb_enable,
            neodb_download,
            neodb_username,
            neodb_token,
            bangumi_enable,
            bangumi_download,
            bangumi_username,
            bangumi_token
        })
    }
}