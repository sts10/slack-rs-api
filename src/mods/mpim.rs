//! Get info on your multiparty direct messages.
//TODO: check signatures

use types::*;

/// Closes a multiparty direct message channel.
///
/// Wraps https://api.slack.com/methods/mpim.close

api_call!(close, "mpim.close", CloseRequest, CloseResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct CloseRequest<'a> {
    /// MPIM to close.
    pub channel: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CloseResponse {
    ok: bool,
}

/// Fetches history of messages and events from a multiparty direct message.
///
/// Wraps https://api.slack.com/methods/mpim.history

api_call!(history, "mpim.history", HistoryRequest, HistoryResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct HistoryRequest<'a> {
    /// Multiparty direct message to fetch history for.
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

/// Lists multiparty direct message channels for the calling user.
///
/// Wraps https://api.slack.com/methods/mpim.list

api_call!(list, "mpim.list", ListResponse);

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponse {
    ok: bool,
    pub groups: Vec<Mpim>,
}

/// Sets the read cursor in a multiparty direct message channel.
///
/// Wraps https://api.slack.com/methods/mpim.mark

api_call!(mark, "mpim.mark", MarkRequest, MarkResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct MarkRequest<'a> {
    /// multiparty direct message channel to set reading cursor in.
    pub channel: &'a str,
    /// Timestamp of the most recently seen message.
    pub ts: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MarkResponse {
    ok: bool,
}

/// This method opens a multiparty direct message.
///
/// Wraps https://api.slack.com/methods/mpim.open

api_call!(open, "mpim.open", OpenRequest, OpenResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct OpenRequest<'a> {
    /// Comma separated lists of users.  The ordering of the users is preserved whenever a MPIM group is returned.
    pub users: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OpenResponse {
    ok: bool,
    pub group: Option<::Mpim>,
}

/// Retrieve a thread of messages posted to a direct message conversation from a multiparty direct message.
///
/// Wraps https://api.slack.com/methods/mpim.replies

api_call!(replies, "mpim.replies", RepliesRequest, RepliesResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct RepliesRequest<'a> {
    /// Multiparty direct message channel to fetch thread from.
    pub channel: &'a str,
    /// Unique identifier of a thread's parent message.
    pub thread_ts: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RepliesResponse {
    ok: bool,
    pub messages: Option<Vec<::Message>>,
    pub thread_info: Option<::ThreadInfo>,
}
