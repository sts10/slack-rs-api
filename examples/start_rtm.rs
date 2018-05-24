extern crate reqwest;
extern crate slack_api as slack;

use std::default::Default;
use std::env;

fn main() {
    let token = env::var("SLACK_API_TOKEN").expect("SLACK_API_TOKEN not set.");
    let client = slack::default_client().unwrap();

    {
        let request = slack::rtm::StartRequest::default();
        let response = slack::rtm::start(&client, &token, &request);

        if let Ok(response) = response {
            if let Some(channels) = response.channels {
                let channels = channels.into_iter().map(|c| c.name).collect::<Vec<_>>();

                println!("Got channels: {}", channels.join(", "));
            }

            if let Some(users) = response.users {
                let users = users.into_iter().map(|u| u.name).collect::<Vec<_>>();

                println!("Got users: {}", users.join(", "));
            }
        } else {
            println!("{:?}", response);
        }
    }
}
