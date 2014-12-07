//! A dead-simple test helper for rust.

#![unstable]
#![doc(html_root_url = "https://simnalamburt.github.io/bear-rs")]

#![deny(warnings)]
#![deny(missing_docs)]

use std::io::{File, BufferedReader};

/// Open a file from the 'tests/fixtures' directory, and return its content as a `String`
///
/// Example
/// --------
///
/// ```rust
/// // tests/test.rs
///
/// #[test]
/// fn do_some_test() {
///     // Get a content of 'tests/fixtures/hello.obj'
///     let content: String = fixture("hello.obj");
///
///     /* Do whatever you want with it */
/// }
/// ```
pub fn fixture(filename: &str) -> String {
    let path = Path::new("tests").join("fixtures").join(filename);
    let file = File::open(&path).unwrap();
    let mut reader = BufferedReader::new(file);
    let buf = reader.read_to_end().unwrap();

    String::from_utf8(buf).unwrap()
}
