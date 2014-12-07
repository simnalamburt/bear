![](https://simnalamburt.github.io/bear-rs/cute.png) bear
========

Dead-simple test helper for rust. Documentations are [here](//simnalamburt.github.io/bear-rs)

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
