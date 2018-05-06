//! Get info on your team's Slack channels, create or archive channels, invite users, set the topic and purpose, and mark a channel as read.

extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_qs;

#[macro_use]
extern crate requests;

/// Revokes a token.
///
/// Wraps https://api.slack.com/methods/auth.revoke

api_call!(revoke, "auth.revoke", RevokeRequest, RevokeResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct RevokeRequest {
    /// Setting this parameter to 1 triggers a testing mode where the specified token will not actually be revoked.
    pub test: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RevokeResponse {
    ok: bool,
    pub revoked: bool,
}

/// Checks authentication & identity.
///
/// Wraps https://api.slack.com/methods/auth.test

api_call!(test, "auth.test", TestRequest, TestResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct TestRequest {}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TestResponse {
    ok: bool,
    pub team: String,
    pub team_id: String,
    pub url: String,
    pub user: String,
    pub user_id: String,
}
