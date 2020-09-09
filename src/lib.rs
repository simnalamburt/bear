//! A dead-simple test helper for rust.
//!
//! Example
//! --------
//!
//! ```rust
//! use bear::fixture;
//!
//! // Get a content of 'tests/fixtures/hello.txt'
//! let content = fixture("hello.txt");
//!
//! // Do whatever you want with it
//! ```

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/simnalamburt/i/master/bear/cute.png",
    html_favicon_url = "https://raw.githubusercontent.com/simnalamburt/i/master/bear/cute.png",
)]

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

/// Open a file from the 'tests/fixtures' directory, and return its content as a `String`
///
/// Example
/// --------
///
/// ```rust
/// use bear::fixture;
///
/// // Get a content of 'tests/fixtures/hello.txt'
/// let content = fixture("hello.txt");
///
/// /* Do whatever you want with it */
/// ```
pub fn fixture(filename: &str) -> String {
    let path = Path::new("tests").join("fixtures").join(filename);
    let file = File::open(&path).unwrap();
    let mut reader = BufReader::new(file);

    let mut ret = String::new();

    reader.read_to_string(&mut ret).unwrap();

    ret
}
