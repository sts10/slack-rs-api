/// Gets information about a bot user.
///
/// Wraps https://api.slack.com/methods/bots.info

api_call!(info, "bots.info", InfoRequest, InfoResponse);
//TODO: This is very silly to call without a bot in the request
// especially because that's the only situation in which we get an ok but no bot field
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_info() {
        let client = ::requests::default_client().unwrap();
        let token = env::var("SLACK_API_TOKEN").unwrap();

        info(&client, &token, &InfoRequest::default()).unwrap();
    }
}