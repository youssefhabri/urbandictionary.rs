extern crate urbandictionary;

use urbandictionary::UrbanClient;

#[test]
fn test_define() {
    let client = UrbanClient::new();
    assert!(client.define("cat").unwrap().is_some());
}

#[test]
fn test_definitions() {
    let client = UrbanClient::new();
    assert!(client.definitions("cat").unwrap().definitions.len() > 0);
}
