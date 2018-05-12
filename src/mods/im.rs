//! Get info on your direct messages.
// TODO: check signatures

/// Close a direct message channel.
///
/// Wraps https://api.slack.com/methods/im.close

api_call!(close, "im.close", CloseRequest => ());

#[derive(Clone, Default, Debug, Serialize)]
pub struct CloseRequest<'a> {
    /// Direct message channel to close.
    pub channel: &'a str,
}

/// Fetches history of messages and events from direct message channel.
///
/// Wraps https://api.slack.com/methods/im.history

api_call!(history, "im.history", HistoryRequest, HistoryResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct HistoryRequest<'a> {
    /// Direct message channel to fetch history for.
    pub channel: &'a str,
    /// End of time range of messages to include in results.
    pub latest: Option<&'a str>,
    /// Start of time range of messages to include in results.
    pub oldest: Option<&'a str>,
    /// Include messages with latest or oldest timestamp in results.
    pub inclusive: Option<bool>,
    /// Number of messages to return, between 1 and 1000.
    pub count: Option<u32>,
    /// Include unread_count_display in the output?
    pub unreads: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HistoryResponse {
    ok: bool,
    pub has_more: Option<bool>,
    pub latest: Option<String>,
    pub messages: Option<Vec<::Message>>,
}

/// Lists direct message channels for the calling user.
///
/// Wraps https://api.slack.com/methods/im.list

api_call!(list, "im.list", ListRequest, ListResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct ListRequest<'a> {
    /// Paginate through collections of data by setting the `cursor` parameter to a `next_cursor` attribute returned by a previous request's `response_metadata`. Default value fetches the first "page" of the collection. See pagination for more detail.
    pub cursor: Option<&'a str>,
    /// The maximum number of items to return. Fewer than the requested number of items may be returned, even if the end of the users list hasn't been reached.
    pub limit: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListResponse {
    ok: bool,
    pub ims: Option<Vec<::Im>>,
}

/// Sets the read cursor in a direct message channel.
///
/// Wraps https://api.slack.com/methods/im.mark

api_call!(mark, "im.mark", MarkRequest => ());

#[derive(Clone, Default, Debug, Serialize)]
pub struct MarkRequest<'a> {
    /// Direct message channel to set reading cursor in.
    pub channel: &'a str,
    /// Timestamp of the most recently seen message.
    pub ts: &'a str,
}

/// Opens a direct message channel.
///
/// Wraps https://api.slack.com/methods/im.open

api_call!(open, "im.open", OpenRequest, OpenResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct OpenRequest<'a> {
    /// User to open a direct message channel with.
    pub user: &'a str,
    /// Boolean, indicates you want the full IM channel definition in the response.
    pub return_im: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct OpenResponse {
    ok: bool,
    pub channel: Option<::Im>,
}

/// Retrieve a thread of messages posted to a direct message conversation
///
/// Wraps https://api.slack.com/methods/im.replies

api_call!(replies, "im.replies", RepliesRequest, RepliesResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct RepliesRequest<'a> {
    /// Direct message channel to fetch thread from
    pub channel: &'a str,
    /// Unique identifier of a thread's parent message
    pub thread_ts: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RepliesResponse {
    ok: bool,
    pub messages: Option<Vec<::Message>>,
    pub thread_info: Option<::ThreadInfo>,
}
