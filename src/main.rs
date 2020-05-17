use news_tour::news::request::*;
use news_tour::news::response::*;
use news_tour::settings::Settings;
use news_tour::slack::*;
use std::env;

fn main() {
    let settings = Settings::create_new().unwrap();

    let news_params = RequestParams::new(
        settings.api.country,
        settings.api.category,
        settings.api.page_size,
        env::var(&settings.api.api_key).unwrap(),
    );
    let news = RequestNews::new(settings.api.scheme, settings.api.domain, news_params);

    let news_response: ResponseNews = news.get_topic(settings.api.endpoint).unwrap();
    let filtered_response: ResponseNews =
        news_response.filter_by_source(settings.api.exclude_news_domain);

    let slack_params = SlackParams::new(
        env::var(&settings.slack.token_key).unwrap(),
        filtered_response.create_summary(),
        settings.bot.channel,
        settings.bot.as_user,
    );
    let slack = Slack::new(settings.slack.scheme, settings.slack.domain, slack_params);

    let slack_response = slack.post_message(settings.slack.post_message_endpoint);

    if let Err(err) = slack_response {
        println!("Failed: {}", err);
    }
}
