/// Starts a Real Time Messaging session.
///
/// Wraps https://api.slack.com/methods/rtm.connect

api_call!(connect, "rtm.connect", ConnectRequest, ConnectResponse);

#[derive(Clone, Debug, Default, Serialize)]
pub struct ConnectRequest {
    batch_presence_aware: Option<bool>,
    presence_sub: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConnectResponse {
    ok: bool,
    #[serde(rename = "self")]
    pub slf: ConnectResponseSelf,
    pub team: ConnectResponseTeam,
    pub url: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConnectResponseSelf {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConnectResponseTeam {
    pub domain: Option<String>,
    pub enterprise_id: Option<String>,
    pub enterprise_name: Option<String>,
    pub id: Option<String>,
    pub name: String,
}

/// Starts a Real Time Messaging session.
///
/// Wraps https://api.slack.com/methods/rtm.start

api_call!(start, "rtm.start", StartRequest, StartResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct StartRequest {
    /// Skip unread counts for each channel (improves performance).
    pub no_unreads: Option<bool>,
    /// Returns MPIMs to the client in the API response.
    pub mpim_aware: Option<bool>,
    /// Exclude latest timestamps for channels, groups, mpims, and ims. Automatically sets no_unreads to 1
    pub no_latest: Option<bool>,
    /// Only deliver presence events when requested by subscription. See [presence subscriptions](/docs/presence-and-status#subscriptions).
    pub batch_presence_aware: Option<bool>,
    /// Set this to `true` to receive the locale for users and channels. Defaults to `false`
    pub include_locale: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StartResponse {
    ok: bool,
    pub bots: Option<Vec<::Bot>>,
    pub channels: Option<Vec<::Channel>>,
    pub groups: Option<Vec<::Group>>,
    pub ims: Option<Vec<::Im>>,
    pub mpims: Option<Vec<::Mpim>>,
    #[serde(rename = "self")]
    pub slf: Option<::User>,
    pub team: Option<::Team>,
    pub url: Option<String>,
    pub users: Option<Vec<::User>>,
}
