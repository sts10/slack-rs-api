/// Pins an item to a channel.
///
/// Wraps https://api.slack.com/methods/pins.add

api_call!(add, "pins.add", AddRequest => ());

#[derive(Clone, Default, Debug, Serialize)]
pub struct AddRequest<'a> {
    /// Channel to pin the item in.
    pub channel: &'a str,
    /// File to pin.
    pub file: Option<&'a str>,
    /// File comment to pin.
    pub file_comment: Option<&'a str>,
    /// Timestamp of the message to pin.
    pub timestamp: Option<&'a str>,
}

/// Lists items pinned to a channel.
///
/// Wraps https://api.slack.com/methods/pins.list

api_call!(list, "pins.list", ListRequest, ListResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct ListRequest<'a> {
    /// Channel to get pinned items for.
    pub channel: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListResponse {
    pub items: Option<Vec<ListResponseItem>>,
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
    pub created: Option<f32>,
    pub created_by: Option<String>,
    pub file: ::File,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListResponseItemFileComment {
    pub comment: ::FileComment,
    pub created: Option<f32>,
    pub created_by: Option<String>,
    pub file: ::File,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListResponseItemMessage {
    pub channel: String,
    pub created: Option<f32>,
    pub created_by: Option<String>,
    pub message: ::Message,
}

/// Un-pins an item from a channel.
///
/// Wraps https://api.slack.com/methods/pins.remove

api_call!(remove, "pins.remove", RemoveRequest => ());

#[derive(Clone, Default, Debug, Serialize)]
pub struct RemoveRequest<'a> {
    /// Channel where the item is pinned to.
    pub channel: &'a str,
    /// File to un-pin.
    pub file: Option<&'a str>,
    /// File comment to un-pin.
    pub file_comment: Option<&'a str>,
    /// Timestamp of the message to un-pin.
    pub timestamp: Option<&'a str>,
}
