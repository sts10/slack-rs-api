//! Get info on your team's Slack channels, create or archive channels, invite users, set the topic and purpose, and mark a channel as read.

#[macro_use]
extern crate derive_error;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_qs;

#[macro_use]
extern crate requests;
use requests::get_slack_url_for_method;

extern crate types;
use types::*;

/// Archives a channel.
///
/// Wraps https://api.slack.com/methods/channels.archive
api_call!(archive, "channels.archive", ArchiveRequest, Result<ArchiveResponse, ArchiveError>);

#[derive(Clone, Default, Debug, Serialize)]
pub struct ArchiveRequest<'a> {
    /// Channel to archive
    pub channel: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArchiveResponse {
    ok: bool,
}

#[derive(Debug, Deserialize, Error)]
#[serde(rename_all = "snake_case")]
#[error(non_std)]
pub enum ArchiveError {
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// Channel has already been archived.
    AlreadyArchived,
    /// You cannot archive the general channel
    CantArchiveGeneral,
    /// A team preference prevents the authenticated user from archiving.
    RestrictedAction,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// This method cannot be called by a bot user.
    UserIsBot,
    /// This method cannot be called by a restricted user or single channel guest.
    UserIsRestricted,
    /// The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.
    InvalidArgName,
    /// The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.
    InvalidArrayArg,
    /// The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.
    InvalidCharset,
    /// The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.
    InvalidFormData,
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    #[serde(skip_deserializing)]
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    #[serde(skip_deserializing)]
    #[error(msg_embedded, no_from, non_std)]
    Unknown(String),
    /// The client had an error sending the request to Slack
    #[serde(skip_deserializing)]
    Client(::reqwest::Error),
}

/// Creates a channel.
///
/// Wraps https://api.slack.com/methods/channels.create
api_call!(create, "channels.create", CreateRequest, Result<CreateResponse, CreateError>);

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
    pub channel: Channel,
}

#[derive(Debug, Deserialize, Error)]
#[serde(rename_all = "snake_case")]
#[error(non_std)]
pub enum CreateError {
    /// A channel cannot be created with the given name.
    NameTaken,
    /// A team preference prevents the authenticated user from creating channels.
    RestrictedAction,
    /// Value passed for name was empty.
    NoChannel,
    /// Value passed for name was empty.
    InvalidNameRequired,
    /// Value passed for name contained only punctuation.
    InvalidNamePunctuation,
    /// Value passed for name exceeded max length.
    InvalidNameMaxlength,
    /// Value passed for name contained unallowed special characters or upper case characters.
    InvalidNameSpecials,
    /// Value passed for name was invalid.
    InvalidName,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// This method cannot be called by a bot user.
    UserIsBot,
    /// This method cannot be called by a restricted user or single channel guest.
    UserIsRestricted,
    /// The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.
    InvalidArgName,
    /// The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.
    InvalidArrayArg,
    /// The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.
    InvalidCharset,
    /// The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.
    InvalidFormData,
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    #[serde(skip_deserializing)]
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    #[serde(skip_deserializing)]
    #[error(msg_embedded, no_from, non_std)]
    Unknown(String),
    /// The client had an error sending the request to Slack
    #[serde(skip_deserializing)]
    Client(::reqwest::Error),
}

/// Fetches history of messages and events from a channel.
///
/// Wraps https://api.slack.com/methods/channels.history
api_call!(history, "channels.history", HistoryRequest, Result<HistoryResponse, HistoryError>);

#[derive(Clone, Default, Debug, Serialize)]
pub struct HistoryRequest<'a> {
    /// Channel to fetch history for.
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
    pub latest: Option<::Timestamp>,
    pub messages: Vec<::Message>,
    pub is_limited: Option<bool>,
}

#[derive(Debug, Deserialize, Error)]
#[serde(rename_all = "snake_case")]
#[error(non_std)]
pub enum HistoryError {
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// Value passed for latest was invalid
    InvalidTsLatest,
    /// Value passed for oldest was invalid
    InvalidTsOldest,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.
    InvalidArgName,
    /// The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.
    InvalidArrayArg,
    /// The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.
    InvalidCharset,
    /// The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.
    InvalidFormData,
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    #[serde(skip_deserializing)]
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    #[serde(skip_deserializing)]
    #[error(msg_embedded, no_from, non_std)]
    Unknown(String),
    /// The client had an error sending the request to Slack
    #[serde(skip_deserializing)]
    Client(::reqwest::Error),
}

/// Gets information about a channel.
///
/// Wraps https://api.slack.com/methods/channels.info
api_call!(info, "channels.info", InfoRequest, Result<InfoResponse, InfoError>);

#[derive(Clone, Default, Debug, Serialize)]
pub struct InfoRequest<'a> {
    /// Channel to get info on
    pub channel: &'a str,
    pub include_locale: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InfoResponse {
    ok: bool,
    pub channel: Channel,
}

#[derive(Debug, Deserialize, Error)]
#[serde(rename_all = "snake_case")]
#[error(non_std)]
pub enum InfoError {
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.
    InvalidArgName,
    /// The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.
    InvalidArrayArg,
    /// The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.
    InvalidCharset,
    /// The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.
    InvalidFormData,
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    #[serde(skip_deserializing)]
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    #[serde(skip_deserializing)]
    #[error(msg_embedded, no_from, non_std)]
    Unknown(String),
    /// The client had an error sending the request to Slack
    #[serde(skip_deserializing)]
    Client(::reqwest::Error),
}

/// Invites a user to a channel.
///
/// Wraps https://api.slack.com/methods/channels.invite
api_call!(invite, "channels.invite", InviteRequest, Result<InviteResponse, InviteError>);

#[derive(Clone, Default, Debug, Serialize)]
pub struct InviteRequest<'a> {
    /// Channel to invite user to.
    pub channel: &'a str,
    /// User to invite to channel.
    pub user: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InviteResponse {
    ok: bool,
    pub channel: Channel,
}

#[derive(Debug, Deserialize, Error)]
#[serde(rename_all = "snake_case")]
#[error(non_std)]
pub enum InviteError {
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// Value passed for user was invalid.
    UserNotFound,
    /// Authenticated user cannot invite themselves to a channel.
    CantInviteSelf,
    /// Authenticated user is not in the channel.
    NotInChannel,
    /// Invited user is already in the channel.
    AlreadyInChannel,
    /// Channel has been archived.
    IsArchived,
    /// User cannot be invited to this channel.
    CantInvite,
    /// URA is already in the maximum number of channels.
    UraMaxChannels,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// This method cannot be called by a bot user.
    UserIsBot,
    /// This method cannot be called by a single channel guest.
    UserIsUltraRestricted,
    /// The method was passed an argument whose name falls outside the bounds of common decency.
    /// This includes very long names and names with non-alphanumeric characters other than _.
    /// If you get this error, it is typically an indication that you have made a very malformed API call.
    InvalidArgName,
    /// The method was passed a PHP-style array argument (e.g. with a name like foo[7]).
    /// These are never valid with the Slack API.
    InvalidArrayArg,
    /// The method was called via a POST request, but the charset specified in the Content-Type header was invalid.
    /// Valid charset names are: utf-8 iso-8859-1.
    InvalidCharset,
    /// The method was called via a POST request with Content-Type application/x-www-form-urlencoded or
    /// multipart/form-data, but the form data was either missing or syntactically invalid.
    InvalidFormData,
    /// The method was called via a POST request, but the specified Content-Type was invalid.
    /// Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload,
    /// but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization.
    /// Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    #[serde(skip_deserializing)]
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    #[serde(skip_deserializing)]
    #[error(msg_embedded, no_from, non_std)]
    Unknown(String),
    /// The client had an error sending the request to Slack
    #[serde(skip_deserializing)]
    Client(::reqwest::Error),
}

/// Joins a channel, creating it if needed.
///
/// Wraps https://api.slack.com/methods/channels.join
api_call!(join, "channels.join", JoinRequest, Result<JoinResponse, JoinError>);

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
    pub channel: Channel,
    pub already_in_channel: bool,
}

#[derive(Debug, Deserialize, Error)]
#[serde(rename_all = "snake_case")]
#[error(non_std)]
pub enum JoinError {
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// A channel cannot be created with the given name.
    NameTaken,
    /// A team preference prevents the authenticated user from creating channels.
    RestrictedAction,
    /// Value passed for name was empty.
    NoChannel,
    /// Channel has been archived.
    IsArchived,
    /// Value passed for name was empty.
    InvalidNameRequired,
    /// Value passed for name contained only punctuation.
    InvalidNamePunctuation,
    /// Value passed for name exceeded max length.
    InvalidNameMaxlength,
    /// Value passed for name contained unallowed special characters or upper case characters.
    InvalidNameSpecials,
    /// Value passed for name was invalid.
    InvalidName,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// This method cannot be called by a bot user.
    UserIsBot,
    /// This method cannot be called by a restricted user or single channel guest.
    UserIsRestricted,
    /// The method was passed an argument whose name falls outside the bounds of common decency.
    /// This includes very long names and names with non-alphanumeric characters other than _.
    /// If you get this error, it is typically an indication that you have made a very malformed API call.
    InvalidArgName,
    /// The method was passed a PHP-style array argument (e.g. with a name like foo[7]).
    /// These are never valid with the Slack API.
    InvalidArrayArg,
    /// The method was called via a POST request, but the charset specified in the Content-Type header was invalid.
    /// Valid charset names are: utf-8 iso-8859-1.
    InvalidCharset,
    /// The method was called via a POST request with Content-Type application/x-www-form-urlencoded
    /// or multipart/form-data, but the form data was either missing or syntactically invalid.
    InvalidFormData,
    /// The method was called via a POST request, but the specified Content-Type was invalid.
    /// Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload,
    /// but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization.
    /// Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    #[serde(skip_deserializing)]
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    #[serde(skip_deserializing)]
    #[error(msg_embedded, no_from, non_std)]
    Unknown(String),
    /// The client had an error sending the request to Slack
    #[serde(skip_deserializing)]
    Client(::reqwest::Error),
}

/// Removes a user from a channel.
///
/// Wraps https://api.slack.com/methods/channels.kick
api_call!(kick, "channels.kick", KickRequest, Result<KickResponse, KickError>);

#[derive(Clone, Default, Debug, Serialize)]
pub struct KickRequest<'a> {
    /// Channel to remove user from.
    pub channel: &'a str,
    /// User to remove from channel.
    pub user: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KickResponse {
    ok: bool,
}

#[derive(Debug, Deserialize, Error)]
#[serde(rename_all = "snake_case")]
#[error(non_std)]
pub enum KickError {
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// Value passed for user was invalid.
    UserNotFound,
    /// Authenticated user can't kick themselves from a channel.
    CantKickSelf,
    /// User was not in the channel.
    NotInChannel,
    /// User cannot be removed from #general.
    CantKickFromGeneral,
    /// A team preference prevents the authenticated user from kicking.
    RestrictedAction,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// This method cannot be called by a bot user.
    UserIsBot,
    /// This method cannot be called by a restricted user or single channel guest.
    UserIsRestricted,
    /// The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.
    InvalidArgName,
    /// The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.
    InvalidArrayArg,
    /// The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.
    InvalidCharset,
    /// The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.
    InvalidFormData,
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    #[serde(skip_deserializing)]
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    #[serde(skip_deserializing)]
    #[error(msg_embedded, no_from, non_std)]
    Unknown(String),
    /// The client had an error sending the request to Slack
    #[serde(skip_deserializing)]
    Client(::reqwest::Error),
}

/// Leaves a channel.
///
/// Wraps https://api.slack.com/methods/channels.leave
api_call!(leave, "channels.leave", LeaveRequest, Result<LeaveResponse, LeaveError>);

#[derive(Clone, Default, Debug, Serialize)]
pub struct LeaveRequest<'a> {
    /// Channel to leave
    pub channel: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LeaveResponse {
    ok: bool,
    not_in_channel: Option<bool>,
}

#[derive(Debug, Deserialize, Error)]
#[serde(rename_all = "snake_case")]
#[error(non_std)]
pub enum LeaveError {
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// Channel has been archived.
    IsArchived,
    /// Authenticated user cannot leave the general channel
    CantLeaveGeneral,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// This method cannot be called by a bot user.
    UserIsBot,
    /// This method cannot be called by a restricted user or single channel guest.
    UserIsRestricted,
    /// The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.
    InvalidArgName,
    /// The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.
    InvalidArrayArg,
    /// The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.
    InvalidCharset,
    /// The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.
    InvalidFormData,
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    #[serde(skip_deserializing)]
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    #[serde(skip_deserializing)]
    #[error(msg_embedded, no_from, non_std)]
    Unknown(String),
    /// The client had an error sending the request to Slack
    #[serde(skip_deserializing)]
    Client(::reqwest::Error),
}

/// Lists all channels in a Slack team.
///
/// Wraps https://api.slack.com/methods/channels.list
api_call!(list, "channels.list", ListRequest, Result<ListResponse, ListError>);

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

#[derive(Debug, Deserialize, Error)]
#[serde(rename_all = "snake_case")]
#[error(non_std)]
pub enum ListError {
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.
    InvalidArgName,
    /// The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.
    InvalidArrayArg,
    /// The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.
    InvalidCharset,
    /// The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.
    InvalidFormData,
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    #[serde(skip_deserializing)]
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    #[serde(skip_deserializing)]
    #[error(msg_embedded, no_from, non_std)]
    Unknown(String),
    /// The client had an error sending the request to Slack
    #[serde(skip_deserializing)]
    Client(::reqwest::Error),
}

/// Sets the read cursor in a channel.
///
/// Wraps https://api.slack.com/methods/channels.mark
api_call!(mark, "channels.mark", MarkRequest, Result<MarkResponse, MarkError>);

#[derive(Clone, Default, Debug, Serialize)]
pub struct MarkRequest<'a> {
    /// Channel to set reading cursor in.
    pub channel: &'a str,
    /// Timestamp of the most recently seen message.
    pub ts: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MarkResponse {
    ok: bool,
}

#[derive(Debug, Deserialize, Error)]
#[serde(rename_all = "snake_case")]
#[error(non_std)]
pub enum MarkError {
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// Value passed for timestamp was invalid.
    InvalidTimestamp,
    /// Caller is not a member of the channel.
    NotInChannel,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.
    InvalidArgName,
    /// The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.
    InvalidArrayArg,
    /// The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.
    InvalidCharset,
    /// The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.
    InvalidFormData,
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    #[serde(skip_deserializing)]
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    #[serde(skip_deserializing)]
    #[error(msg_embedded, no_from, non_std)]
    Unknown(String),
    /// The client had an error sending the request to Slack
    #[serde(skip_deserializing)]
    Client(::reqwest::Error),
}

/// Renames a channel.
///
/// Wraps https://api.slack.com/methods/channels.rename
api_call!(rename, "channels.rename", RenameRequest, Result<RenameResponse, RenameError>);

#[derive(Clone, Default, Debug, Serialize)]
pub struct RenameRequest<'a> {
    /// Channel to rename
    pub channel: &'a str,
    /// New name for channel.
    pub name: &'a str,
    /// Whether to return errors on invalid channel name instead of modifying it to meet the specified criteria.
    pub validate: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RenameResponse {
    ok: bool,
    pub channel: Channel,
}

#[derive(Debug, Deserialize, Error)]
#[serde(rename_all = "snake_case")]
#[error(non_std)]
pub enum RenameError {
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// Caller is not a member of the channel.
    NotInChannel,
    /// Caller cannot rename this channel
    NotAuthorized,
    /// Value passed for name was invalid.
    InvalidName,
    /// New channel name is taken
    NameTaken,
    /// Value passed for name was empty.
    InvalidNameRequired,
    /// Value passed for name contained only punctuation.
    InvalidNamePunctuation,
    /// Value passed for name exceeded max length.
    InvalidNameMaxlength,
    /// Value passed for name contained unallowed special characters or upper case characters.
    InvalidNameSpecials,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// This method cannot be called by a bot user.
    UserIsBot,
    /// This method cannot be called by a restricted user or single channel guest.
    UserIsRestricted,
    /// The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.
    InvalidArgName,
    /// The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.
    InvalidArrayArg,
    /// The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.
    InvalidCharset,
    /// The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.
    InvalidFormData,
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    #[serde(skip_deserializing)]
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    #[serde(skip_deserializing)]
    #[error(msg_embedded, no_from, non_std)]
    Unknown(String),
    /// The client had an error sending the request to Slack
    #[serde(skip_deserializing)]
    Client(::reqwest::Error),
}

/// Retrieve a thread of messages posted to a channel
///
/// Wraps https://api.slack.com/methods/channels.replies
api_call!(replies, "channels.replies", RepliesRequest, Result<RepliesResponse, RepliesError>);

#[derive(Clone, Default, Debug, Serialize)]
pub struct RepliesRequest<'a> {
    /// Channel to fetch thread from
    pub channel: &'a str,
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

#[derive(Debug, Deserialize, Error)]
#[serde(rename_all = "snake_case")]
#[error(non_std)]
pub enum RepliesError {
    /// Value for channel was missing or invalid.
    ChannelNotFound,
    /// Value for thread_ts was missing or invalid.
    ThreadNotFound,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.
    InvalidArgName,
    /// The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.
    InvalidArrayArg,
    /// The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.
    InvalidCharset,
    /// The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.
    InvalidFormData,
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    #[serde(skip_deserializing)]
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    #[serde(skip_deserializing)]
    #[error(msg_embedded, no_from, non_std)]
    Unknown(String),
    /// The client had an error sending the request to Slack
    #[serde(skip_deserializing)]
    Client(::reqwest::Error),
}

/// Sets the purpose for a channel.
///
/// Wraps https://api.slack.com/methods/channels.setPurpose
api_call!(set_purpose, "channels.setPurpose", SetPurposeRequest, Result<SetPurposeResponse, SetPurposeError>);

#[derive(Clone, Default, Debug, Serialize)]
pub struct SetPurposeRequest<'a> {
    /// Channel to set the purpose of
    pub channel: &'a str,
    /// The new purpose
    pub purpose: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SetPurposeResponse {
    ok: bool,
    pub purpose: String,
}

#[derive(Debug, Deserialize, Error)]
#[serde(rename_all = "snake_case")]
#[error(non_std)]
pub enum SetPurposeError {
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// Authenticated user is not in the channel.
    NotInChannel,
    /// Channel has been archived.
    IsArchived,
    /// Purpose was longer than 250 characters.
    TooLong,
    /// This method cannot be called by a restricted user or single channel guest.
    UserIsRestricted,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.
    InvalidArgName,
    /// The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.
    InvalidArrayArg,
    /// The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.
    InvalidCharset,
    /// The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.
    InvalidFormData,
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    #[serde(skip_deserializing)]
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    #[serde(skip_deserializing)]
    #[error(msg_embedded, no_from, non_std)]
    Unknown(String),
    /// The client had an error sending the request to Slack
    #[serde(skip_deserializing)]
    Client(::reqwest::Error),
}

/// Sets the topic for a channel.
///
/// Wraps https://api.slack.com/methods/channels.setTopic
api_call!(set_topic, "channels.setTopic", SetTopicRequest, Result<SetTopicResponse, SetTopicError>);

#[derive(Clone, Default, Debug, Serialize)]
pub struct SetTopicRequest<'a> {
    /// Channel to set the topic of
    pub channel: &'a str,
    /// The new topic
    pub topic: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SetTopicResponse {
    ok: bool,
    pub topic: String,
}

#[derive(Debug, Deserialize, Error)]
#[serde(rename_all = "snake_case")]
#[error(non_std)]
pub enum SetTopicError {
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// Authenticated user is not in the channel.
    NotInChannel,
    /// Channel has been archived.
    IsArchived,
    /// Topic was longer than 250 characters.
    TooLong,
    /// This method cannot be called by a restricted user or single channel guest.
    UserIsRestricted,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.
    InvalidArgName,
    /// The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.
    InvalidArrayArg,
    /// The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.
    InvalidCharset,
    /// The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.
    InvalidFormData,
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    #[serde(skip_deserializing)]
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    #[serde(skip_deserializing)]
    #[error(msg_embedded, no_from, non_std)]
    Unknown(String),
    /// The client had an error sending the request to Slack
    #[serde(skip_deserializing)]
    Client(::reqwest::Error),
}

/// Unarchives a channel.
///
/// Wraps https://api.slack.com/methods/channels.unarchive
api_call!(unarchive, "channels.unarchive", UnarchiveRequest, Result<UnarchiveResponse, UnarchiveError>);

#[derive(Clone, Default, Debug, Serialize)]
pub struct UnarchiveRequest<'a> {
    /// Channel to unarchive
    pub channel: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnarchiveResponse {
    ok: bool,
}

#[derive(Debug, Deserialize, Error)]
#[serde(rename_all = "snake_case")]
#[error(non_std)]
pub enum UnarchiveError {
    /// Value passed for channel was invalid.
    ChannelNotFound,
    /// Channel is not archived.
    NotArchived,
    /// No authentication token provided.
    NotAuthed,
    /// Invalid authentication token.
    InvalidAuth,
    /// Authentication token is for a deleted user or team.
    AccountInactive,
    /// This method cannot be called by a bot user.
    UserIsBot,
    /// This method cannot be called by a restricted user or single channel guest.
    UserIsRestricted,
    /// The method was passed an argument whose name falls outside the bounds of common decency. This includes very long names and names with non-alphanumeric characters other than _. If you get this error, it is typically an indication that you have made a very malformed API call.
    InvalidArgName,
    /// The method was passed a PHP-style array argument (e.g. with a name like foo[7]). These are never valid with the Slack API.
    InvalidArrayArg,
    /// The method was called via a POST request, but the charset specified in the Content-Type header was invalid. Valid charset names are: utf-8 iso-8859-1.
    InvalidCharset,
    /// The method was called via a POST request with Content-Type application/x-www-form-urlencoded or multipart/form-data, but the form data was either missing or syntactically invalid.
    InvalidFormData,
    /// The method was called via a POST request, but the specified Content-Type was invalid. Valid types are: application/x-www-form-urlencoded multipart/form-data text/plain.
    InvalidPostType,
    /// The method was called via a POST request and included a data payload, but the request did not include a Content-Type header.
    MissingPostType,
    /// The team associated with your request is currently undergoing migration to an Enterprise Organization. Web API and other platform operations will be intermittently unavailable until the transition is complete.
    TeamAddedToOrg,
    /// The method was called via a POST request, but the POST data was either missing or truncated.
    RequestTimeout,
    /// The response was not parseable as the expected object
    #[serde(skip_deserializing)]
    MalformedResponse(serde_json::error::Error),
    /// The response returned an error that was unknown to the library
    #[serde(skip_deserializing)]
    #[error(msg_embedded, no_from, non_std)]
    Unknown(String),
    /// The client had an error sending the request to Slack
    #[serde(skip_deserializing)]
    Client(::reqwest::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_create() {
        let client = requests::default_client().unwrap();
        let token = env::var("SLACK_API_TOKEN").unwrap();
        match create(
            &client,
            &token,
            &CreateRequest {
                name: "testchannel",
                validate: Some(false),
            },
        ) {
            Ok(_) => {},
            Err(CreateError::NameTaken) => {},
            Err(e) => panic!(e),
        }
    }

    #[test]
    fn test_list() {
        let client = requests::default_client().unwrap();
        let token = env::var("SLACK_API_TOKEN").unwrap();
        let response = list(&client, &token, &ListRequest::default()).unwrap();
        assert_eq!(response.channels.len(), 3);
    }

    #[test]
    fn test_history() {
        let client = requests::default_client().unwrap();
        let token = env::var("SLACK_API_TOKEN").unwrap();
        let mut req = HistoryRequest::default();
        req.channel = "CAGMCM14K";
        let _response = history(&client, &token, &req).unwrap();
    }

    #[test]
    fn test_archive() {
        let client = requests::default_client().unwrap();
        let token = env::var("SLACK_API_TOKEN").unwrap();

        let mut req = UnarchiveRequest::default();
        req.channel = "CAGMCM14K";
        unarchive(&client, &token, &req);

        let mut req = ArchiveRequest::default();
        req.channel = "CAGMCM14K";
        archive(&client, &token, &req).unwrap();

        let mut req = UnarchiveRequest::default();
        req.channel = "CAGMCM14K";
        unarchive(&client, &token, &req);
    }

    #[test]
    fn test_unarchive() {
        let client = requests::default_client().unwrap();
        let token = env::var("SLACK_API_TOKEN").unwrap();
        let mut req = ArchiveRequest::default();
        req.channel = "CAGMCM14K";
        archive(&client, &token, &req);
        let mut req = UnarchiveRequest::default();
        req.channel = "CAGMCM14K";
        unarchive(&client, &token, &req).unwrap();
    }

    #[test]
    fn test_info() {
        let client = requests::default_client().unwrap();
        let token = env::var("SLACK_API_TOKEN").unwrap();
        let mut req = InfoRequest::default();
        req.channel = "CAGMCM14K";
        info(&client, &token, &req).unwrap();
    }

    #[test]
    fn test_invite() {
        let client = requests::default_client().unwrap();
        let token = env::var("SLACK_API_TOKEN").unwrap();
        let mut req = InviteRequest::default();
        req.channel = "CAGMCM14K";
        req.user = "U9WDA1CGN";
        invite(&client, &token, &req).unwrap();
    }

    #[test]
    fn test_set_topic() {
        let client = requests::default_client().unwrap();
        let token = env::var("SLACK_API_TOKEN").unwrap();
        let mut req = SetTopicRequest::default();
        req.channel = "CAGMCM14K";
        req.topic = "test_topic";

        let response = set_topic(&client, &token, &req).unwrap();
        assert_eq!(response.topic, "test_topic");

        req.topic = "other_test_topic";
        let response = set_topic(&client, &token, &req).unwrap();
        assert_eq!(response.topic, "other_test_topic");
    }

    #[test]
    fn test_set_purpose() {
        let client = requests::default_client().unwrap();
        let token = env::var("SLACK_API_TOKEN").unwrap();
        let mut req = SetPurposeRequest::default();
        req.channel = "CAGMCM14K";
        req.purpose = "test_purpose";

        let response = set_purpose(&client, &token, &req).unwrap();
        assert_eq!(response.purpose, "test_purpose");

        req.purpose = "other_test_purpose";
        let response = set_purpose(&client, &token, &req).unwrap();
        assert_eq!(response.purpose, "other_test_purpose");
    }

    #[test]
    fn test_replies() {
        let client = requests::default_client().unwrap();
        let token = env::var("SLACK_API_TOKEN").unwrap();
        let mut req = RepliesRequest::default();
        req.channel = "CAGMCM14K";
        replies(&client, &token, &req).unwrap();
    }

    #[test]
    fn test_rename() {
        let client = requests::default_client().unwrap();
        let token = env::var("SLACK_API_TOKEN").unwrap();
        let mut req = RenameRequest::default();
        req.channel = "CAGMCM14K";
        req.name = "testchannel";
        rename(&client, &token, &req).unwrap();

        req.name = "other_testchannel";
        rename(&client, &token, &req).unwrap();

        req.name = "testchannel";
        rename(&client, &token, &req).unwrap();
    }

}
