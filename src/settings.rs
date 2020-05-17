use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Slack {
    pub scheme: String,
    pub domain: String,
    pub post_message_endpoint: String,
    pub token_key: String,
}

#[derive(Debug, Deserialize)]
pub struct Bot {
    pub channel: String,
    pub as_user: bool,
}

#[derive(Debug, Deserialize)]
pub struct Api {
    pub scheme: String,
    pub domain: String,
    pub endpoint: String,
    pub country: String,
    pub category: String,
    pub exclude_news_domain: Vec<String>,
    pub page_size: usize,
    pub api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub slack: Slack,
    pub bot: Bot,
    pub api: Api,
}

impl Settings {
    pub fn create_new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name("config/slack"))?;
        s.merge(File::with_name("config/news"))?;
        s.try_into()
    }
}
