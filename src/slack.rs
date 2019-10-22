use reqwest::header;
use serde::Serialize;

pub struct Slack {
    pub url: String,
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
        let url = reqwest::Url::parse(&format!("{}{}", self.url, endpoint)).unwrap();

        println!("{}", url);

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
