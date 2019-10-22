use news_tour::news;
use news_tour::news::{RequestNews, ResponseNews};
use news_tour::settings::Settings;
use news_tour::slack;
use news_tour::slack::Slack;
use std::env;

fn main() {
    let settings = Settings::create_new().unwrap();

    let news_params = news::RequestParams {
        country: settings.api.country,
        category: settings.api.category,
        page_size: settings.api.page_size,
        api_key: env::var(&settings.api.api_key).unwrap(),
    };
    let news = RequestNews {
        url: format!("{}://{}", settings.api.scheme, settings.api.domain),
        params: news_params,
    };

    let news_response: ResponseNews = news.get_topic(settings.api.endpoint).unwrap();

    println!("news:\n {:?}", news_response);

    let slack_params = slack::Params {
        token: env::var(&settings.slack.token_key).unwrap(),
        text: format!(
            "{}\nPowered by News API\n{}",
            news_response.articles[0].title,
            news_response.articles[0].url.as_ref().unwrap()
        ),
        channel: settings.bot.channel,
        as_user: settings.bot.as_user,
    };
    let slack = Slack {
        url: format!("{}://{}", settings.slack.scheme, settings.slack.domain),
        params: slack_params,
    };

    let slack_response = slack.post_message(settings.slack.post_message_endpoint);

    match slack_response {
        Ok(_) => println!("Success!!"),
        Err(err) => println!("Failed: {}", err),
    }
}
