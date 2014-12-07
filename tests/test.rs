extern crate bear;

use bear::fixture;

#[test]
fn test_fixture() {
    assert_eq!(fixture("helloworld.txt"), "Hello, world!\n".to_string());
}
