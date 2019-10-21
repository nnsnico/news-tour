use news_tour::slack::{Params, Slack};

fn main() -> reqwest::Result<()> {
    let token = match Params::create_token("NEWS_TOUR_TOKEN") {
        Ok(v) => v,
        Err(err) => panic!("{}", err),
    };
    let url = "https://slack.com";
    let params = Params {
        token: token,
        text: "hogehoge".to_string(),
        channel: "#news".to_string(),
        as_user: true,
    };

    let slack = Slack {
        domain: url.to_string(),
        params: params,
    };

    let mut response = slack.post_message("/api/chat.postMessage".to_string())?;

    response.copy_to(&mut std::io::stdout())?;

    Ok(())
}
