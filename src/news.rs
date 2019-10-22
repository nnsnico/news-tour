use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct RequestNews {
    pub url: String,
    pub params: RequestParams,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestParams {
    pub country: String,
    pub category: String,
    pub page_size: usize,
    pub api_key: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseNews {
    pub status: String,
    pub total_results: u32,
    pub articles: Vec<Articles>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Articles {
    pub source: HashMap<String, Option<String>>,
    pub author: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub url_to_image: Option<String>,
    pub published_at: Option<String>,
    pub content: Option<String>,
}

impl RequestNews {
    pub fn get_topic(&self, endpoint: String) -> reqwest::Result<ResponseNews> {
        let client = reqwest::Client::new();
        let url = reqwest::Url::parse(&format!("{}{}", self.url, endpoint)).unwrap();

        println!("{}", url);

        client.get(url).query(&self.params).send()?.json()
    }
}
