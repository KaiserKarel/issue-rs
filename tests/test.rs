#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let t = trybuild::TestCases::new();

        t.pass("ui/github_active_issue.rs");

        std::env::set_var("ISSUE_HARD_FAIL", "True");
        t.compile_fail("ui/github_closed_issue.rs");
        t.compile_fail("ui/github_issue_not_found.rs");
    }
}

#[issue::track(url = "https://github.com/KaiserKarel/issue/issues/1")]
mod active {}

#[issue::track(url = "https://github.com/KaiserKarel/issue/issues/2")]
mod closed {}
