/// List all users in a User Group
///
/// Wraps https://api.slack.com/methods/usergroups.users.list

api_call!(list, "usergroups.users.list", ListRequest, ListResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct ListRequest<'a> {
    /// The encoded ID of the User Group to update.
    pub usergroup: &'a str,
    /// Allow results that involve disabled User Groups.
    pub include_disabled: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListResponse {
    ok: bool,
    pub users: Option<Vec<String>>,
}

/// Update the list of users for a User Group
///
/// Wraps https://api.slack.com/methods/usergroups.users.update

api_call!(update, "usergroups.users.update", UpdateRequest, UpdateResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct UpdateRequest<'a> {
    /// The encoded ID of the User Group to update.
    pub usergroup: &'a str,
    /// A comma separated string of encoded user IDs that represent the entire list of users for the User Group.
    pub users: &'a str,
    /// Include the number of users in the User Group.
    pub include_count: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateResponse {
    ok: bool,
    pub usergroup: Option<::Usergroup>,
}
