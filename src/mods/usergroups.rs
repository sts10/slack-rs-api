//! Get info on your team's User Groups.

/// Create a User Group
///
/// Wraps https://api.slack.com/methods/usergroups.create

api_call!(create, "usergroups.create", CreateRequest, CreateResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct CreateRequest<'a> {
    /// A name for the User Group. Must be unique among User Groups.
    pub name: &'a str,
    /// A mention handle. Must be unique among channels, users and User Groups.
    pub handle: Option<&'a str>,
    /// A short description of the User Group.
    pub description: Option<&'a str>,
    /// A comma separated string of encoded channel IDs for which the User Group uses as a default.
    pub channels: Option<&'a str>,
    /// Include the number of users in each User Group.
    pub include_count: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateResponse {
    ok: bool,
    pub usergroup: Option<::Usergroup>,
}

/// Disable an existing User Group
///
/// Wraps https://api.slack.com/methods/usergroups.disable

api_call!(disable, "usergroups.disable", DisableRequest, DisableResponse);

#[derive(Clone, Debug, Serialize)]
pub struct DisableRequest {
    /// The encoded ID of the User Group to disable.
    pub usergroup: ::UsergroupId,
    /// Include the number of users in the User Group.
    pub include_count: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DisableResponse {
    ok: bool,
    pub usergroup: Option<::Usergroup>,
}

/// Enable a User Group
///
/// Wraps https://api.slack.com/methods/usergroups.enable

api_call!(enable, "usergroups.enable", EnableRequest, EnableResponse);

#[derive(Clone, Debug, Serialize)]
pub struct EnableRequest {
    /// The encoded ID of the User Group to enable.
    pub usergroup: ::UsergroupId,
    /// Include the number of users in the User Group.
    pub include_count: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EnableResponse {
    ok: bool,
    pub usergroup: Option<::Usergroup>,
}

/// List all User Groups for a team
///
/// Wraps https://api.slack.com/methods/usergroups.list

api_call!(list, "usergroups.list", ListRequest, ListResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct ListRequest {
    /// Include disabled User Groups.
    pub include_disabled: Option<bool>,
    /// Include the number of users in each User Group.
    pub include_count: Option<bool>,
    /// Include the list of users for each User Group.
    pub include_users: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListResponse {
    ok: bool,
    pub usergroups: Option<Vec<::Usergroup>>,
}

/// Update an existing User Group
///
/// Wraps https://api.slack.com/methods/usergroups.update

api_call!(update, "usergroups.update", UpdateRequest, UpdateResponse);

#[derive(Clone, Debug, Serialize)]
pub struct UpdateRequest<'a> {
    /// The encoded ID of the User Group to update.
    pub usergroup: ::UsergroupId,
    /// A name for the User Group. Must be unique among User Groups.
    pub name: Option<&'a str>,
    /// A mention handle. Must be unique among channels, users and User Groups.
    pub handle: Option<&'a str>,
    /// A short description of the User Group.
    pub description: Option<&'a str>,
    /// A comma separated string of encoded channel IDs for which the User Group uses as a default.
    pub channels: Option<&'a str>, // TODO: Should be a Vec<ChannelId> and serialize_with
    /// Include the number of users in the User Group.
    pub include_count: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateResponse {
    ok: bool,
    pub usergroup: Option<::Usergroup>,
}
