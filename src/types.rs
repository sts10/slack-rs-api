use serde::de::{self, Deserialize, Deserializer, Visitor};
use std::collections::HashMap;
use std::fmt;
use uuid::Uuid;

use id::*;
use timestamp::Timestamp;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ChannelName {
    len: u8,
    buf: [u8; 22],
}

impl ::std::fmt::Display for ChannelName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        for c in self.buf.iter().take(self.len as usize) {
            write!(f, "{}", *c as char)?;
        }
        Ok(())
    }
}

impl ::std::fmt::Debug for ChannelName {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "\"{}\"", self).map(|_| ())
    }
}

struct ChannelNameVisitor;

impl<'de> Visitor<'de> for ChannelNameVisitor {
    type Value = ChannelName;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a 9-byte str")
    }

    fn visit_str<E>(self, value: &str) -> Result<ChannelName, E>
    where
        E: de::Error,
    {
        if value.len() < 22 {
            let mut ret = ChannelName::default();
            ret.len = value.len() as u8;
            ret.buf[..value.len()].copy_from_slice(value.as_bytes());
            Ok(ret)
        } else {
            Err(E::custom(format!(
                "Channel names must be shorter than 22 characters,found {:?}",
                value,
            )))
        }
    }
}

impl<'de> Deserialize<'de> for ChannelName {
    fn deserialize<D>(deserializer: D) -> Result<ChannelName, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ChannelNameVisitor)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Bot {
    pub app_id: Option<AppId>,
    pub deleted: Option<bool>,
    pub icons: Option<BotIcons>,
    pub id: BotId,
    pub name: String,
    pub updated: Option<Timestamp>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BotIcons {
    pub image_36: Option<String>,
    pub image_48: Option<String>,
    pub image_72: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Channel {
    pub accepted_user: Option<UserId>,
    pub created: Option<Timestamp>,
    pub creator: Option<String>,
    pub id: ChannelId,
    pub is_archived: Option<bool>,
    pub is_channel: Option<bool>,
    pub is_general: Option<bool>,
    pub is_member: Option<bool>,
    pub is_moved: Option<i32>,
    pub is_mpim: Option<bool>,
    pub is_org_shared: Option<bool>,
    pub is_pending_ext_shared: Option<bool>,
    pub is_private: Option<bool>,
    pub is_read_only: Option<bool>,
    pub is_shared: Option<bool>,
    pub last_read: Option<Timestamp>,
    pub latest: Option<::Message>,
    pub members: Option<Vec<UserId>>,
    pub name: String,
    pub name_normalized: Option<String>,
    pub num_members: Option<i32>,
    pub previous_names: Option<Vec<String>>,
    pub priority: Option<i32>,
    pub purpose: Option<ChannelPurpose>,
    pub topic: Option<ChannelTopic>,
    pub unlinked: Option<i32>,
    pub unread_count: Option<i32>,
    pub unread_count_display: Option<i32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ChannelPurpose {
    pub creator: Option<String>,
    pub last_set: Option<Timestamp>,
    pub value: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ChannelTopic {
    pub creator: Option<String>,
    pub last_set: Option<Timestamp>,
    pub value: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct File {
    pub channels: Option<Vec<String>>,
    pub comments_count: Option<i32>,
    pub created: Option<i32>,
    pub display_as_bot: Option<bool>,
    pub edit_link: Option<String>,
    pub editable: Option<bool>,
    pub external_type: Option<String>,
    pub filetype: Option<String>,
    pub groups: Option<Vec<String>>,
    pub id: Option<String>,
    pub ims: Option<Vec<String>>,
    pub initial_comment: Option<::FileComment>,
    pub is_external: Option<bool>,
    pub is_public: Option<bool>,
    pub is_starred: Option<bool>,
    pub lines: Option<i32>,
    pub lines_more: Option<i32>,
    pub mimetype: Option<String>,
    pub mode: Option<String>,
    pub name: Option<String>,
    pub num_stars: Option<i32>,
    pub permalink: Option<String>,
    pub permalink_public: Option<String>,
    pub pinned_to: Option<Vec<String>>,
    pub pretty_type: Option<String>,
    pub preview: Option<String>,
    pub preview_highlight: Option<String>,
    pub public_url_shared: Option<bool>,
    #[serde(default)]
    pub reactions: Vec<::Reaction>,
    pub size: Option<i32>,
    pub thumb_160: Option<String>,
    pub thumb_360: Option<String>,
    pub thumb_360_gif: Option<String>,
    pub thumb_360_h: Option<i32>,
    pub thumb_360_w: Option<i32>,
    pub thumb_480: Option<String>,
    pub thumb_480_h: Option<i32>,
    pub thumb_480_w: Option<i32>,
    pub thumb_64: Option<String>,
    pub thumb_80: Option<String>,
    pub timestamp: Option<i32>,
    pub title: Option<String>,
    pub url_private: Option<String>,
    pub url_private_download: Option<String>,
    pub user: Option<UserId>,
    pub username: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct FileComment {
    pub comment: Option<String>,
    pub id: Option<String>,
    #[serde(default)]
    pub reactions: Vec<::Reaction>,
    pub timestamp: Option<i32>,
    pub user: Option<UserId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Group {
    pub created: Option<i32>,
    pub creator: Option<String>,
    pub id: GroupId,
    pub is_archived: Option<bool>,
    pub is_group: Option<bool>,
    pub is_mpim: Option<bool>,
    pub last_read: Option<Timestamp>,
    pub latest: Option<::Message>,
    pub members: Option<Vec<String>>,
    pub name: String,
    pub purpose: Option<GroupPurpose>,
    pub topic: Option<GroupTopic>,
    pub unread_count: Option<i32>,
    pub unread_count_display: Option<i32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GroupPurpose {
    pub creator: Option<String>,
    pub last_set: Option<i32>,
    pub value: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GroupTopic {
    pub creator: Option<String>,
    pub last_set: Option<i32>,
    pub value: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Im {
    pub created: Option<i32>,
    pub id: Option<String>,
    pub is_im: Option<bool>,
    pub is_user_deleted: Option<bool>,
    pub user: Option<UserId>,
}

macro_rules! deserialize_internally_tagged {
    {
        tag_field = $tagfield:expr,
        default_variant = $default_variant:ident,
        default_struct = $default_struct:ty,
        $(#[$attr:meta])*
        pub enum $enumname:ident {
            $($variant_name:ident($struct_name:ty)),*,
        }
   } => {

        $(#[$attr])*
        pub enum $enumname {
            $($variant_name($struct_name),)*
        }

        impl<'de> ::serde::Deserialize<'de> for $enumname {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                D: ::serde::Deserializer<'de>,
            {
                let v: ::serde_json::Value = ::serde::Deserialize::deserialize(deserializer)?;

                #[derive(Deserialize)]
                #[serde(field_identifier, rename_all = "snake_case")]
                enum Tag {
                    $($variant_name,)*
                }

                match Option::deserialize(&v[$tagfield]).map_err(::serde::de::Error::custom)? {
                    $(
                    Some(Tag::$variant_name) => {
                        ::serde::Deserialize::deserialize(v)
                        .map($enumname::$variant_name)
                        .map_err(|e| ::serde::de::Error::custom(format!("{} while deserializing {}", e, stringify!($struct_name))))
                    }
                    )*
                    None => {
                        ::serde::Deserialize::deserialize(v)
                            .map($enumname::$default_variant)
                            .map_err(|e| ::serde::de::Error::custom(format!("{} while deserializing {}", e, stringify!($default_struct))))
                    }
                }
            }
        }
    };
    // Impl for when there is no default
    {
        tag_field = $tagfield:expr,
        $(#[$attr:meta])*
        pub enum $enumname:ident {
            $($variant_name:ident($struct_name:ty)),*,
        }
   } => {

        $(#[$attr])*
        pub enum $enumname {
            $($variant_name($struct_name),)*
        }

        impl<'de> ::serde::Deserialize<'de> for $enumname {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                D: ::serde::Deserializer<'de>,
            {
                #[derive(Deserialize)]
                #[serde(field_identifier, rename_all = "snake_case")]
                enum Tag {
                    $($variant_name,)*
                }

                let mut v: ::serde_json::Value = ::serde::Deserialize::deserialize(deserializer)?;

                // Remove the tag from the Value so that we can deny_unknown_fields further down
                let tag = v
                    .as_object_mut()
                    .and_then(|obj| obj.remove($tagfield))
                    .ok_or_else(|| ::serde::de::Error::custom(format!("cannot deserialize {} without a tag", stringify!($enumname))))
                    .and_then(|str_tag| Tag::deserialize(str_tag))
                    // No idea how to do this conversion gracefully
                    .map_err(|e| ::serde::de::Error::custom(format!("{}", e)))?;

                match tag {
                    $(
                    Tag::$variant_name => {
                        ::serde::Deserialize::deserialize(v)
                        .map($enumname::$variant_name)
                        .map_err(|e| ::serde::de::Error::custom(format!("{} while deserializing {}", e, stringify!($struct_name))))
                    }
                    )*
                }
            }
        }
    }
}

deserialize_internally_tagged! {
    tag_field = "type",
    #[derive(Clone, Debug)]
    pub enum Event {
        AppsChanged(EventAppsChanged),
        //AccountsChanged,
        //BotAdded,
        BotChanged(EventBotChanged),
        //ChannelArchive,
        //ChannelCreated,
        //ChannelDeleted,
        //ChannelHistoryChanged,
        //ChannelJoined,
        //ChannelLeft,
        ChannelMarked(EventChannelMarked),
        //ChannelRename,
        //ChannelUnarchive,
        //CommandsChanged,
        DndUpdatedUser(EventDndUpdatedUser),
        //EmailDomainChanged,
        FileChange(EventFileChange),
        //FileCommentAdded,
        //FileCommentDeleted,
        //FileCommentEdited,
        FileCreated(EventFileCreated),
        //FileDeleted,
        FilePublic(EventFilePublic),
        FileShared(EventFileShared),
        //FileUnshared,
        //Goodbye,
        //GroupArchive,
        //GroupClose,
        //GroupHistoryChanged,
        //GroupJoined,
        //GroupLeft,
        GroupMarked(EventGroupMarked),
        //GroupOpen,
        //GroupRename,
        //GroupUnarchive,
        Hello(EventHello),
        //ImClose,
        //ImCreated,
        //ImHistoryChanged,
        //ImMarked,
        //ImOpen,
        //ManualPresenceChange,
        //MemberJoinedChannel,
        //MemberLeftChannel,
        Message(Message),
        PinAdded(EventPinAdded),
        //PinRemoved,
        //PrefChange,
        //PresenceChange,
        //PresenceQuery,
        //PresenceSub,
        ReactionAdded(EventReactionAdded),
        //ReactionRemoved,
        //ReconnectUrl, // Experimental?
        //StarAdded,
        //StarRemoved,
        //SubteamCreated,
        //SubteamMembersChanged,
        //SubteamSelfAdded,
        //SubteamUpdated,
        //TeamDomainChange,
        //TeamJoin,
        //TeamMigrationStarted,
        //TeamPlanChange,
        //TeamPrefChange,
        //TeamProfileChange,
        //TeamProfileDelete,
        //TeamProfileReorder,
        //TeamRename,
        UserChange(EventUserChange),
        UserTyping(EventUserTyping),
    }
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventFileChange {
    pub file_id: FileId,
    pub user_id: UserId,
    pub file: JustAFileId,
    pub event_ts: Timestamp,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventPinAdded {
    user: UserId,
    channel_id: ConversationId,
    item: Message,
    item_user: UserId,
    pin_count: i32,
    pinned_info: PinnedInfo,
    event_ts: Timestamp,
    ts: Option<Timestamp>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PinnedInfo {
    channel: ConversationId,
    pinned_by: UserId,
    pinned_ts: Timestamp,
    event_ts: Timestamp,
    ts: Option<Timestamp>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventHello {}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventAppsChanged {
    pub app: App,
    pub event_ts: Timestamp,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct App {
    pub id: AppId,
    pub name: String,
    pub icons: Option<AppIcons>,
    pub deleted: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AppIcons {
    pub image_32: Option<String>,
    pub image_36: Option<String>,
    pub image_48: Option<String>,
    pub image_64: Option<String>,
    pub image_72: Option<String>,
    pub image_96: Option<String>,
    pub image_128: Option<String>,
    pub image_192: Option<String>,
    pub image_512: Option<String>,
    pub image_1024: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventBotChanged {
    bot: Bot,
    cache_ts: Option<Timestamp>,
    event_ts: Timestamp,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventUserTyping {
    pub channel: ConversationId,
    pub user: UserId,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventReactionAdded {
    pub user: UserId,
    pub item: Box<Event>, // This is an event inside of an event, sounds like this could be bad
    pub reaction: String,
    pub item_user: UserId,
    pub event_ts: Timestamp,
    pub ts: Timestamp,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventDndUpdatedUser {
    pub user: UserId,
    pub dnd_status: DndStatus,
    pub event_ts: Timestamp,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DndStatus {
    pub dnd_enabled: bool,
    pub next_dnd_start_ts: Timestamp,
    pub next_dnd_end_ts: Timestamp,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventFileShared {
    pub file_id: FileId,
    pub user_id: UserId,
    pub file: JustAFileId,
    pub event_ts: Timestamp,
    pub ts: Option<Timestamp>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct JustAFileId {
    pub id: FileId,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventFilePublic {
    pub file_id: FileId,
    pub user_id: UserId,
    pub file: JustAFileId,
    pub event_ts: Timestamp,
    pub ts: Option<Timestamp>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventFileCreated {
    pub file: JustAFileId,
    pub file_id: FileId,
    pub user_id: UserId,
    pub event_ts: Timestamp,
    pub ts: Option<Timestamp>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventChannelMarked {
    pub channel: ChannelId,
    pub ts: Timestamp,
    pub unread_count: u32,
    pub unread_count_display: u32,
    pub num_mentions: u32,
    pub num_mentions_display: u32,
    pub mention_count: u32,
    pub mention_count_display: u32,
    pub event_ts: Timestamp,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventGroupMarked {
    pub channel: GroupId,
    pub ts: Timestamp,
    pub unread_count: u32,
    pub unread_count_display: u32,
    pub num_mentions: u32,
    pub num_mentions_display: u32,
    pub mention_count: u32,
    pub mention_count_display: u32,
    pub event_ts: Timestamp,
    pub is_mpim: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventUserChange {
    pub user: User,
    pub cache_ts: Timestamp,
    pub event_ts: Timestamp,
}

deserialize_internally_tagged! {
    tag_field = "subtype",
    default_variant = Standard,
    default_struct = MessageStandard,
    #[derive(Clone, Debug)]
    pub enum Message {
        Standard(MessageStandard),
        BotAdd(MessageBotAdd),
        BotRemove(MessageBotRemove),
        BotMessage(MessageBotMessage),
        ChannelArchive(MessageChannelArchive),
        ChannelJoin(MessageChannelJoin),
        ChannelLeave(MessageChannelLeave),
        ChannelName(MessageChannelName),
        ChannelPurpose(MessageChannelPurpose),
        ChannelTopic(MessageChannelTopic),
        ChannelUnarchive(MessageChannelUnarchive),
        FileComment(Box<MessageFileComment>),
        FileMention(Box<MessageFileMention>),
        FileShare(Box<MessageFileShare>),
        GroupArchive(MessageGroupArchive),
        GroupJoin(MessageGroupJoin),
        GroupLeave(MessageGroupLeave),
        GroupName(MessageGroupName),
        GroupPurpose(MessageGroupPurpose),
        GroupTopic(MessageGroupTopic),
        GroupUnarchive(MessageGroupUnarchive),
        MeMessage(MessageMeMessage),
        //Message(MessageMessageChanged),
        MessageChanged(MessageMessageChanged),
        MessageDeleted(MessageMessageDeleted),
        MessageReplied(MessageMessageReplied),
        PinnedItem(MessagePinnedItem),
        ReplyBroadcast(MessageReplyBroadcast),
        ReminderAdd(MessageReminderAdd),
        SlackbotResponse(MessageSlackbotResponse),
        ThreadBroadcast(Box<MessageThreadBroadcast>),
        UnpinnedItem(MessageUnpinnedItem),
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageChannelMarked {
    pub channel: Option<ChannelId>,
    pub ts: Option<Timestamp>,
    pub unread_count: Option<i32>,
    pub unread_count_display: Option<i32>,
    pub num_mentions: Option<i32>,
    pub num_mentions_display: Option<i32>,
    pub mention_count: Option<i32>,
    pub event_ts: Option<Timestamp>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageBotAdd {
    pub bot_id: Option<BotId>,
    pub bot_link: Option<String>,
    #[serde(default)]
    pub text: String,
    pub ts: Option<Timestamp>,
    pub user: Option<UserId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageBotRemove {
    pub bot_id: Option<BotId>,
    pub bot_link: Option<String>,
    #[serde(default)]
    pub text: String,
    pub ts: Option<Timestamp>,
    pub user: Option<UserId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageBotMessage {
    pub bot_id: Option<BotId>,
    pub icons: Option<MessageBotMessageIcons>,
    #[serde(default)]
    pub text: String,
    pub ts: Option<Timestamp>,
    pub username: Option<String>,
    pub channel: Option<ConversationId>,
    pub team: Option<TeamId>,
    #[serde(default)]
    pub reactions: Vec<Reaction>,
    pub attachments: Option<Vec<MessageStandardAttachment>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageBotMessageIcons {
    pub image_36: Option<String>,
    pub image_48: Option<String>,
    pub image_72: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageChannelArchive {
    pub members: Option<Vec<String>>,
    #[serde(default)]
    pub text: String,
    pub ts: Option<Timestamp>,
    pub user: Option<UserId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageChannelJoin {
    #[serde(default)]
    pub text: String,
    pub ts: Option<Timestamp>,
    pub user: Option<UserId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageChannelLeave {
    #[serde(default)]
    pub text: String,
    pub ts: Option<Timestamp>,
    pub user: Option<UserId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageChannelName {
    pub name: Option<String>,
    pub old_name: Option<String>,
    #[serde(default)]
    pub text: String,
    pub ts: Option<Timestamp>,
    pub user: Option<UserId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageChannelPurpose {
    pub purpose: Option<String>,
    #[serde(default)]
    pub text: String,
    pub ts: Option<Timestamp>,
    pub user: Option<UserId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageChannelTopic {
    #[serde(default)]
    pub text: String,
    pub topic: Option<String>,
    pub ts: Option<Timestamp>,
    pub user: Option<UserId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageChannelUnarchive {
    #[serde(default)]
    pub text: String,
    pub ts: Option<Timestamp>,
    pub user: Option<UserId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageFileComment {
    pub comment: Option<::FileComment>,
    pub file: Option<::File>,
    #[serde(default)]
    pub text: String,
    pub ts: Option<Timestamp>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageFileMention {
    pub file: Option<::File>,
    #[serde(default)]
    pub text: String,
    pub ts: Option<Timestamp>,
    pub user: Option<UserId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageFileShare {
    pub channel: Option<ConversationId>,
    pub file: Option<::File>,
    #[serde(default)]
    pub text: String,
    pub ts: Option<Timestamp>,
    pub upload: Option<bool>,
    pub user: Option<UserId>,
    #[serde(default)]
    pub reactions: Vec<::Reaction>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageGroupArchive {
    pub members: Option<Vec<String>>,
    #[serde(default)]
    pub text: String,
    pub ts: Option<Timestamp>,
    pub user: Option<UserId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageGroupJoin {
    #[serde(default)]
    pub text: String,
    pub ts: Option<Timestamp>,
    pub user: Option<UserId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageGroupLeave {
    #[serde(default)]
    pub text: String,
    pub ts: Option<Timestamp>,
    pub user: Option<UserId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageGroupName {
    pub name: Option<String>,
    pub old_name: Option<String>,
    #[serde(default)]
    pub text: String,
    pub ts: Option<Timestamp>,
    pub user: Option<UserId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageGroupPurpose {
    pub purpose: Option<String>,
    #[serde(default)]
    pub text: String,
    pub ts: Option<Timestamp>,
    pub user: Option<UserId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageGroupTopic {
    #[serde(default)]
    pub text: String,
    pub topic: Option<String>,
    pub ts: Option<Timestamp>,
    pub user: Option<UserId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageGroupUnarchive {
    #[serde(default)]
    pub text: String,
    pub ts: Option<Timestamp>,
    pub user: Option<UserId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMeMessage {
    pub channel: Option<ConversationId>,
    #[serde(default)]
    pub text: String,
    pub ts: Option<Timestamp>,
    pub user: Option<UserId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageChanged {
    pub channel: ConversationId,
    pub event_ts: Timestamp,
    pub hidden: Option<bool>,
    pub message: Option<MessageMessageChangedMessage>,
    pub previous_message: Option<MessageMessageChangedPreviousMessage>,
    pub ts: Timestamp,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageChangedMessage {
    pub bot_id: Option<BotId>,
    pub edited: Option<MessageMessageChangedMessageEdited>,
    pub last_read: Option<String>,
    pub parent_user_id: Option<UserId>,
    pub replies: Option<Vec<MessageMessageChangedMessageReply>>,
    pub reply_count: Option<i32>,
    pub subscribed: Option<bool>,
    #[serde(default)]
    pub text: String,
    pub thread_ts: Option<Timestamp>,
    pub ts: Timestamp,
    pub unread_count: Option<i32>,
    pub user: Option<UserId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageChangedMessageEdited {
    pub ts: Timestamp,
    pub user: UserId,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageChangedMessageReply {
    pub ts: Timestamp,
    pub user: UserId,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageChangedPreviousMessage {
    pub bot_id: Option<BotId>,
    pub edited: Option<MessageMessageChangedPreviousMessageEdited>,
    pub last_read: Option<Timestamp>,
    pub parent_user_id: Option<UserId>,
    pub replies: Option<Vec<MessageMessageChangedPreviousMessageReply>>,
    pub reply_count: Option<i32>,
    pub subscribed: Option<bool>,
    #[serde(default)]
    pub text: String,
    pub thread_ts: Option<Timestamp>,
    pub ts: Timestamp,
    pub unread_count: Option<i32>,
    pub user: Option<UserId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageChangedPreviousMessageEdited {
    pub ts: Timestamp,
    pub user: UserId,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageChangedPreviousMessageReply {
    pub ts: Timestamp,
    pub user: UserId,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageDeleted {
    pub channel: Option<String>,
    pub deleted_ts: Option<String>,
    pub event_ts: Option<String>,
    pub hidden: Option<bool>,
    pub previous_message: Option<MessageMessageDeletedPreviousMessage>,
    pub ts: Option<Timestamp>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageDeletedPreviousMessage {
    pub bot_id: Option<BotId>,
    pub edited: Option<MessageMessageDeletedPreviousMessageEdited>,
    pub last_read: Option<Timestamp>,
    pub parent_user_id: Option<String>,
    pub replies: Option<Vec<MessageMessageDeletedPreviousMessageReply>>,
    pub reply_count: Option<i32>,
    pub subscribed: Option<bool>,
    #[serde(default)]
    pub text: String,
    pub thread_ts: Option<Timestamp>,
    pub ts: Timestamp,
    pub unread_count: Option<i32>,
    pub user: Option<UserId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageDeletedPreviousMessageEdited {
    pub ts: Timestamp,
    pub user: UserId,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageDeletedPreviousMessageReply {
    pub ts: Timestamp,
    pub user: UserId,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageReplied {
    pub channel: Option<ConversationId>,
    pub event_ts: Timestamp,
    pub hidden: Option<bool>,
    pub message: Option<MessageMessageRepliedMessage>,
    pub thread_ts: Option<Timestamp>,
    pub ts: Timestamp,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageRepliedMessage {
    pub bot_id: Option<BotId>,
    pub edited: Option<MessageMessageRepliedMessageEdited>,
    pub last_read: Option<Timestamp>,
    pub parent_user_id: Option<UserId>,
    pub replies: Option<Vec<MessageMessageRepliedMessageReply>>,
    pub reply_count: Option<i32>,
    pub subscribed: Option<bool>,
    #[serde(default)]
    pub text: String,
    pub thread_ts: Option<Timestamp>,
    pub ts: Timestamp,
    pub unread_count: Option<i32>,
    pub user: Option<UserId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageRepliedMessageEdited {
    pub ts: Timestamp,
    pub user: UserId,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageMessageRepliedMessageReply {
    pub ts: Timestamp,
    pub user: UserId,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessagePinnedItem {
    pub channel: Option<ConversationId>,
    pub item: Option<MessagePinnedItemItem>,
    pub item_type: Option<String>,
    #[serde(default)]
    pub text: String,
    pub ts: Option<Timestamp>,
    pub user: Option<UserId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessagePinnedItemItem {}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageReminderAdd {
    pub message: Option<String>,
    pub ts: Option<Timestamp>,
    pub user: Option<UserId>,
    pub channel: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageReplyBroadcast {
    pub attachments: Option<Vec<MessageReplyBroadcastAttachment>>,
    pub channel: Option<String>,
    pub event_ts: Option<String>,
    pub ts: Option<Timestamp>,
    pub user: Option<UserId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageReplyBroadcastAttachment {
    pub author_icon: Option<String>,
    pub author_link: Option<String>,
    pub author_subname: Option<String>,
    pub channel_id: Option<String>,
    pub channel_name: Option<String>,
    pub fallback: Option<String>,
    pub footer: Option<String>,
    pub from_url: Option<String>,
    pub id: Option<i32>,
    pub mrkdwn_in: Option<Vec<String>>,
    #[serde(default)]
    pub text: String,
    pub ts: Option<Timestamp>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MessageStandard {
    pub attachments: Option<Vec<MessageStandardAttachment>>,
    pub bot_id: Option<BotId>,
    pub channel: Option<ConversationId>,
    pub edited: Option<MessageStandardEdited>,
    pub event_ts: Option<Timestamp>,
    pub reply_broadcast: Option<bool>,
    pub source_team: Option<TeamId>,
    pub team: Option<TeamId>,
    #[serde(default)]
    pub text: String,
    pub thread_ts: Option<Timestamp>,
    pub ts: Option<Timestamp>,
    pub user: Option<UserId>,
    pub client_msg_id: Option<Uuid>,
    #[serde(default)]
    pub reactions: Vec<Reaction>,
    pub parent_user_id: Option<UserId>,
    pub replies: Option<Vec<MessageStandardReply>>,
    pub reply_count: Option<i32>,
    pub last_read: Option<Timestamp>,
    pub subscribed: Option<bool>,
    pub pinned_info: Option<MessagePinnedItem>,
    pub unread_count: Option<i32>,
    pub pinned_to: Option<Vec<String>>,
    pub is_starred: Option<bool>,
    pub display_as_bot: Option<bool>,
    // TODO: These fields should belong to a flattened struct
    pub files: Option<Vec<File>>,
    pub upload: Option<bool>,
    pub upload_reply_to: Option<Uuid>,
    pub x_files: Option<Vec<FileId>>,
    pub user_profile: Option<UserProfile>,
    pub user_team: Option<TeamId>,
    #[serde(rename = "type")]
    ty: Option<String>,
}

// TODO: need to add the fields necessary here
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MessageStandardReply {
    pub ts: Timestamp,
    pub user: UserId,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageStandardAttachment {
    pub author_icon: Option<String>,
    pub author_link: Option<String>,
    pub author_name: Option<String>,
    pub color: Option<String>,
    pub fallback: Option<String>,
    pub fields: Option<Vec<MessageStandardAttachmentField>>,
    pub footer: Option<String>,
    pub footer_icon: Option<String>,
    pub image_url: Option<String>,
    pub pretext: Option<String>,
    #[serde(default)]
    pub text: String,
    pub thumb_url: Option<String>,
    pub title: Option<String>,
    pub title_link: Option<String>,
    pub ts: Option<Timestamp>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageStandardAttachmentField {
    pub short: Option<bool>,
    pub title: Option<String>,
    pub value: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageStandardEdited {
    pub ts: Option<Timestamp>,
    pub user: Option<UserId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageUnpinnedItem {
    pub channel: Option<ConversationId>,
    pub item: Option<MessageUnpinnedItemItem>,
    #[serde(default)]
    pub text: String,
    pub ts: Option<Timestamp>,
    pub user: Option<UserId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageUnpinnedItemItem {}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageSlackbotResponse {
    #[serde(default)]
    pub text: String,
    pub ts: Option<Timestamp>,
    pub user: Option<UserId>,
    pub channel: Option<ConversationId>,
    #[serde(default)]
    pub reactions: Vec<::Reaction>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageThreadBroadcast {
    pub attachments: Option<Vec<MessageThreadBroadcastAttachment>>,
    pub root: Option<MessageStandard>,
    #[serde(default)]
    pub text: String,
    pub thread_ts: Option<String>,
    pub user: Option<UserId>,
    pub ts: Option<Timestamp>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct MessageThreadBroadcastAttachment {
    pub fallback: Option<String>,
    pub from_url: Option<String>,
    pub id: Option<i32>,
    pub service_icon: Option<String>,
    pub service_name: Option<String>,
    #[serde(default)]
    pub text: String,
    pub title: Option<String>,
    pub title_link: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Mpim {
    pub created: Option<i32>,
    pub creator: Option<String>,
    pub id: Option<String>,
    pub is_group: Option<bool>,
    pub is_mpim: Option<bool>,
    pub last_read: Option<Timestamp>,
    pub latest: Option<::Message>,
    pub members: Option<Vec<UserId>>,
    pub name: Option<String>,
    pub unread_count: Option<i32>,
    pub unread_count_display: Option<i32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Paging {
    pub count: Option<i32>,
    pub page: Option<i32>,
    pub pages: Option<i32>,
    pub total: Option<i32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Reaction {
    pub count: Option<i32>,
    pub name: String,
    pub users: Option<Vec<UserId>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Reminder {
    pub complete_ts: Option<f32>,
    pub creator: Option<String>,
    pub id: Option<String>,
    pub recurring: Option<bool>,
    #[serde(default)]
    pub text: String,
    pub time: Option<f32>,
    pub user: Option<UserId>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Team {
    pub domain: Option<String>,
    pub email_domain: Option<String>,
    pub icon: Option<TeamIcon>,
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TeamIcon {
    pub image_102: Option<String>,
    pub image_132: Option<String>,
    pub image_34: Option<String>,
    pub image_44: Option<String>,
    pub image_68: Option<String>,
    pub image_88: Option<String>,
    pub image_default: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ThreadInfo {
    pub complete: Option<bool>,
    pub count: Option<i32>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct User {
    pub color: Option<String>,
    pub deleted: Option<bool>,
    pub has_2fa: Option<bool>,
    pub id: UserId,
    pub is_admin: Option<bool>,
    pub is_app_user: Option<bool>,
    pub is_bot: Option<bool>,
    pub is_owner: Option<bool>,
    pub is_primary_owner: Option<bool>,
    pub is_restricted: Option<bool>,
    pub is_ultra_restricted: Option<bool>,
    pub locale: Option<String>,
    pub name: String,
    pub profile: Option<::UserProfile>,
    pub real_name: Option<String>,
    pub team_id: Option<String>,
    pub two_factor_type: Option<String>,
    pub tz: Option<String>,
    pub tz_label: Option<String>,
    pub tz_offset: Option<f32>,
    pub updated: Option<f32>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Usergroup {
    pub auto_type: Option<String>,
    pub created_by: Option<String>,
    pub date_create: Option<i32>,
    pub date_delete: Option<i32>,
    pub date_update: Option<i32>,
    pub deleted_by: Option<String>,
    pub description: Option<String>,
    pub handle: Option<String>,
    pub id: Option<String>,
    pub is_external: Option<bool>,
    pub is_usergroup: Option<bool>,
    pub name: Option<String>,
    pub prefs: Option<UsergroupPrefs>,
    pub team_id: Option<String>,
    pub updated_by: Option<String>,
    pub user_count: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UsergroupPrefs {
    pub channels: Option<Vec<String>>,
    pub groups: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserProfile {
    pub avatar_hash: Option<String>,
    pub display_name: Option<String>,
    pub display_name_normalized: Option<String>,
    pub email: Option<String>,
    #[serde(deserialize_with = "optional_struct_or_empty_array")]
    #[serde(default)]
    pub fields: Option<HashMap<String, UserProfileFields>>,
    pub first_name: Option<String>,
    pub guest_channels: Option<String>,
    pub image_192: Option<String>,
    pub image_24: Option<String>,
    pub image_32: Option<String>,
    pub image_48: Option<String>,
    pub image_512: Option<String>,
    pub image_72: Option<String>,
    pub image_original: Option<String>,
    pub last_name: Option<String>,
    pub phone: Option<String>,
    pub real_name: Option<String>,
    pub real_name_normalized: Option<String>,
    pub skype: Option<String>,
    pub status_emoji: Option<String>,
    pub status_text: Option<String>,
    pub team: Option<String>,
    pub title: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct UserProfileFields {
    pub alt: Option<String>,
    pub label: Option<String>,
    pub value: Option<String>,
}

fn optional_struct_or_empty_array<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
where
    T: ::serde::Deserialize<'de> + Default,
    D: ::serde::Deserializer<'de>,
{
    use serde::de;
    use std::marker::PhantomData;

    struct StructOrEmptyArray<T>(PhantomData<T>);

    impl<'de, T> de::Visitor<'de> for StructOrEmptyArray<T>
    where
        T: de::Deserialize<'de> + Default,
    {
        type Value = Option<T>;

        fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            formatter.write_str("struct or empty array")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Option<T>, A::Error>
        where
            A: de::SeqAccess<'de>,
        {
            match seq.next_element::<T>()? {
                Some(_) => Err(de::Error::custom("non-empty array is not valid")),
                None => Ok(Some(T::default())),
            }
        }

        fn visit_unit<E>(self) -> Result<Option<T>, E>
        where
            E: de::Error,
        {
            Ok(None)
        }

        fn visit_map<M>(self, access: M) -> Result<Option<T>, M::Error>
        where
            M: de::MapAccess<'de>,
        {
            T::deserialize(de::value::MapAccessDeserializer::new(access)).map(Some)
        }
    }

    deserializer.deserialize_any(StructOrEmptyArray(PhantomData))
}

#[cfg(test)]
mod tests {
    use super::UserProfile;
    use serde_json;

    #[test]
    fn test_user_profile_fields_empty_array_deserialize() {
        let user_profile: UserProfile = serde_json::from_str(r#"{"fields": []}"#).unwrap();
        assert_eq!(0, user_profile.fields.unwrap().len());
    }

    #[test]
    fn test_user_profile_fields_empty_map_deserialize() {
        let user_profile: UserProfile = serde_json::from_str(r#"{"fields": {}}"#).unwrap();
        assert_eq!(0, user_profile.fields.unwrap().len());
    }

    #[test]
    fn test_user_profile_fields_nonempty_map_deserialize() {
        let user_profile: UserProfile =
            serde_json::from_str(r#"{"fields": {"some_field": {"alt": "foo", "label": "bar"}}}"#).unwrap();
        assert_eq!(1, user_profile.fields.unwrap().len());
    }

    #[test]
    fn test_user_profile_fields_null_deserialize() {
        let user_profile: UserProfile = serde_json::from_str(r#"{"fields": null}"#).unwrap();
        assert!(user_profile.fields.is_none());
    }

    #[test]
    fn test_user_profile_fields_undefined_deserialize() {
        let user_profile: UserProfile = serde_json::from_str(r#"{}"#).unwrap();
        assert!(user_profile.fields.is_none());
    }
}
