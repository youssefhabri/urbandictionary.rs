#![cfg(feature = "reqwest-support")]

extern crate reqwest;
extern crate urbandictionary;

use reqwest::blocking::Client;
use urbandictionary::ReqwestUrbanDictionaryRequester;

#[ignore]
#[test]
fn test_define() {
    let resp = Client::new().define("england").unwrap();

    assert!(resp.is_some());
}

#[ignore]
#[test]
fn test_definitions() {
    let resp = Client::new().definitions("england").unwrap();

    assert!(!resp.definitions.is_empty());
}
