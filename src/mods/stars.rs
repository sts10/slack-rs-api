/// Adds a star to an item.
///
/// Wraps https://api.slack.com/methods/stars.add

api_call!(add, "stars.add", AddRequest => ());

#[derive(Clone, Default, Debug, Serialize)]
pub struct AddRequest<'a> {
    /// File to add star to.
    pub file: Option<&'a str>,
    /// File comment to add star to.
    pub file_comment: Option<&'a str>,
    /// Channel to add star to, or channel where the message to add star to was posted (used with timestamp).
    pub channel: Option<&'a str>,
    /// Timestamp of the message to add star to.
    pub timestamp: Option<&'a str>,
}

/// Lists stars for a user.
///
/// Wraps https://api.slack.com/methods/stars.list

api_call!(list, "stars.list", ListRequest, ListResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct ListRequest {
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
pub enum ListResponseItem {
    Message { channel: String, message: ::Message },
    File { file: ::File },
    FileComment { comment: ::FileComment, file: ::File },
    Channel { channel: String },
    Im { channel: String },
    Group { group: String },
}

/// Removes a star from an item.
///
/// Wraps https://api.slack.com/methods/stars.remove

api_call!(remove, "stars.remove", RemoveRequest => ());

#[derive(Clone, Default, Debug, Serialize)]
pub struct RemoveRequest<'a> {
    /// File to remove star from.
    pub file: Option<&'a str>,
    /// File comment to remove star from.
    pub file_comment: Option<&'a str>,
    /// Channel to remove star from, or channel where the message to remove star from was posted (used with timestamp).
    pub channel: Option<&'a str>,
    /// Timestamp of the message to remove star from.
    pub timestamp: Option<&'a str>,
}
