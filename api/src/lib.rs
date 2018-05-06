//! Get info on your team's Slack channels, create or archive channels, invite users, set the topic and purpose, and mark a channel as read.

#[macro_use]
extern crate derive_error;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_qs;

#[macro_use]
extern crate requests;
use requests::get_slack_url_for_method;

/// Checks API calling code.
///
/// Wraps https://api.slack.com/methods/api.test

api_call!(test, "api.test", TestRequest, Result<TestResponse, TestError>);

#[derive(Clone, Default, Debug, Serialize)]
pub struct TestRequest<'a> {
    /// Error response to return
    pub error: Option<&'a str>,
    /// example property to return
    pub foo: Option<&'a str>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestResponse {
    ok: bool,
    pub args: std::collections::HashMap<String, String>,
}

#[derive(Debug, Deserialize, Error)]
#[serde(rename_all = "snake_case")]
#[error(non_std)]
pub enum TestError {
    /// The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.
    InvalidArgName,
    /// The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.
    InvalidArrayArg,
    /// The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.
    InvalidCharset,
    /// The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.
    InvalidFormData,
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    #[serde(skip_deserializing)]
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    #[serde(skip_deserializing)]
    #[error(msg_embedded, no_from, non_std)]
    Unknown(String),
    /// The client had an error sending the request to Slack
    #[serde(skip_deserializing)]
    Client(::reqwest::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_test() {
        let client = requests::default_client().unwrap();
        let token = env::var("SLACK_API_TOKEN").unwrap();

        let req = TestRequest {
            error: None,
            foo: Some("bar"),
        };
        let response = test(&client, &token, &req).unwrap();
        assert_eq!(response.args["foo"], "bar".to_string())
    }
}
