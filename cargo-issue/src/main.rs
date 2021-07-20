use futures::stream::StreamExt;
use issue::{GithubIssue, APP_USER_AGENT, CONFIG_ENV};
use serde_json::Deserializer;
use std::process::{Command, Stdio};
use structopt::StructOpt;
use tokio::time::Duration;
use tokio_stream::{self as stream};

#[cfg(test)]
mod tests;

#[derive(StructOpt, Debug)]
#[structopt(bin_name = "cargo")]
pub enum Cli {
    #[structopt(name = "issue")]
    Issue(Cmd),
}

#[derive(StructOpt, Debug, Clone)]
pub enum Cmd {
    Check {
        #[structopt(short, long)]
        package: Option<String>,
        #[structopt(short, long)]
        manifest_path: Option<String>,
    },

    #[structopt(name = "list")]
    List {
        #[structopt(short, long)]
        package: Option<String>,
        #[structopt(short, long)]
        manifest_path: Option<String>,
    },
}

#[tokio::main]
async fn main() {
    let Cli::Issue(opts) = Cli::from_args();

    let (package, manifest_path) = match opts.clone() {
        Cmd::Check {
            package,
            manifest_path,
        } => (package, manifest_path),
        Cmd::List {
            package,
            manifest_path,
        } => (package, manifest_path),
    };

    let cargo = std::env::var("CARGO").expect("cargo not found");

    let mut command = Command::new(cargo);
    command.env(
        CONFIG_ENV,
        serde_json::to_string(&issue::Mode::Pipe).expect("serializing configuration"),
    );
    command.arg("--quiet");
    command.arg("check");

    package
        .as_ref()
        .map(|package| command.args(["-p", package]));

    manifest_path
        .as_ref()
        .map(|manifest_path| command.args(["--manifest-path", manifest_path]));

    command.stdout(Stdio::piped());

    let mut handle = command.spawn().unwrap();
    let stdout = handle.stdout.as_mut().unwrap();
    let stream = Deserializer::from_reader(stdout).into_iter::<issue::Issue>();

    // Ensures that cargo's build output is not interleaved with the actuall issues being printed.
    tokio::time::sleep(Duration::from_millis(100)).await;

    stream::iter(stream)
        .for_each_concurrent(None, |issue| {
            let opts = opts.clone();

            async move {
                let issue = issue.expect("deserializing cargo check stdout");

                match opts {
                    Cmd::List { .. } => println!("{:?}", &issue),
                    Cmd::Check { .. } => {
                        // TODO handle network failure
                        if is_closed(&issue).await.unwrap() {
                            println!("Closed issue: \n\n       - url: {}", &issue.url)
                        }
                    }
                }
            }
        })
        .await;

    handle.wait().unwrap();
}

pub async fn is_closed(issue: &issue::Issue) -> Result<bool, anyhow::Error> {
    let client = reqwest::ClientBuilder::new()
        .user_agent(APP_USER_AGENT)
        .build()
        .unwrap();

    let response = client.get(issue.canonicalize_url()).send().await?;

    if !response.status().is_success() {
        anyhow::bail!(
            "failed to fetch issue: {}",
            response
                .text()
                .await
                .unwrap_or_else(|e| format!("no response found: {}", e))
        )
    }

    let issue: GithubIssue = response.json().await?;

    Ok(issue.closed_at.is_some())
}
