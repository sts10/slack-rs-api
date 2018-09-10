use std::collections::HashMap;

/// Checks API calling code.
///
/// Wraps https://api.slack.com/methods/api.test
#[derive(Debug, Clone, Default, Serialize, new)]
pub struct TestRequest<'a> {
    /// Error response to return
    error: Option<&'a str>,
    /// example property to return
    foo: Option<&'a str>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TestResponse {
    args: HashMap<String, String>,
}

api_call!(test, "api.test", TestRequest, TestResponse);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_test() {
        let client = ::requests::default_client();
        let token = ::std::env::var("SLACK_API_TOKEN").unwrap();

        let response = test(&client, &token, &TestRequest::new(None, Some("bar"))).unwrap();
        assert_eq!(response.args["foo"], "bar".to_string());
    }
}
