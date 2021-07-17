use darling::FromMeta;
use itertools::Itertools;
use proc_macro::TokenStream;
use proc_macro_error::{emit_error, emit_warning, proc_macro_error};
use syn::{parse_macro_input, spanned::Spanned, AttributeArgs};

const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// Fetches the issue from Github and emits a warning or error if it is closed depending on the
/// `ISSUE_HARD_FAIL` environment variable.
#[proc_macro_error]
#[proc_macro_attribute]
pub fn track(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);

    let issue = match Issue::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    match issue.is_closed() {
        Err(e) => {
            if std::env::var("ISSUE_HARD_FAIL").is_ok() {
                emit_error!(
                    attr_args[0].span(),
                    "unable to access {}\n  {}",
                    issue.url,
                    e
                )
            } else {
                emit_warning!(
                    attr_args[0].span(),
                    "unable to access {}\n  {}",
                    issue.url,
                    e
                )
            }
        }
        Ok(true) => {
            if std::env::var("ISSUE_HARD_FAIL").is_ok() {
                emit_error!(attr_args[0].span(), "issue {} has been closed", issue.url)
            } else {
                emit_warning!(attr_args[0].span(), "issue {} has been closed", issue.url)
            }
        }
        _ => (),
    }

    input
}

// Both Github and Gitlab use the `closed_at` field to identify closed issues.
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
struct ApiIssue {
    pub closed_at: Option<String>,
}

#[derive(Default, FromMeta)]
#[darling(default)]
struct Issue {
    pub url: String,
}

impl Issue {
    fn canonicalize_url(&self) -> url::Url {
        let url = url::Url::parse(&self.url).unwrap();

        if url.host_str().unwrap().contains("github") {
            let path: String = Itertools::intersperse(url.path_segments().unwrap(), "/").collect();
            return url::Url::parse(&format!("https://api.github.com/repos/{}", path)).unwrap();
        }

        unreachable!("only github public repositories are currently supported")
    }

    pub fn is_closed(&self) -> Result<bool, anyhow::Error> {
        let client = reqwest::blocking::ClientBuilder::new()
            .user_agent(APP_USER_AGENT)
            .build()
            .unwrap();

        let response = client.get(self.canonicalize_url()).send()?;

        if !response.status().is_success() {
            anyhow::bail!(
                "failed to fetch issue: {}",
                response
                    .text()
                    .unwrap_or_else(|e| format!("no response found: {}", e))
            )
        }

        let issue: ApiIssue = response.json()?;

        Ok(issue.closed_at.is_some())
    }
}
