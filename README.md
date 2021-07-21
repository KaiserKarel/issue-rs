# Issue-rs

Ever added a todo based on an open issue (perhaps in one of your dependencies)? Track the issue and be warned when it is
closed!


```rust
// Our trait implementation never returns an error, but until the `nevertype`
// is stabilized, we need to use the unit type.
#[issue::track(url="https://github.com/rust-lang/rust/issues/35121")]
type Result<T> = core::result::Result<T, ()>;
```

Once the tracked issue is resolved, a warning will be emitted during compile time. 

# CI and Configuration

Locally it is recommended to always run the tracked issue. Alternatively, setting the environment variable 
`ISSUE_RS_IGNORE` to any value will disable it entirely. 

Using cargo-issue


## TODO

- [ ] Support Gitlab
- [ ] Support arbitrary URLS/private instances
- [ ] Authentication for private repos
