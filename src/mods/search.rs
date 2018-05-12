//! Search your team's files and messages.

/// Searches for messages and files matching a query.
///
/// Wraps https://api.slack.com/methods/search.all

api_call!(all, "search.all", AllRequest, AllResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct AllRequest<'a> {
    /// Search query. May contains booleans, etc.
    pub query: &'a str,
    /// Return matches sorted by either score or timestamp.
    pub sort: Option<&'a str>,
    /// Change sort direction to ascending (asc) or descending (desc).
    pub sort_dir: Option<&'a str>,
    /// Pass a value of true to enable query highlight markers (see below).
    pub highlight: Option<bool>,
    /// Number of items to return per page.
    pub count: Option<u32>,
    /// Page number of results to return.
    pub page: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AllResponse {
    ok: bool,
    pub files: Option<AllResponseFiles>,
    pub messages: Option<AllResponseMessages>,
    pub query: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AllResponseFiles {
    pub matches: Vec<::File>,
    pub paging: ::Paging,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AllResponseMessages {
    pub matches: Vec<::Message>,
    pub paging: ::Paging,
}

/// Searches for files matching a query.
///
/// Wraps https://api.slack.com/methods/search.files

api_call!(files, "search.files", FilesRequest, FilesResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct FilesRequest<'a> {
    /// Search query. May contain booleans, etc.
    pub query: &'a str,
    /// Return matches sorted by either score or timestamp.
    pub sort: Option<&'a str>,
    /// Change sort direction to ascending (asc) or descending (desc).
    pub sort_dir: Option<&'a str>,
    /// Pass a value of true to enable query highlight markers (see below).
    pub highlight: Option<bool>,
    /// Number of items to return per page.
    pub count: Option<u32>,
    /// Page number of results to return.
    pub page: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FilesResponse {
    ok: bool,
    pub files: Option<FilesResponseFiles>,
    pub query: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FilesResponseFiles {
    pub matches: Option<Vec<::File>>,
    pub paging: Option<::Paging>,
    pub total: Option<i32>,
}

/// Searches for messages matching a query.
///
/// Wraps https://api.slack.com/methods/search.messages

api_call!(messages, "search.messages", MessagesRequest, MessagesResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct MessagesRequest<'a> {
    /// Search query. May contains booleans, etc.
    pub query: &'a str,
    /// Return matches sorted by either score or timestamp.
    pub sort: Option<&'a str>,
    /// Change sort direction to ascending (asc) or descending (desc).
    pub sort_dir: Option<&'a str>,
    /// Pass a value of true to enable query highlight markers (see below).
    pub highlight: Option<bool>,
    /// Number of items to return per page.
    pub count: Option<u32>,
    /// Page number of results to return.
    pub page: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MessagesResponse {
    ok: bool,
    pub messages: Option<MessagesResponseMessages>,
    pub query: Option<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MessagesResponseMessages {
    pub matches: Option<Vec<::Message>>,
    pub paging: Option<::Paging>,
    pub total: Option<i32>,
}
