use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseNews {
    status: String,
    total_results: u32,
    articles: Vec<Article>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    source: Source,
    author: Option<String>,
    title: String,
    description: Option<String>,
    url: Option<String>,
    url_to_image: Option<String>,
    published_at: Option<String>,
    content: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Source {
    id: Option<String>,
    name: String,
}

impl ResponseNews {
    pub fn create_summary(&self) -> String {
        format!(
            "{}\nPowered by News API\n{}",
            self.articles[0].title,
            self.articles[0].url.as_ref().unwrap()
        )
    }

    pub fn filter_by_source(&self, ignore_sources: Vec<String>) -> ResponseNews {
        ResponseNews {
            status: self.status.clone(),
            total_results: self.total_results,
            articles: (self
                .articles
                .clone()
                .into_iter()
                .filter(|article| !ignore_sources.contains(&article.source.name))
                .collect::<Vec<Article>>())
            .to_vec(),
        }
    }
}
