use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Slack {
    pub scheme: String,
    pub domain: String,
    pub token_key: String,
}

#[derive(Debug, Deserialize)]
pub struct Bot {
    pub channel: String,
    pub as_user: bool,
}

#[derive(Debug, Deserialize)]
pub struct Endpoint {
    pub post_message: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub slack: Slack,
    pub bot: Bot,
    pub endpoint: Endpoint,
}

impl Settings {
    pub fn create_new() -> Result<Settings, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name("config/slack"))?;
        s.try_into()
    }
}
