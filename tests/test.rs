#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let t = trybuild::TestCases::new();

        t.pass("ui/github_active_issue.rs");

        std::env::set_var("ISSUE_RS::Config::Mode", r#"{"Emit":"Warn"}"#);
        t.pass("ui/github_closed_issue.rs");
        t.pass("ui/github_issue_not_found.rs");
    }
}

#[cfg(feature = "integration")]
mod integration {
    #[issue::track(url = "https://github.com/KaiserKarel/issue/issues/1")]
    mod active {}

    #[issue::track(url = "https://github.com/KaiserKarel/issue/issues/2")]
    mod closed {}
}


