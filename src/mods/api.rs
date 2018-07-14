use std::collections::HashMap;

// Experimental, not sure if I like this syntax, but it is almost free of boilerplate
slack!{
    /// Checks API calling code.
    ///
    /// Wraps https://api.slack.com/methods/api.test
    test,
    "api.test",
    TestRequest {
        /// Error response to return
        error: Option<&'a str>,
        /// example property to return
        foo: Option<&'a str>,
    },
    TestResponse {
        args: HashMap<String, String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_test() {
        let client = ::requests::default_client();
        let token = env::var("SLACK_API_TOKEN").unwrap();

        let req = TestRequest {
            error: None,
            foo: Some("bar"),
        };
        let response = test(&client, &token, &req).unwrap();
        assert_eq!(response.args["foo"], "bar".to_string())
    }
}
