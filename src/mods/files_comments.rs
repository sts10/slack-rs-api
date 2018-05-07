use types::*;

/// Add a comment to an existing file.
///
/// Wraps https://api.slack.com/methods/files.comments.add

api_call!(add, "files.comments.add", AddRequest, AddResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct AddRequest<'a> {
    /// File to add a comment to.
    pub file: &'a str,
    /// Text of the comment to add.
    pub comment: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AddResponse {
    ok: bool,
    pub comment: FileComment,
}

/// Deletes an existing comment on a file.
///
/// Wraps https://api.slack.com/methods/files.comments.delete

api_call!(delete, "files.comments.delete", DeleteRequest, DeleteResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct DeleteRequest<'a> {
    /// File to delete a comment from.
    pub file: &'a str,
    /// The comment to delete.
    pub id: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeleteResponse {
    ok: bool,
}

/// Edit an existing file comment.
///
/// Wraps https://api.slack.com/methods/files.comments.edit

api_call!(edit, "files.comments.edit", EditRequest, EditResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct EditRequest<'a> {
    /// File containing the comment to edit.
    pub file: &'a str,
    /// The comment to edit.
    pub id: &'a str,
    /// Text of the comment to edit.
    pub comment: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EditResponse {
    ok: bool,
    pub comment: FileComment,
}
