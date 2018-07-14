//! Get info on members of your Slack team.

/// Delete the user profile photo
///
/// Wraps https://api.slack.com/methods/users.deletePhoto

api_call!(delete_photo, "users.deletePhoto");

/// Gets user presence information.
///
/// Wraps https://api.slack.com/methods/users.getPresence

api_call!(
    get_presence,
    "users.getPresence",
    GetPresenceRequest,
    GetPresenceResponse
);

#[derive(Clone, Default, Debug, Serialize)]
pub struct GetPresenceRequest<'a> {
    /// User to get presence info on. Defaults to the authed user.
    pub user: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetPresenceResponse {
    ok: bool,
    pub presence: Option<String>,
}

/// Get a user's identity.
///
/// Wraps https://api.slack.com/methods/users.identity

api_call!(identity, "users.identity", () => IdentityResponse);

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdentityResponse {
    ok: bool,
    pub team: Option<::Team>,
    pub user: Option<::User>,
}

/// Gets information about a user.
///
/// Wraps https://api.slack.com/methods/users.info

api_call!(info, "users.info", InfoRequest, InfoResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct InfoRequest<'a> {
    /// User to get info on
    pub user: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InfoResponse {
    ok: bool,
    pub user: Option<::User>,
}

/// Lists all users in a Slack team.
///
/// Wraps https://api.slack.com/methods/users.list

api_call!(list, "users.list", ListRequest, ListResponse);

/// At this time, providing no limit value will result in Slack
/// attempting to deliver you the entire result set.
/// If the collection is too large you may experience HTTP 500 errors.
/// Resolve this scenario by using pagination.
///
/// One day pagination will become required to use this method.
#[derive(Clone, Default, Debug, Serialize)]
pub struct ListRequest {
    /// Whether to include presence data in the output
    pub presence: Option<bool>,
    pub cursor: Option<String>,
    pub limit: Option<usize>,
    pub include_locale: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListResponse {
    ok: bool,
    pub members: Vec<::User>,
    pub cache_ts: Option<::Timestamp>,
    pub response_metadata: Option<ResponseMetadata>,
    pub is_limited: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResponseMetadata {
    next_cursor: Option<String>,
}

/// Gets a users's preferences
///
/// Wraps https://api.slack.com/methods/users.prefs.get

api_call!(prefs_get, "users.prefs.get", () => PrefsResponse);

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PrefsResponse {
    ok: bool,
    pub prefs: UserPrefs,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserPrefs {
    muted_channels: Vec<::ChannelId>,
}

/// Marks a user as active.
///
/// Wraps https://api.slack.com/methods/users.setActive

api_call!(set_active, "users.setActive");

/// Manually sets user presence.
///
/// Wraps https://api.slack.com/methods/users.setPresence

api_call!(set_presence, "users.setPresence", SetPresenceRequest => ());

#[derive(Clone, Default, Debug, Serialize)]
pub struct SetPresenceRequest<'a> {
    /// Either auto or away
    pub presence: &'a str,
}
