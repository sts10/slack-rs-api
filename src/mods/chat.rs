//! Post chat messages to Slack.

use types::*;

/// Deletes a message.
///
/// Wraps https://api.slack.com/methods/chat.delete

api_call!(delete, "chat.delete", DeleteRequest, DeleteResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct DeleteRequest<'a> {
    /// Timestamp of the message to be deleted.
    pub ts: &'a str,
    /// Channel containing the message to be deleted.
    pub channel: &'a str,
    /// Pass true to delete the message as the authed user. Bot users in this context are considered authed users.
    pub as_user: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeleteResponse {
    ok: bool,
    pub channel: String,
    pub ts: String,
}

/// Share a me message into a channel.
///
/// Wraps https://api.slack.com/methods/chat.meMessage

api_call!(me_message, "chat.meMessage", MeMessageRequest, MeMessageResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct MeMessageRequest<'a> {
    /// Channel to send message to. Can be a public channel, private group or IM channel. Can be an encoded ID, or a name.
    pub channel: &'a str,
    /// Text of the message to send.
    pub text: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MeMessageResponse {
    ok: bool,
    pub channel: Option<String>,
    pub ts: Option<String>,
}

/// Sends a message to a channel.
///
/// Wraps https://api.slack.com/methods/chat.postMessage

api_call!(
    post_message,
    "chat.postMessage",
    PostMessageRequest,
    PostMessageResponse
);

#[derive(Clone, Default, Debug, Serialize)]
pub struct PostMessageRequest<'a> {
    /// Channel, private group, or IM channel to send message to. Can be an encoded ID, or a name. See below for more details.
    pub channel: &'a str,
    /// Text of the message to send. See below for an explanation of formatting. This field is usually required, unless you're providing only attachments instead.
    pub text: &'a str,
    /// Change how messages are treated. Defaults to none. See below.
    pub parse: Option<&'a str>,
    /// Find and link channel names and usernames.
    pub link_names: Option<bool>,
    /// Structured message attachments.
    pub attachments: Option<&'a str>,
    /// Pass true to enable unfurling of primarily text-based content.
    pub unfurl_links: Option<bool>,
    /// Pass false to disable unfurling of media content.
    pub unfurl_media: Option<bool>,
    /// Set your bot's user name. Must be used in conjunction with as_user set to false, otherwise ignored. See authorship below.
    pub username: Option<&'a str>,
    /// Pass true to post the message as the authed user, instead of as a bot. Defaults to false. See authorship below.
    pub as_user: Option<bool>,
    /// URL to an image to use as the icon for this message. Must be used in conjunction with as_user set to false, otherwise ignored. See authorship below.
    pub icon_url: Option<&'a str>,
    /// Emoji to use as the icon for this message. Overrides icon_url. Must be used in conjunction with as_user set to false, otherwise ignored. See authorship below.
    pub icon_emoji: Option<&'a str>,
    /// Provide another message's ts value to make this message a reply. Avoid using a reply's ts value; use its parent instead.
    pub thread_ts: Option<&'a str>,
    /// Used in conjunction with thread_ts and indicates whether reply should be made visible to everyone in the channel or conversation. Defaults to false.
    pub reply_broadcast: Option<bool>,
    /// Disable Slack markup parsing by setting to false. Enabled by default.
    pub mrkdwn: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PostMessageResponse {
    ok: bool,
    pub channel: String,
    pub message: Message,
    pub ts: String,
}

/// Unfurl a URL that a user posted
///
/// Wraps https://api.slack.com/methods/chat.unfurl

api_call!(unfurl, "chat.unfurl", UnfurlRequest => ());

#[derive(Clone, Default, Debug, Serialize)]
pub struct UnfurlRequest<'a> {
    /// Channel ID of the message
    pub channel: &'a str,
    /// Timestamp of the message to add unfurl behavior to
    pub ts: &'a str,
    /// JSON mapping a set of URLs from the message to their unfurl attachments
    pub unfurls: &'a str,
    /// Set to true or 1 to indicate the user must install your Slack app to trigger unfurls for this domain
    pub user_auth_required: Option<bool>,
    /// Provide a simply-formatted string to send as an ephemeral message to the user as invitation to authenticate further and enable full unfurling behavior
    pub user_auth_message: Option<&'a str>,
    /// Send users to this custom URL where they will complete authentication in your app to fully trigger unfurling. Value should be properly URL-encoded.
    pub user_auth_url: Option<&'a str>,
}

/// Updates a message.
///
/// Wraps https://api.slack.com/methods/chat.update

api_call!(update, "chat.update", UpdateRequest, UpdateResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct UpdateRequest<'a> {
    /// Timestamp of the message to be updated.
    pub ts: &'a str,
    /// Channel containing the message to be updated.
    pub channel: &'a str,
    /// New text for the message, using the default formatting rules.
    pub text: &'a str,
    /// Structured message attachments.
    pub attachments: Option<&'a str>,
    /// Change how messages are treated. Defaults to client, unlike chat.postMessage. See below.
    pub parse: Option<&'a str>,
    /// Find and link channel names and usernames. Defaults to none. This parameter should be used in conjunction with parse. To set link_names to 1, specify a parse mode of full.
    pub link_names: Option<bool>,
    /// Pass true to update the message as the authed user. Bot users in this context are considered authed users.
    pub as_user: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateResponse {
    ok: bool,
    pub channel: String,
    pub text: String,
    pub ts: String,
}
