use news_tour::settings::Settings;
use news_tour::slack::{Params, Slack};

fn main() {
    let settings = Settings::create_new().unwrap();

    let token = match Params::create_token(&settings.slack.token_key) {
        Ok(v) => v,
        Err(err) => panic!("{}", err),
    };
    let url = format!("{}://{}", settings.slack.scheme, settings.slack.domain);
    let params = Params {
        token: token,
        text: "This is test".to_string(),
        channel: settings.bot.channel,
        as_user: settings.bot.as_user,
    };
    let slack = Slack {
        domain: url,
        params: params,
    };

    let response = slack.post_message(settings.endpoint.post_message);

    match response {
        Ok(_) => println!("Success!!"),
        Err(err) => println!("Failed: {}", err),
    }
}
