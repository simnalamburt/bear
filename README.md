![][bear-i] bear [![][version-i]][crates] [![][buildstat-i]][travis]
========

Dead-simple test helper for rust. Documentations are [here][docs]

Example
--------

###### Cargo.toml

```toml
[dev-dependencies]
bear = "^0.1.1"
```

###### tests/test.rs

```rust
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

[BSD 2-Clause](LICENSE.md)

[docs]:         //simnalamburt.github.io/bear
[crates]:       //crates.io/crates/bear
[travis]:       //travis-ci.org/simnalamburt/bear

[bear-i]:       https://simnalamburt.github.io/bear/cute.png
[version-i]:    https://img.shields.io/badge/cargo-v0.1.1-orange.svg?style=flat
[buildstat-i]:  https://img.shields.io/travis/simnalamburt/bear/master.svg?style=flat
