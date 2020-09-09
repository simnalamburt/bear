![bear-i] bear [![version-i]][crates] [![buildstat-i]][travis]
========

Dead-simple test helper for rust. See **[documentation]** for the further
details.

### Example
```toml
# Cargo.toml
[dev-dependencies]
bear = "0.2"
```
```rust
// tests/test.rs
extern crate bear;

use bear::fixture;

#[test]
fn do_some_test() {
    // Get a content of 'tests/fixtures/hello.txt'
    let content = fixture("hello.txt");

    // Do whatever you want with it
}
```

&nbsp;

--------

[BSD 2-Clause](LICENSE.md)

[documentation]: https://docs.rs/crate/bear/
[crates]: https//crates.io/crates/bear
[travis]: https://travis-ci.com/simnalamburt/bear

[bear-i]: https://raw.githubusercontent.com/simnalamburt/i/master/bear/cute.png
[version-i]: https://badgen.net/crates/v/bear
[buildstat-i]: https://badgen.net/travis/simnalamburt/bear
