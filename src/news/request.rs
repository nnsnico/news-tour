use super::response::ResponseNews;
use serde::Serialize;

pub struct RequestNews {
    url: String,
    params: RequestParams,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestParams {
    country: String,
    category: String,
    page_size: usize,
    api_key: String,
}

impl RequestNews {
    pub fn new(scheme: String, domain: String, params: RequestParams) -> Self {
        let url = format!("{}://{}", scheme, domain);
        RequestNews { url, params }
    }

    pub async fn get_topic(&self, endpoint: String) -> reqwest::Result<ResponseNews> {
        let client = reqwest::Client::new();
        let url = reqwest::Url::parse(&format!("{}{}", self.url, endpoint)).unwrap();

        client
            .get(url)
            .query(&self.params)
            .send()
            .await?
            .json()
            .await
    }
}

impl RequestParams {
    pub fn new(country: String, category: String, page_size: usize, api_key: String) -> Self {
        RequestParams {
            country,
            category,
            page_size,
            api_key,
        }
    }
}
