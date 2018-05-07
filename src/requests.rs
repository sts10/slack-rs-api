//! Functionality for sending requests to Slack.
pub use reqwest::Client;
use std::error;

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
pub fn send_structured<T: ::serde::Serialize>(
    client: &::reqwest::Client,
    method_url: &str,
    params: &T,
) -> Result<String, ::reqwest::Error> {
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

macro_rules! api_call {
    ($name:ident, $strname:expr, $reqty:ty, $okty:ty) => {
        pub fn $name(client: &::reqwest::Client, token: &str, request: &$reqty) -> Result<$okty, ::requests::Error> {
            use requests::Error;
            #[derive(Deserialize)]
            struct IsError {
                ok: bool,
                error: Option<String>,
            }

            let url = ::requests::get_slack_url_for_method($strname) + "?token=" + token + "&";
            let bytes = ::requests::send_structured(client, &url, &request).map_err(Error::Client)?;

            let is_error = ::serde_json::from_str::<IsError>(&bytes);
            match is_error {
                // Complete failure, can't do anything with the bytes
                Err(e) => Err(Error::CannotParse(e, bytes)),
                // Slack sent us an error
                Ok(IsError { ok: false, error }) => Err(Error::Slack(error.unwrap_or_default())),
                // Slack sent us an success result
                Ok(IsError { ok: true, .. }) => match ::serde_json::from_str::<$okty>(&bytes) {
                    Ok(res) => Ok(res),
                    Err(e) => Err(Error::CannotParse(e, bytes)),
                },
            }
        }
    };
}

pub fn get_slack_url_for_method(method: &str) -> String {
    format!("https://slack.com/api/{}", method)
}

#[derive(Debug)]
pub enum Error {
    Slack(String),
    CannotParse(::serde_json::error::Error, String),
    Client(::reqwest::Error),
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match *self {
            Error::Slack(ref reason) => write!(f, "{}", reason),
            Error::CannotParse(..) => write!(f, "Could not parse as specified result type"),
            Error::Client(..) => write!(f, "The requests client failed"),
        }
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Slack(ref reason) => reason,
            Error::CannotParse(..) => "Could not parse as specified result type",
            Error::Client(..) => "The requests client failed",
        }
    }
}
