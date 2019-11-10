use serde::{Deserialize, Serialize};

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
    pub articles: Vec<Article>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub source: Source,
    pub author: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub url_to_image: Option<String>,
    pub published_at: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Source {
    pub id: Option<String>,
    pub name: String,
}

impl RequestNews {
    pub fn get_topic(&self, endpoint: String) -> reqwest::Result<ResponseNews> {
        let client = reqwest::Client::new();
        let url = reqwest::Url::parse(&format!("{}{}", self.url, endpoint)).unwrap();

        println!("{}", url);

        client.get(url).query(&self.params).send()?.json()
    }
}

impl ResponseNews {
    pub fn filter_by_source(&self, ignore_source: Vec<String>) -> ResponseNews {
        ResponseNews {
            status: self.status.clone(),
            total_results: self.total_results,
            articles: (*self
                .articles
                .clone()
                .into_iter()
                .filter(|article| !ignore_source.contains(&article.source.name))
                .collect::<Vec<Article>>())
            .to_vec(),
        }
    }
}
