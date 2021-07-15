# Issue-rs

Ever added a todo based on an open issue (perhaps in one of your dependencies)? Track the issue and be warned once it is
closed?

```rust
fn main() {
    
    // do_something never returns an error in our case, but until the nevertype
    // is stabilized, we need to check it anyways.
    #[issue::track(url="https://github.com/rust-lang/rust/issues/35121")]
    let a = do_something().unwrap();
}
```

Once the tracked issue is resolved, a warning will be emitted during compile time. 


## TODO

- [ ] Support Gitlab
- [ ] Support arbitrary URLS/private instances
- [ ] Authentication for private repos
