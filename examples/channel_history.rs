extern crate chrono;
extern crate openssl_probe;
extern crate slack_api as slack;

use std::env;

fn main() {
    openssl_probe::init_ssl_cert_env_vars();

    let token = env::var("SLACK_API_TOKEN").expect("SLACK_API_TOKEN not set.");
    let client = slack::default_client();

    let response = slack::channels::list(&client, &token, &slack::channels::ListRequest::default()).unwrap();

    for channel in response.channels {
        println!("{}, {}", channel.id, channel.name);
        let response = slack::channels::history(
            &client,
            &token,
            &slack::channels::HistoryRequest {
                channel: channel.id,
                count: Some(1000),
                ..slack::channels::HistoryRequest::default()
            },
        );
    }
}
