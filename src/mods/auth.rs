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

api_call!(test, "auth.test", () => TestResponse);

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
