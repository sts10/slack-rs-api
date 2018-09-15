extern crate reqwest;
extern crate slack_api as slack;
use std::error::Error;

fn main() {
    let token = ::std::env::var("SLACK_API_TOKEN").expect("SLACK_API_TOKEN not set.");
    let client = ::reqwest::Client::new();

    {
        let request = slack::rtm::StartRequest::new();
        let response = slack::rtm::start(&client, &token, &request);

        match response {
            Ok(response) => {
                //println!("{:#?}", response.others);
                if let Some(channels) = response.channels {
                    let channels = channels.into_iter().map(|c| c.name).collect::<Vec<_>>();

                    println!("Got channels: {}", channels.join(", "));
                }

                if let Some(users) = response.users {
                    let users = users.into_iter().map(|u| u.name).collect::<Vec<_>>();

                    println!("Got users: {}", users.join(", "));
                }
            }
            Err(e) => {
                println!("{}\n{:?}", e.description(), e.cause());
            }
        }
    }
}
