extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate serde_qs;

#[macro_use]
extern crate requests;

/// Gets information about a bot user.
///
/// Wraps https://api.slack.com/methods/bots.info

api_call!(info, "bots.info", InfoRequest, InfoResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct InfoRequest<'a> {
    /// Bot user to get info on
    pub bot: Option<&'a str>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InfoResponse {
    ok: bool,
    pub bot: InfoResponseBot,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InfoResponseBot {
    pub app_id: String,
    pub deleted: bool,
    pub icons: InfoResponseBotIcons,
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InfoResponseBotIcons {
    pub image_36: Option<String>,
    pub image_48: Option<String>,
    pub image_72: Option<String>,
}
