use std::collections::HashMap;

/// Lists custom emoji for a team.
///
/// Wraps https://api.slack.com/methods/emoji.list

api_call!(list, "emoji.list", ListRequest, ListResponse);

#[derive(Clone, Default, Debug, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ListRequest;

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponse {
    ok: bool,
    pub emoji: Option<HashMap<String, String>>,
}
