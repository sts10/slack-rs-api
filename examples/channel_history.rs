extern crate reqwest;
extern crate slack_api as slack;

use std::env;

fn main() {
    let token = env::var("SLACK_API_TOKEN").expect("SLACK_API_TOKEN not set.");
    let client = slack::default_client().unwrap();

    let response = slack::channels::history(
        &client,
        &token,
        &slack::channels::HistoryRequest {
            channel: &env::args().nth(1).unwrap(),
            ..slack::channels::HistoryRequest::default()
        },
    );

    if let Ok(response) = response {
        println!("Got {} messages:", response.messages.len());
        for message in response.messages {
            println!("{:?}", message);
        }
    } else {
        println!("{:?}", response);
    }
}
