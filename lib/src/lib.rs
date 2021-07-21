mod config;

pub use config::{Level, Mode};
use itertools::Itertools;
use std::fmt::{Display, Formatter};

pub const CONFIG_ENV: &str = "ISSUE_RS::Config::Mode";
pub const IGNORE_ENV: &str = "ISSUE_RS_IGNORE";

pub fn get_mode() -> Mode {
    std::env::var(CONFIG_ENV)
        .map(|var| serde_json::from_str(&var).expect("reading Mode from configuration"))
        .unwrap_or_else(|_| {
            std::env::var(IGNORE_ENV)
                .ok()
                .map(|_| Mode::Noop)
                .unwrap_or_default()
        })
}

pub const APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Issue {
    pub url: String,
}

impl Display for Issue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "issue: {}", self.url)
    }
}

// Both Github and Gitlab use the `closed_at` field to identify closed issues.
#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GithubIssue {
    pub closed_at: Option<String>,
}

impl Issue {
    pub fn canonicalize_url(&self) -> url::Url {
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

        let issue: GithubIssue = response.json()?;

        Ok(issue.closed_at.is_some())
    }
}
