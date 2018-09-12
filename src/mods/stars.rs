/// Adds a star to an item.
///
/// Wraps https://api.slack.com/methods/stars.add

api_call!(add, "stars.add", AddRequest => ());

//TODO: These requests also require combinations of fields- file, channel, or channel and timestamp
#[derive(Clone, Debug, Serialize, new)]
pub struct AddRequest {
    /// File to add star to.
    #[new(default)]
    pub file: Option<::FileId>,
    /// Channel to add star to, or channel where the message to add star to was posted (used with timestamp).
    #[new(default)]
    pub channel: Option<::ChannelId>,
    /// Timestamp of the message to add star to.
    #[new(default)]
    pub timestamp: Option<::Timestamp>,
}

/// Lists stars for a user.
///
/// Wraps https://api.slack.com/methods/stars.list

api_call!(list, "stars.list", ListRequest, ListResponse);

#[derive(Clone, Debug, Serialize, new)]
pub struct ListRequest {
    /// Number of items to return per page.
    #[new(default)]
    pub count: Option<u32>,
    /// Page number of results to return.
    #[new(default)]
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

#[derive(Clone, Debug, Serialize, new)]
pub struct RemoveRequest {
    /// File to remove star from.
    #[new(default)]
    pub file: Option<::FileId>,
    /// Channel to remove star from, or channel where the message to remove star from was posted (used with timestamp).
    #[new(default)]
    pub channel: Option<::ChannelId>,
    /// Timestamp of the message to remove star from.
    #[new(default)]
    pub timestamp: Option<::Timestamp>,
}
