extern crate reqwest;
extern crate slack_api as slack;
use slack::http::channels;

use std::env;

fn main() {
    let token = env::var("SLACK_API_TOKEN").expect("SLACK_API_TOKEN not set.");
    let client = ::reqwest::Client::new();

    let response = channels::list(&client, &token, &channels::ListRequest::new()).unwrap();

    for channel in response.channels {
        println!("{}, {}", channel.id, channel.name);
        let _response = channels::history(&client, &token, &channels::HistoryRequest::new(channel.id));
    }
}
