/// Creates a reminder.
///
/// Wraps https://api.slack.com/methods/reminders.add

api_call!(add, "reminders.add", AddRequest, AddResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct AddRequest<'a> {
    /// The content of the reminder
    pub text: &'a str,
    /// When this reminder should happen: the Unix timestamp (up to five years from now), the number of seconds until the reminder (if within 24 hours), or a natural language description (Ex. "in 15 minutes," or "every Thursday")
    pub time: u32,
    /// The user who will receive the reminder. If no user is specified, the reminder will go to user who created it.
    pub user: Option<&'a str>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AddResponse {
    ok: bool,
    pub reminder: Option<::Reminder>,
}

/// Marks a reminder as complete.
///
/// Wraps https://api.slack.com/methods/reminders.complete

api_call!(complete, "reminders.complete", CompleteRequest, CompleteResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct CompleteRequest<'a> {
    /// The ID of the reminder to be marked as complete
    pub reminder: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CompleteResponse {
    ok: bool,
}

/// Deletes a reminder.
///
/// Wraps https://api.slack.com/methods/reminders.delete

api_call!(delete, "reminders.delete", DeleteRequest, DeleteResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct DeleteRequest<'a> {
    /// The ID of the reminder
    pub reminder: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeleteResponse {
    ok: bool,
}

/// Gets information about a reminder.
///
/// Wraps https://api.slack.com/methods/reminders.info

api_call!(info, "reminders.info", InfoRequest, InfoResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct InfoRequest<'a> {
    /// The ID of the reminder
    pub reminder: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InfoResponse {
    ok: bool,
    pub reminder: Option<::Reminder>,
}

/// Lists all reminders created by or for a given user.
///
/// Wraps https://api.slack.com/methods/reminders.list
// TOOD: This seems like it should have a Request struct
api_call!(list, "reminders.list", ListResponse);

#[derive(Clone, Debug, Deserialize)]
pub struct ListResponse {
    ok: bool,
    pub reminders: Option<Vec<::Reminder>>,
}

