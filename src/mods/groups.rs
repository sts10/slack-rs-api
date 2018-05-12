//! Get info on your team's private channels.
//TODO: Check signatures
use types::*;

/// Archives a private channel.
///
/// Wraps https://api.slack.com/methods/groups.archive

api_call!(archive, "groups.archive", ArchiveRequest => ());

#[derive(Clone, Default, Debug, Serialize)]
pub struct ArchiveRequest<'a> {
    /// Private channel to archive
    pub channel: &'a str,
}

/// Closes a private channel.
///
/// Wraps https://api.slack.com/methods/groups.close

api_call!(close, "groups.close", CloseRequest => ());

#[derive(Clone, Default, Debug, Serialize)]
pub struct CloseRequest<'a> {
    /// Private channel to close.
    pub channel: &'a str,
}

/// Creates a private channel.
///
/// Wraps https://api.slack.com/methods/groups.create

api_call!(create, "groups.create", CreateRequest, CreateResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct CreateRequest<'a> {
    /// Name of private channel to create
    pub name: &'a str,
    /// Whether to return errors on invalid channel name instead of modifying it to meet the specified criteria.
    pub validate: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateResponse {
    ok: bool,
    pub group: Option<Group>,
}

/// Clones and archives a private channel.
///
/// Wraps https://api.slack.com/methods/groups.createChild

api_call!(
    create_child,
    "groups.createChild",
    CreateChildRequest,
    CreateChildResponse
);

#[derive(Clone, Default, Debug, Serialize)]
pub struct CreateChildRequest<'a> {
    /// Private channel to clone and archive.
    pub channel: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateChildResponse {
    ok: bool,
    pub group: Option<Group>,
}

/// Fetches history of messages and events from a private channel.
///
/// Wraps https://api.slack.com/methods/groups.history

api_call!(history, "groups.history", HistoryRequest, HistoryResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct HistoryRequest<'a> {
    /// Private channel to fetch history for.
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
    pub has_more: bool,
    pub latest: Option<Timestamp>,
    pub messages: Vec<Message>,
}

/// Gets information about a private channel.
///
/// Wraps https://api.slack.com/methods/groups.info

api_call!(info, "groups.info", InfoRequest, InfoResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct InfoRequest<'a> {
    /// Private channel to get info on
    pub channel: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InfoResponse {
    ok: bool,
    pub group: Group,
}

/// Invites a user to a private channel.
///
/// Wraps https://api.slack.com/methods/groups.invite

api_call!(invite, "groups.invite", InviteRequest, InviteResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct InviteRequest<'a> {
    /// Private channel to invite user to.
    pub channel: &'a str,
    /// User to invite.
    pub user: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InviteResponse {
    ok: bool,
    pub group: Option<::Group>,
}

/// Removes a user from a private channel.
///
/// Wraps https://api.slack.com/methods/groups.kick

api_call!(kick, "groups.kick", KickRequest => ());

#[derive(Clone, Default, Debug, Serialize)]
pub struct KickRequest<'a> {
    /// Private channel to remove user from.
    pub channel: &'a str,
    /// User to remove from private channel.
    pub user: &'a str,
}

/// Leaves a private channel.
///
/// Wraps https://api.slack.com/methods/groups.leave

api_call!(leave, "groups.leave", LeaveRequest => ());

#[derive(Clone, Default, Debug, Serialize)]
pub struct LeaveRequest<'a> {
    /// Private channel to leave
    pub channel: &'a str,
}

/// Lists private channels that the calling user has access to.
///
/// Wraps https://api.slack.com/methods/groups.list

api_call!(list, "groups.list", ListRequest, ListResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct ListRequest {
    /// Don't return archived private channels.
    pub exclude_archived: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListResponse {
    ok: bool,
    pub groups: Vec<Group>,
}

/// Sets the read cursor in a private channel.
///
/// Wraps https://api.slack.com/methods/groups.mark

api_call!(mark, "groups.mark", MarkRequest => ());

#[derive(Clone, Default, Debug, Serialize)]
pub struct MarkRequest<'a> {
    /// Private channel to set reading cursor in.
    pub channel: &'a str,
    /// Timestamp of the most recently seen message.
    pub ts: &'a str,
}

/// Opens a private channel.
///
/// Wraps https://api.slack.com/methods/groups.open

api_call!(open, "groups.open", OpenRequest => ());

#[derive(Clone, Default, Debug, Serialize)]
pub struct OpenRequest<'a> {
    /// Private channel to open.
    pub channel: &'a str,
}

/// Renames a private channel.
///
/// Wraps https://api.slack.com/methods/groups.rename

api_call!(rename, "groups.rename", RenameRequest, RenameResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct RenameRequest<'a> {
    /// Private channel to rename
    pub channel: &'a str,
    /// New name for private channel.
    pub name: &'a str,
    /// Whether to return errors on invalid channel name instead of modifying it to meet the specified criteria.
    pub validate: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RenameResponse {
    ok: bool,
    pub channel: Option<RenameResponseChannel>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RenameResponseChannel {
    pub created: Option<f32>,
    pub id: Option<String>,
    pub is_group: Option<bool>,
    pub name: Option<String>,
}

/// Retrieve a thread of messages posted to a private channel
///
/// Wraps https://api.slack.com/methods/groups.replies

api_call!(replies, "groups.replies", RepliesRequest, RepliesResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct RepliesRequest<'a> {
    /// Private channel to fetch thread from
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

/// Sets the purpose for a private channel.
///
/// Wraps https://api.slack.com/methods/groups.setPurpose

api_call!(set_purpose, "groups.setPurpose", SetPurposeRequest, SetPurposeResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct SetPurposeRequest<'a> {
    /// Private channel to set the purpose of
    pub channel: &'a str,
    /// The new purpose
    pub purpose: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SetPurposeResponse {
    ok: bool,
    pub purpose: Option<String>,
}

/// Sets the topic for a private channel.
///
/// Wraps https://api.slack.com/methods/groups.setTopic

api_call!(set_topic, "groups.setTopic", SetTopicRequest, SetTopicResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct SetTopicRequest<'a> {
    /// Private channel to set the topic of
    pub channel: &'a str,
    /// The new topic
    pub topic: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SetTopicResponse {
    ok: bool,
    pub topic: Option<String>,
}

/// Unarchives a private channel.
///
/// Wraps https://api.slack.com/methods/groups.unarchive

api_call!(unarchive, "groups.unarchive", UnarchiveRequest => ());

#[derive(Clone, Default, Debug, Serialize)]
pub struct UnarchiveRequest<'a> {
    /// Private channel to unarchive
    pub channel: &'a str,
}
