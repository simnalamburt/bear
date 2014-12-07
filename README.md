![][bear] bear [![Cargo version][cargo-i]][cargo-a] [![Build Status][travis-i]][travis-a]
========

Dead-simple test helper for rust. Documentations are [here][docs]

Example
--------

```rust
// tests/test.rs

extern crate bear;

use bear::fixture;

#[test]
fn do_some_test() {
    // Get a content of 'tests/fixtures/hello.obj'
    let content: String = fixture("hello.obj");

    /* Do whatever you want with it */
}
```

--------

MIT License

[docs]: https://simnalamburt.github.io/bear
[bear]: https://simnalamburt.github.io/bear/cute.png
[cargo-i]: https://img.shields.io/badge/cargo-0.1.1-brightgreen.svg
[cargo-a]: https://crates.io/crates/bear
[travis-i]: https://travis-ci.org/simnalamburt/bear.svg?branch=master
[travis-a]: https://travis-ci.org/simnalamburt/bear
