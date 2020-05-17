use reqwest::header;
use serde::Serialize;

pub struct Slack {
    url: String,
    params: SlackParams,
}

#[derive(Serialize, Debug)]
pub struct SlackParams {
    token: String,
    text: String,
    channel: String,
    as_user: bool,
}

impl Slack {
    pub fn new(scheme: String, domain: String, params: SlackParams) -> Self {
        let url = format!("{}://{}", scheme, domain);
        Slack { url, params }
    }

    pub fn post_message(&self, endpoint: String) -> reqwest::Result<reqwest::Response> {
        let client = reqwest::Client::new();
        let url = reqwest::Url::parse(&format!("{}{}", self.url, endpoint)).unwrap();

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

impl SlackParams {
    pub fn new(token: String, text: String, channel: String, as_user: bool) -> Self {
        SlackParams {
            token,
            text,
            channel,
            as_user,
        }
    }
}
