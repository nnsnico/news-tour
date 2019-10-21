use reqwest::header;
use serde::Serialize;
use std::env;
use std::result::Result;

pub struct Slack {
    pub domain: String,
    pub params: Params,
}

#[derive(Serialize, Debug)]
pub struct Params {
    pub token: String,
    pub text: String,
    pub channel: String,
    pub as_user: bool,
}

impl Slack {
    pub fn post_message(&self, endpoint: String) -> reqwest::Result<reqwest::Response> {
        let client = reqwest::Client::new();
        let url = reqwest::Url::parse(&format!("{}{}", self.domain, endpoint)).unwrap();
        println!("{}", format!("{}{}", self.domain, endpoint));
        client
            .post(url)
            .header(
                header::AUTHORIZATION,
                format!("Bearer {}", self.params.token),
            )
            .json(&self.params)
            .send()
    }
}

impl Params {
    pub fn create_token(name: &str) -> Result<String, String> {
        match env::var(name) {
            Ok(v) => Ok(v),
            Err(err) => Err(err.to_string()),
        }
    }
}
