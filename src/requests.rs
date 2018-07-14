//! Functionality for sending requests to Slack.
pub use reqwest::Client;

/// Make an API call to Slack. Takes a struct that describes the request params
pub fn send_structured<T: ::serde::Serialize>(
    client: &::reqwest::Client,
    method_url: &str,
    params: &T,
) -> Result<String, ::reqwest::Error> {
    let url_text = method_url.to_string() + &::serde_qs::to_string(params).unwrap();
    let url = ::reqwest::Url::parse(&url_text).unwrap();
    client.get(url).send()?.text()
}

/// Provides a default `reqwest` client to give to the API functions to send requests.
///
/// # Examples
///
/// ```
/// # let token = "some_token";
/// let client = slack_api::requests::default_client().unwrap();
/// let response = slack_api::channels::list(&client, &token, &Default::default());
/// ```
pub fn default_client() -> ::reqwest::Client {
    ::reqwest::Client::new()
}

macro_rules! slack {
    {
        $(#[$attr:meta])*
        $name:ident,
        $strname:expr,
        $reqname:ident {
            $($(#[$req_item_attr:meta])* $reqattr:ident: $reqtyp:ty),*,
        },
        $resname:ident {
            $($resattr:ident: $restyp:ty),*,
        },
    } => {
        $(#[$attr])*
        pub fn $name(client: &::requests::Client, token: &str, request: &$reqname) -> Result<$resname, ::requests::Error> {
            use requests::Error;
            #[derive(Deserialize)]
            struct IsError {
                ok: bool,
                error: Option<String>,
            }

            let url = ::requests::get_slack_url_for_method($strname) + "?token=" + token + "&";
            let bytes = ::requests::send_structured(client, &url, &request).map_err(Error::Client)?;

            let is_error = ::serde_json::from_str::<IsError>(&bytes);
            match is_error {
                // Complete failure, can't do anything with the bytes
                Err(e) => Err(Error::CannotParse(e, bytes)),
                // Slack sent us an error
                Ok(IsError { ok: false, error }) => Err(Error::Slack(error.unwrap_or_default())),
                // Slack sent us an success result
                Ok(IsError { ok: true, .. }) => match ::serde_json::from_str::<$resname>(&bytes) {
                    Ok(res) => Ok(res),
                    Err(e) => Err(Error::CannotParse(e, bytes)),
                },
            }
        }

        #[derive(Clone, Default, Debug, Serialize)]
        pub struct $reqname<'a> {
            $($(#[$req_item_attr])* pub $reqattr: $reqtyp,)*
        }

        #[derive(Clone, Debug, Deserialize)]
        #[serde(deny_unknown_fields)]
        pub struct $resname {
            ok: bool,
            $(pub $resattr: $restyp,)*
        }
    };
}

macro_rules! api_call {
    ($name:ident, $strname:expr, $reqty:ty, $okty:ty) => {
        pub fn $name(client: &::requests::Client, token: &str, request: &$reqty) -> Result<$okty, ::requests::Error> {
            api_call_internal!(client, token, $strname, request, $okty)
        }
    };
    ($name:ident, $strname:expr,() => $okty:ty) => {
        pub fn $name(client: &::requests::Client, token: &str) -> Result<$okty, ::requests::Error> {
            api_call_internal!(client, token, $strname, (), $okty)
        }
    };
    ($name:ident, $strname:expr, $reqty:ty => ()) => {
        pub fn $name(client: &::requests::Client, token: &str, request: &$reqty) -> Result<(), ::requests::Error> {
            #[allow(dead_code)] // But isn't serde using the field?
            #[derive(Deserialize)]
            #[serde(deny_unknown_fields)]
            struct SimpleOk {
                ok: bool,
            }

            api_call_internal!(client, token, $strname, request, SimpleOk).map(|_| ())
        }
    };
    ($name:ident, $strname:expr) => {
        pub fn $name(client: &::requests::Client, token: &str) -> Result<(), ::requests::Error> {
            #[allow(dead_code)] // But isn't serde using the field?
            #[derive(Deserialize)]
            #[serde(deny_unknown_fields)]
            struct SimpleOk {
                ok: bool,
            }

            api_call_internal!(client, token, $strname, (), SimpleOk).map(|_| ())
        }
    };
}

macro_rules! api_call_internal {
    ($client:expr, $token:expr, $strname:expr, $request:expr, $okty:ty) => {{
        use requests::Error;
        #[derive(Deserialize)]
        struct IsError {
            ok: bool,
            error: Option<String>,
        }

        let url = ::requests::get_slack_url_for_method($strname) + "?token=" + $token + "&";
        let bytes = ::requests::send_structured($client, &url, &$request).map_err(Error::Client)?;

        let is_error = ::serde_json::from_str::<IsError>(&bytes);
        match is_error {
            // Complete failure, can't do anything with the bytes
            Err(e) => Err(Error::CannotParse(e, bytes)),
            // Slack sent us an error
            Ok(IsError { ok: false, error }) => Err(Error::Slack(error.unwrap_or_default())),
            // Slack sent us an success result
            Ok(IsError { ok: true, .. }) => match ::serde_json::from_str::<$okty>(&bytes) {
                Ok(r) => Ok(r),
                Err(e) => Err(Error::CannotParse(e, bytes)),
            },
        }
    }};
}

pub fn get_slack_url_for_method(method: &str) -> String {
    format!("https://slack.com/api/{}", method)
}

pub enum Error {
    Slack(String),
    CannotParse(::serde_json::error::Error, String),
    Client(::reqwest::Error),
}

impl ::std::fmt::Debug for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Error::Slack(reason) => write!(f, "{}", reason),
            Error::CannotParse(e, json) => write!(f, "{}\n{}", e, json),
            Error::Client(..) => write!(f, "The requests client failed"),
        }
    }
}

impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Error::Slack(reason) => write!(f, "{}", reason),
            Error::CannotParse(e, json) => write!(f, "{}\n{}", e, json),
            Error::Client(..) => write!(f, "The requests client failed"),
        }
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match self {
            Error::Slack(ref reason) => reason,
            Error::CannotParse(..) => "Could not parse as specified result type",
            Error::Client(..) => "The requests client failed",
        }
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        match self {
            Error::Slack(_) => None,
            Error::CannotParse(ref cause, _) => Some(cause),
            Error::Client(ref cause) => Some(cause),
        }
    }
}
