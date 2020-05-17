use news_tour::news::request::*;
use news_tour::news::response::*;
use news_tour::settings::Settings;
use news_tour::slack::*;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::create_new()?;

    let news_params = RequestParams::new(
        settings.api.country,
        settings.api.category,
        settings.api.page_size,
        env::var(&settings.api.api_key)?,
    );
    let news = RequestNews::new(settings.api.scheme, settings.api.domain, news_params);

    let news_response: ResponseNews = news.get_topic(settings.api.endpoint)?;
    let filtered_response: ResponseNews =
        news_response.filter_by_source(settings.api.exclude_news_domain);

    let slack_params = SlackParams::new(
        env::var(&settings.slack.token_key)?,
        filtered_response.create_summary(),
        settings.bot.channel,
        settings.bot.as_user,
    );
    let slack = Slack::new(settings.slack.scheme, settings.slack.domain, slack_params);

    slack.post_message(settings.slack.post_message_endpoint)?;

    Ok(())
}
