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
        for message in response.unwrap().messages {
            if let ::slack::Message::Standard(::slack::MessageStandard { ts: Some(ts), .. }) = message {
                let datetime: ::chrono::DateTime<::chrono::Utc> = ts.into();
                println!("{:?}", datetime.timestamp_subsec_millis());
            }
        }
    }

    /*
    use std::collections::BTreeMap;
    let mut total_time = 0;
    let mut total_messages = 0;
    let mut results = BTreeMap::new();
    for span in &slack::flame::spans() {
        let entry = results.entry(span.name.to_string()).or_insert((0, 0));
        entry.0 += span.delta;
        total_time += span.delta;
        entry.1 += 1;
        if span.name != "Message->Value" {
            total_messages += 1;
        }
    }
    for (k, v) in results.into_iter() {
        println!("{}: {:.2}%", k, v.0 as f64 / total_time as f64 * 100.);
        println!("{:.2} ms", v.0 as f64 / 1e6);
        println!("{} messages", v.1);
    }
    println!("{:.2} ms, {} messages", total_time as f64 / 1e6, total_messages);
    */
}
