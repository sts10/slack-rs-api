/// Adds a reaction to an item.
///
/// Wraps https://api.slack.com/methods/reactions.add

api_call!(add, "reactions.add", AddRequest, AddResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct AddRequest<'a> {
    /// Reaction (emoji) name.
    pub name: &'a str,
    /// File to add reaction to.
    pub file: Option<&'a str>,
    /// File comment to add reaction to.
    pub file_comment: Option<&'a str>,
    /// Channel where the message to add reaction to was posted.
    pub channel: Option<&'a str>,
    /// Timestamp of the message to add reaction to.
    pub timestamp: Option<&'a str>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AddResponse {
    ok: bool,
}

/// Gets reactions for an item.
///
/// Wraps https://api.slack.com/methods/reactions.get

api_call!(get, "reactions.get", GetRequest, GetResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct GetRequest<'a> {
    /// File to get reactions for.
    pub file: Option<&'a str>,
    /// File comment to get reactions for.
    pub file_comment: Option<&'a str>,
    /// Channel where the message to get reactions for was posted.
    pub channel: Option<&'a str>,
    /// Timestamp of the message to get reactions for.
    pub timestamp: Option<&'a str>,
    /// If true always return the complete reaction list.
    pub full: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum GetResponse {
    Message(GetResponseMessage),
    File(GetResponseFile),
    FileComment(GetResponseFileComment),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GetResponseFile {
    ok: bool,
    pub file: ::File,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetResponseFileComment {
    ok: bool,
    pub comment: ::FileComment,
    pub file: ::File,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetResponseMessage {
    ok: bool,
    pub channel: String,
    pub message: ::Message,
}

/// Lists reactions made by a user.
///
/// Wraps https://api.slack.com/methods/reactions.list

api_call!(list, "reactions.list", ListRequest, ListResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct ListRequest<'a> {
    /// Show reactions made by this user. Defaults to the authed user.
    pub user: Option<&'a str>,
    /// If true always return the complete reaction list.
    pub full: Option<bool>,
    /// Number of items to return per page.
    pub count: Option<u32>,
    /// Page number of results to return.
    pub page: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListResponse {
    ok: bool,
    pub items: Option<Vec<ListResponseItem>>,
    pub paging: Option<::Paging>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(deny_unknown_fields)]
pub enum ListResponseItem {
    Message(ListResponseItemMessage),
    File(ListResponseItemFile),
    FileComment(ListResponseItemFileComment),
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListResponseItemFile {
    pub file: ::File,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListResponseItemFileComment {
    pub comment: ::FileComment,
    pub file: ::File,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListResponseItemMessage {
    pub channel: String,
    pub message: ::Message,
}

/// Removes a reaction from an item.
///
/// Wraps https://api.slack.com/methods/reactions.remove

api_call!(remove, "reactions.remove", RemoveRequest, RemoveResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct RemoveRequest<'a> {
    /// Reaction (emoji) name.
    pub name: &'a str,
    /// File to remove reaction from.
    pub file: Option<&'a str>,
    /// File comment to remove reaction from.
    pub file_comment: Option<&'a str>,
    /// Channel where the message to remove reaction from was posted.
    pub channel: Option<&'a str>,
    /// Timestamp of the message to remove reaction from.
    pub timestamp: Option<&'a str>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RemoveResponse {
    ok: bool,
}
