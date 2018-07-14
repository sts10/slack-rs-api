//! Get info on your team's Slack channels, create or archive channels, invite users, set the topic and purpose, and mark a channel as read.

/// Archives a channel.
///
/// Wraps https://api.slack.com/methods/channels.archive
api_call!(archive, "channels.archive", ArchiveRequest => ());

#[derive(Clone, Default, Debug, Serialize)]
pub struct ArchiveRequest {
    /// Channel to archive
    pub channel: ::ChannelId,
}

/// Creates a channel.
///
/// Wraps https://api.slack.com/methods/channels.create

api_call!(create, "channels.create", CreateRequest, CreateResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct CreateRequest<'a> {
    /// Name of channel to create
    pub name: &'a str,
    /// Whether to return errors on invalid channel name instead of modifying it to meet the specified criteria.
    pub validate: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateResponse {
    ok: bool,
    pub channel: ::Channel,
}

/// Fetches history of messages and events from a channel.
///
/// Wraps https://api.slack.com/methods/channels.history

api_call!(history, "channels.history", HistoryRequest, HistoryResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct HistoryRequest<'a> {
    /// Channel to fetch history for.
    pub channel: ::ChannelId,
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
    pub latest: Option<::Timestamp>,
    pub messages: Vec<::Message>,
    pub is_limited: Option<bool>,
}

/// Gets information about a channel.
///
/// Wraps https://api.slack.com/methods/channels.info

api_call!(info, "channels.info", InfoRequest, InfoResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct InfoRequest {
    /// Channel to get info on
    pub channel: ::ChannelId,
    pub include_locale: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InfoResponse {
    ok: bool,
    pub channel: ::Channel,
}

/// Invites a user to a channel.
///
/// Wraps https://api.slack.com/methods/channels.invite

api_call!(invite, "channels.invite", InviteRequest, InviteResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct InviteRequest {
    /// Channel to invite user to.
    pub channel: ::ChannelId,
    /// User to invite to channel.
    pub user: ::UserId,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InviteResponse {
    ok: bool,
    pub channel: ::Channel,
}

/// Joins a channel, creating it if needed.
///
/// Wraps https://api.slack.com/methods/channels.join

api_call!(join, "channels.join", JoinRequest, JoinResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct JoinRequest<'a> {
    /// Name of channel to join
    pub name: &'a str,
    /// Whether to return errors on invalid channel name instead of modifying it to meet the specified criteria.
    pub validate: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JoinResponse {
    ok: bool,
    pub channel: ::Channel, //TODO: This is actually an Either based on already_in_channel
    pub already_in_channel: Option<bool>,
}

/// Removes a user from a channel.
///
/// Wraps https://api.slack.com/methods/channels.kick

api_call!(kick, "channels.kick", KickRequest => ());

#[derive(Clone, Default, Debug, Serialize)]
pub struct KickRequest {
    /// Channel to remove user from.
    pub channel: ::ChannelId,
    /// User to remove from channel.
    pub user: ::UserId,
}

/// Leaves a channel.
///
/// Wraps https://api.slack.com/methods/channels.leave

api_call!(leave, "channels.leave", LeaveRequest, LeaveResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct LeaveRequest {
    /// Channel to leave
    pub channel: ::ChannelId,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LeaveResponse {
    ok: bool,
    not_in_channel: Option<bool>,
}

/// Lists all channels in a Slack team.
///
/// Wraps https://api.slack.com/methods/channels.list

api_call!(list, "channels.list", ListRequest, ListResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct ListRequest {
    /// Exclude archived channels from the list
    pub exclude_archived: Option<bool>,
    /// Exclude the members collection from each channel
    pub exclude_members: Option<bool>,
    pub cursor: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListResponse {
    ok: bool,
    pub channels: Vec<::Channel>,
    pub response_metadata: Option<ResponseMetadata>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ResponseMetadata {
    next_cursor: Option<String>,
}

/// Sets the read cursor in a channel.
///
/// Wraps https://api.slack.com/methods/channels.mark

api_call!(mark, "channels.mark", MarkRequest => ());

#[derive(Clone, Default, Debug, Serialize)]
pub struct MarkRequest<'a> {
    /// Channel to set reading cursor in.
    pub channel: ::ChannelId,
    /// Timestamp of the most recently seen message.
    pub ts: &'a str,
}

/// Renames a channel.
///
/// Wraps https://api.slack.com/methods/channels.rename

api_call!(rename, "channels.rename", RenameRequest, RenameResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct RenameRequest<'a> {
    /// Channel to rename
    pub channel: ::ChannelId,
    /// New name for channel.
    pub name: &'a str,
    /// Whether to return errors on invalid channel name instead of modifying it to meet the specified criteria.
    pub validate: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RenameResponse {
    ok: bool,
    pub channel: ::Channel,
}

/// Retrieve a thread of messages posted to a channel
///
/// Wraps https://api.slack.com/methods/channels.replies

api_call!(replies, "channels.replies", RepliesRequest, RepliesResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct RepliesRequest<'a> {
    /// Channel to fetch thread from
    pub channel: ::ChannelId,
    /// Unique identifier of a thread's parent message
    pub thread_ts: &'a str,
}

// TODO: This looks like its messages are of mixed type?
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RepliesResponse {
    ok: bool,
    pub has_more: bool,
    pub messages: Vec<::Message>,
}

/// Sets the purpose for a channel.
///
/// Wraps https://api.slack.com/methods/channels.setPurpose

api_call!(
    set_purpose,
    "channels.setPurpose",
    SetPurposeRequest,
    SetPurposeResponse
);

#[derive(Clone, Default, Debug, Serialize)]
pub struct SetPurposeRequest<'a> {
    /// Channel to set the purpose of
    pub channel: ::ChannelId,
    /// The new purpose
    pub purpose: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SetPurposeResponse {
    ok: bool,
    pub purpose: String,
}

/// Sets the topic for a channel.
///
/// Wraps https://api.slack.com/methods/channels.setTopic

api_call!(set_topic, "channels.setTopic", SetTopicRequest, SetTopicResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct SetTopicRequest<'a> {
    /// Channel to set the topic of
    pub channel: ::ChannelId,
    /// The new topic
    pub topic: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SetTopicResponse {
    ok: bool,
    pub topic: String,
}

/// Unarchives a channel.
///
/// Wraps https://api.slack.com/methods/channels.unarchive

api_call!(unarchive, "channels.unarchive", UnarchiveRequest => ());

#[derive(Clone, Default, Debug, Serialize)]
pub struct UnarchiveRequest {
    /// Channel to unarchive
    pub channel: ::ChannelId,
}

#[cfg(test)]
mod tests {
    use super::*;
    use id::{ChannelId, UserId};
    use requests;
    use std::env;

    #[test]
    fn test_archive_unarchive() {
        let client = requests::default_client();
        let token = env::var("SLACK_API_TOKEN").unwrap();

        let mut req = UnarchiveRequest::default();
        req.channel = ChannelId::from("CAGMCM14K");
        unarchive(&client, &token, &req);

        let mut req = ArchiveRequest::default();
        req.channel = ChannelId::from("CAGMCM14K");
        archive(&client, &token, &req).unwrap();

        let mut req = UnarchiveRequest::default();
        req.channel = ChannelId::from("CAGMCM14K");
        unarchive(&client, &token, &req).unwrap();
    }

    #[test]
    fn test_create() {
        let client = requests::default_client();
        let token = env::var("SLACK_API_TOKEN").unwrap();
        match create(
            &client,
            &token,
            &CreateRequest {
                name: "testchannel",
                validate: Some(false),
            },
        ) {
            Ok(_) => {}
            Err(requests::Error::Slack(cause)) => {
                if cause != "name_taken" {
                    panic!(cause);
                }
            }
            Err(e) => panic!(e),
        }
    }

    #[test]
    fn test_history() {
        let client = requests::default_client();
        let token = env::var("SLACK_API_TOKEN").unwrap();
        let mut req = HistoryRequest::default();
        req.channel = ChannelId::from("CAGMCM14K");
        history(&client, &token, &req).unwrap();
    }

    #[test]
    fn test_info() {
        let client = requests::default_client();
        let token = env::var("SLACK_API_TOKEN").unwrap();
        let mut req = InfoRequest::default();
        req.channel = ChannelId::from("CAGMCM14K");
        info(&client, &token, &req).unwrap();
    }

    #[test]
    fn test_invite_kick() {
        let client = requests::default_client();
        let token = env::var("SLACK_API_TOKEN").unwrap();

        let mut req = KickRequest::default();
        req.channel = ChannelId::from("CAGMCM14K");
        req.user = UserId::from("UAJHFUB0C");
        kick(&client, &token, &req);

        let mut req = InviteRequest::default();
        req.channel = ChannelId::from("CAGMCM14K");
        req.user = UserId::from("UAJHFUB0C");
        invite(&client, &token, &req).unwrap();

        let mut req = KickRequest::default();
        req.channel = ChannelId::from("CAGMCM14K");
        req.user = UserId::from("UAJHFUB0C");
        kick(&client, &token, &req).unwrap();
    }

    #[test]
    fn test_join_leave() {
        let client = requests::default_client();
        let token = env::var("SLACK_API_TOKEN").unwrap();

        let mut req = LeaveRequest::default();
        req.channel = ChannelId::from("CAGMCM14K");
        leave(&client, &token, &req);

        let mut req = JoinRequest::default();
        req.name = "#testchannel";
        join(&client, &token, &req).unwrap();

        let mut req = LeaveRequest::default();
        req.channel = ChannelId::from("CAGMCM14K");
        leave(&client, &token, &req).unwrap();
    }

    #[test]
    fn test_list() {
        let client = requests::default_client();
        let token = env::var("SLACK_API_TOKEN").unwrap();
        list(&client, &token, &ListRequest::default()).unwrap();
    }

    #[test]
    fn test_mark() {
        use std::time::{SystemTime, UNIX_EPOCH};

        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap();
        let time_string = format!("{}", since_the_epoch.as_secs());

        let client = requests::default_client();
        let token = env::var("SLACK_API_TOKEN").unwrap();

        let req = MarkRequest {
            channel: ChannelId::from("C9VGPGBL4"),
            ts: &time_string,
        };
        mark(&client, &token, &req).unwrap();
    }

    #[test]
    fn test_rename() {
        let client = requests::default_client();
        let token = env::var("SLACK_API_TOKEN").unwrap();
        let mut req = RenameRequest::default();
        req.channel = ChannelId::from("CAGMCM14K");
        req.name = "testchannel";
        rename(&client, &token, &req).unwrap();

        req.name = "other_testchannel";
        rename(&client, &token, &req).unwrap();

        req.name = "testchannel";
        rename(&client, &token, &req).unwrap();
    }

    #[test]
    fn test_replies() {
        let client = requests::default_client();
        let token = env::var("SLACK_API_TOKEN").unwrap();
        let mut req = RepliesRequest::default();
        req.channel = ChannelId::from("CAGMCM14K");
        req.thread_ts = "1525306421.000207";
        replies(&client, &token, &req).unwrap();
    }

    #[test]
    fn test_set_purpose() {
        let client = requests::default_client();
        let token = env::var("SLACK_API_TOKEN").unwrap();
        let mut req = SetPurposeRequest::default();
        req.channel = ChannelId::from("C9VGPGBL4");
        req.purpose = "test_purpose";

        let response = set_purpose(&client, &token, &req).unwrap();
        assert_eq!(response.purpose, "test_purpose");

        req.purpose = "other_test_purpose";
        let response = set_purpose(&client, &token, &req).unwrap();
        assert_eq!(response.purpose, "other_test_purpose");
    }

    #[test]
    fn test_set_topic() {
        let client = requests::default_client();
        let token = env::var("SLACK_API_TOKEN").unwrap();
        let mut req = SetTopicRequest::default();
        req.channel = ChannelId::from("C9VGPGBL4");
        req.topic = "test_topic";

        let response = set_topic(&client, &token, &req).unwrap();
        assert_eq!(response.topic, "test_topic");

        req.topic = "other_test_topic";
        let response = set_topic(&client, &token, &req).unwrap();
        assert_eq!(response.topic, "other_test_topic");
    }
}
