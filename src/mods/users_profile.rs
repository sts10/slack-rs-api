/// Retrieves a user's profile information.
///
/// Wraps https://api.slack.com/methods/users.profile.get

api_call!(get, "users.profile.get", GetRequest, GetResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct GetRequest {
    /// User to retrieve profile info for
    pub user: Option<::UserId>,
    /// Include labels for each ID in custom profile fields
    pub include_labels: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetResponse {
    ok: bool,
    pub profile: Option<::UserProfile>,
}

/// Set the profile information for a user.
///
/// Wraps https://api.slack.com/methods/users.profile.set

api_call!(set, "users.profile.set", SetRequest, SetResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct SetRequest<'a> {
    /// ID of user to change. This argument may only be specified by team admins on paid teams.
    pub user: Option<::UserId>,
    /// Collection of key:value pairs presented as a URL-encoded JSON hash.
    pub profile: Option<&'a str>,
    /// Name of a single key to set. Usable only if profile is not passed.
    pub name: Option<&'a str>,
    /// Value to set a single key to. Usable only if profile is not passed.
    pub value: Option<&'a str>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SetResponse {
    ok: bool,
    pub profile: Option<::UserProfile>,
}
