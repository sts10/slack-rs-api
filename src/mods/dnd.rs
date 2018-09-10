//! Adjust and view Do Not Disturb settings for team members.

use std::collections::HashMap;

/// Ends the current user's Do Not Disturb session immediately.
///
/// Wraps https://api.slack.com/methods/dnd.endDnd

api_call!(end_dnd, "dnd.endDnd");

/// Ends the current user's snooze mode immediately.
///
/// Wraps https://api.slack.com/methods/dnd.endSnooze

api_call!(end_snooze, "dnd.endSnooze", () => EndSnoozeResponse);

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EndSnoozeResponse {
    ok: bool,
    pub dnd_enabled: Option<bool>,
    pub next_dnd_end_ts: Option<f32>,
    pub next_dnd_start_ts: Option<f32>,
    pub snooze_enabled: Option<bool>,
}

/// Retrieves a user's current Do Not Disturb status.
///
/// Wraps https://api.slack.com/methods/dnd.info

api_call!(info, "dnd.info", InfoRequest, InfoResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct InfoRequest {
    /// User to fetch status for (defaults to current user)
    pub user: Option<::UserId>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InfoResponse {
    ok: bool,
    pub dnd_enabled: Option<bool>,
    pub next_dnd_end_ts: Option<f32>,
    pub next_dnd_start_ts: Option<f32>,
    pub snooze_enabled: Option<bool>,
    pub snooze_endtime: Option<f32>,
    pub snooze_remaining: Option<f32>,
}

/// Turns on Do Not Disturb mode for the current user, or changes its duration.
///
/// Wraps https://api.slack.com/methods/dnd.setSnooze

api_call!(set_snooze, "dnd.setSnooze", SetSnoozeRequest, SetSnoozeResponse);

#[derive(Clone, Debug, Serialize, new)]
pub struct SetSnoozeRequest {
    /// Number of minutes, from now, to snooze until.
    pub num_minutes: u32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SetSnoozeResponse {
    ok: bool,
    pub snooze_enabled: Option<bool>,
    pub snooze_endtime: Option<f32>,
    pub snooze_remaining: Option<f32>,
}

/// Retrieves the Do Not Disturb status for users on a team.
///
/// Wraps https://api.slack.com/methods/dnd.teamInfo

api_call!(team_info, "dnd.teamInfo", TeamInfoRequest, TeamInfoResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct TeamInfoRequest<'a> {
    /// Comma-separated list of users to fetch Do Not Disturb status for
    pub users: Option<&'a str>, // TODO: This should be a serialize_with on a vec
}

// TODO: idk what's going on here or how it ever worked
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TeamInfoResponse {
    ok: bool,
    pub users: Option<HashMap<String, bool>>,
}
