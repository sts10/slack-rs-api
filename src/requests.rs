//! Functionality for sending requests to Slack.
pub use reqwest::Client;

pub trait SlackSender {
    type Error: ::std::error::Error;

    fn send_structured<T: ::serde::Serialize>(&self, method_url: &str, params: &T) -> Result<String, Self::Error>;
}

impl<C> SlackSender for C
where
    C: ::std::ops::Deref<Target = ::reqwest::Client>,
{
    type Error = ::reqwest::Error;
    /// Make an API call to Slack. Takes a struct that describes the request params
    fn send_structured<T: ::serde::Serialize>(&self, method_url: &str, params: &T) -> Result<String, ::reqwest::Error> {
        let mut url_text = method_url.to_string();
        if let Ok(s) = ::serde_urlencoded::to_string(params) {
            url_text += &s;
        } else {
            // TODO: Log the error
        }
        let url = ::reqwest::Url::parse(&url_text).expect("Internal error, failed to parse Slack URL");
        self.get(url).send()?.text()
    }
}

macro_rules! api_call {
    ($name:ident, $strname:expr, $reqty:ty => $okty:ty) => {
        pub fn $name<C: ::requests::SlackSender>(
            client: &C,
            token: &str,
            request: &$reqty,
        ) -> Result<$okty, ::requests::Error<C>> {
            api_call_internal!(client, token, $strname, request, $okty)
        }
    };
    ($name:ident, $strname:expr, => $okty:ty) => {
        pub fn $name<C: ::requests::SlackSender>(client: &C, token: &str) -> Result<$okty, ::requests::Error<C>> {
            api_call_internal!(client, token, $strname, &"", $okty)
        }
    };
    ($name:ident, $strname:expr, $reqty:ty => ) => {
        pub fn $name<C: ::requests::SlackSender>(
            client: &C,
            token: &str,
            request: &$reqty,
        ) -> Result<(), ::requests::Error<C>> {
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
        pub fn $name<C: ::requests::SlackSender>(client: &C, token: &str) -> Result<(), ::requests::Error<C>> {
            #[allow(dead_code)] // But isn't serde using the field?
            #[derive(Deserialize)]
            #[serde(deny_unknown_fields)]
            struct SimpleOk {
                ok: bool,
            }

            api_call_internal!(client, token, $strname, &"", SimpleOk).map(|_| ())
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
        let bytes = $client.send_structured(&url, $request).map_err(Error::Client)?;

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

pub enum Error<C: SlackSender> {
    Slack(String),
    CannotParse(::serde_json::error::Error, String),
    Client(C::Error),
}

impl<C: SlackSender> ::std::fmt::Debug for Error<C> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Error::Slack(reason) => write!(f, "{}", reason),
            Error::CannotParse(e, json) => write!(f, "{}\n{}", e, json),
            Error::Client(..) => write!(f, "The requests client failed"),
        }
    }
}

impl<C: SlackSender> ::std::fmt::Display for Error<C> {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        match self {
            Error::Slack(reason) => write!(f, "{}", reason),
            Error::CannotParse(e, json) => write!(f, "{}\n{}", e, json),
            Error::Client(..) => write!(f, "The requests client failed"),
        }
    }
}

impl<C: SlackSender> ::std::error::Error for Error<C> {
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
