![](https://simnalamburt.github.io/bear-rs/cute.png) bear
========

A dead-simple test helper for rust.

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
