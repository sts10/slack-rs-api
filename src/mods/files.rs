//! Get info on files uploaded to Slack, upload new files to Slack.

use types::*;

/// Deletes a file.
///
/// Wraps https://api.slack.com/methods/files.delete

api_call!(delete, "files.delete", DeleteRequest => ());

#[derive(Clone, Default, Debug, Serialize)]
pub struct DeleteRequest<'a> {
    /// ID of file to delete.
    pub file: &'a str,
}

/// Gets information about a team file.
///
/// Wraps https://api.slack.com/methods/files.info

api_call!(info, "files.info", InfoRequest, InfoResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct InfoRequest<'a> {
    /// Specify a file by providing its ID.
    pub file: &'a str,
    /// Number of items to return per page.
    pub count: Option<u32>,
    /// Page number of results to return.
    pub page: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InfoResponse {
    ok: bool,
    pub comments: Vec<FileComment>,
    pub file: File,
    pub paging: Paging,
}

/// Lists & filters team files.
///
/// Wraps https://api.slack.com/methods/files.list

api_call!(list, "files.list", ListRequest, ListResponse);

#[derive(Clone, Default, Debug, Serialize)]
pub struct ListRequest<'a> {
    /// Filter files created by a single user.
    pub user: Option<&'a str>,
    /// Filter files appearing in a specific channel, indicated by its ID.
    pub channel: Option<&'a str>,
    /// Filter files created after this timestamp (inclusive).
    pub ts_from: Option<u32>,
    /// Filter files created before this timestamp (inclusive).
    pub ts_to: Option<u32>,
    /// Filter files by type:
    ///
    ///
    /// all - All files
    /// spaces - Posts
    /// snippets - Snippets
    /// images - Image files
    /// gdocs - Google docs
    /// zips - Zip files
    /// pdfs - PDF files
    ///
    ///
    /// You can pass multiple values in the types argument, like types=spaces,snippets.The default value is all, which does not filter the list.
    pub types: Option<&'a str>,
    /// Number of items to return per page.
    pub count: Option<u32>,
    /// Page number of results to return.
    pub page: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ListResponse {
    ok: bool,
    pub files: Option<Vec<::File>>,
    pub paging: Option<::Paging>,
}

/// Revokes public/external sharing access for a file
///
/// Wraps https://api.slack.com/methods/files.revokePublicURL

api_call!(
    revoke_public_url,
    "files.revokePublicURL",
    RevokePublicURLRequest,
    RevokePublicURLResponse
);

#[derive(Clone, Default, Debug, Serialize)]
pub struct RevokePublicURLRequest<'a> {
    /// File to revoke
    pub file: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RevokePublicURLResponse {
    ok: bool,
    pub file: File,
}

/// Enables a file for public/external sharing.
///
/// Wraps https://api.slack.com/methods/files.sharedPublicURL

api_call!(
    shared_public_url,
    "files.sharedPublicURL",
    SharedPublicURLRequest,
    SharedPublicURLResponse
);

#[derive(Clone, Default, Debug, Serialize)]
pub struct SharedPublicURLRequest<'a> {
    /// File to share
    pub file: &'a str,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SharedPublicURLResponse {
    ok: bool,
    pub file: File,
}
