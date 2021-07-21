use darling::FromMeta;
use lazy_static::lazy_static;
use lib::{Level, Mode};
use proc_macro::TokenStream;
use proc_macro_error::{emit_error, emit_warning, proc_macro_error};
use syn::{parse_macro_input, spanned::Spanned, AttributeArgs};

#[cfg(not(debug_assertions))]
lazy_static! {
    static ref MODE: Mode = lib::get_mode().unwrap_or(Mode::Noop);
}

#[cfg(debug_assertions)]
lazy_static! {
    static ref MODE: Mode = lib::get_mode().unwrap_or(Mode::Emit(Level::Warn));
}

/// Fetches the issue from Github and emits a warning or error if it is closed depending on the
/// `ISSUE_HARD_FAIL` environment variable.
#[proc_macro_error]
#[proc_macro_attribute]
pub fn track(args: TokenStream, input: TokenStream) -> TokenStream {
    let attr_args = parse_macro_input!(args as AttributeArgs);

    let issue: lib::Issue = match Issue::from_list(&attr_args) {
        Ok(v) => v.into(),
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    match *MODE {
        Mode::Noop => (),
        Mode::Pipe => println!(
            "{}",
            serde_json::to_string(&issue).expect("serializing issue")
        ),
        Mode::Emit(Level::Warn) => match issue.is_closed() {
            Err(e) => {
                emit_warning!(
                    attr_args[0].span(),
                    "unable to access {}\n  {}",
                    issue.url,
                    e
                )
            }
            Ok(true) => {
                emit_warning!(attr_args[0].span(), "issue {} has been closed", issue.url)
            }
            _ => (),
        },
        Mode::Emit(Level::Error) => match issue.is_closed() {
            Err(e) => {
                emit_error!(
                    attr_args[0].span(),
                    "unable to access {}\n  {}",
                    issue.url,
                    e
                )
            }
            Ok(true) => {
                emit_error!(attr_args[0].span(), "issue {} has been closed", issue.url)
            }
            _ => (),
        },
    }
    input
}

#[derive(Default, Debug, FromMeta, serde::Serialize)]
#[darling(default)]
struct Issue {
    pub url: String,
}

impl From<Issue> for lib::Issue {
    fn from(issue: Issue) -> Self {
        lib::Issue { url: issue.url }
    }
}
