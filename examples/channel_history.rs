extern crate openssl_probe;
extern crate reqwest;
extern crate slack_api as slack;

use std::env;

fn main() {
    openssl_probe::init_ssl_cert_env_vars();

    let token = env::var("SLACK_API_TOKEN").expect("SLACK_API_TOKEN not set.");
    let client = ::reqwest::Client::new();

    let response = slack::channels::list(&client, &token, &slack::channels::ListRequest::new()).unwrap();

    for channel in response.channels {
        println!("{}, {}", channel.id, channel.name);
        let _response = slack::channels::history(&client, &token, &slack::channels::HistoryRequest::new(channel.id));
    }
}
