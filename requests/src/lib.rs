//! Functionality for sending requests to Slack.
extern crate reqwest;
extern crate serde;
extern crate serde_qs;

use std::error;

pub use reqwest::Client;
pub use reqwest::Error;

/// Functionality for sending authenticated and unauthenticated requests to Slack via HTTP.
///
/// If you do not have a custom client to integrate with and just want to send requests, use
/// the [`default_client()`] function to get a simple request sender.
pub trait SlackWebRequestSender {
    type Error: error::Error;

    /// Make an API call to Slack. Takes a map of parameters that get appended to the request as query
    /// params.
    fn send(&self, method: &str, params: &[(&str, &str)]) -> Result<String, Self::Error>;
}

impl SlackWebRequestSender for ::reqwest::Client {
    type Error = ::reqwest::Error;

    fn send(&self, method_url: &str, params: &[(&str, &str)]) -> Result<String, Self::Error> {
        let mut url = ::reqwest::Url::parse(method_url).expect("Unable to parse url");

        url.query_pairs_mut().extend_pairs(params);

        self.get(url).send()?.text()
    }
}

/// Make an API call to Slack. Takes a struct that describes the request params
pub fn send_structured<T: ::serde::Serialize>(client: &::reqwest::Client, method_url: &str, params: &T) -> Result<String, ::reqwest::Error> {
    let url_text = method_url.to_string() + &::serde_qs::to_string(params).unwrap();
    let url = ::reqwest::Url::parse(&url_text).unwrap();
    client.get(url).send()?.text()
}

/// Provides a default `reqwest` client to give to the API functions to send requests.
///
/// # Examples
///
/// ```
/// # let token = "some_token";
/// let client = slack_api::requests::default_client().unwrap();
/// let response = slack_api::channels::list(&client, &token, &Default::default());
/// ```
pub fn default_client() -> Result<::reqwest::Client, ::reqwest::Error> {
    ::reqwest::Client::builder().build()
}

#[macro_export]
macro_rules! api_call {
    ($name:ident, $strname:expr, $reqty:ty, Result<$okty:ty, $errty:tt>) => {
        pub fn $name (
            client: &::reqwest::Client,
            token: &str,
            request: &$reqty,
        ) -> Result<$okty, $errty>
        {
            #[derive(Deserialize)]
            struct Temp {
                error: $errty,
            }

            let url = ::get_slack_url_for_method($strname) + "?token=" + token + "&";
            let bytes = ::requests::send_structured(client, &url, &request).map_err($errty::Client)?;
            match serde_json::from_str::<$okty>(&bytes) {
                Ok(v) => Ok(v),
                Err(_) => match serde_json::from_str::<Temp>(&bytes) {
                    Ok(temp) => Err(temp.error),
                    Err(e) => Err($errty::MalformedResponse(e)),
                }
            }
        }
    }
}

pub fn get_slack_url_for_method(method: &str) -> String {
    format!("https://slack.com/api/{}", method)
}


