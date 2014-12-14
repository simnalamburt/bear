//! A dead-simple test helper for rust.
//!
//! Example
//! --------
//!
//! ```rust
//! use bear::fixture;
//!
//! // Get a content of 'tests/fixtures/hello.txt'
//! let content: String = fixture("hello.txt");
//!
//! /* Do whatever you want with it */
//! ```

#![unstable]
#![doc(html_logo_url    = "https://simnalamburt.github.io/bear/cute.png",
       html_favicon_url = "https://simnalamburt.github.io/bear/cute.png",
       html_root_url    = "https://simnalamburt.github.io/bear")]

#![deny(warnings)]
#![deny(missing_docs)]

use std::io::{File, BufferedReader};

/// Open a file from the 'tests/fixtures' directory, and return its content as a `String`
///
/// Example
/// --------
///
/// ```rust
/// use bear::fixture;
///
/// // Get a content of 'tests/fixtures/hello.txt'
/// let content: String = fixture("hello.txt");
///
/// /* Do whatever you want with it */
/// ```
pub fn fixture(filename: &str) -> String {
    let path = Path::new("tests").join("fixtures").join(filename);
    let file = File::open(&path).unwrap();
    let mut reader = BufferedReader::new(file);
    let buf = reader.read_to_end().unwrap();

    String::from_utf8(buf).unwrap()
}
