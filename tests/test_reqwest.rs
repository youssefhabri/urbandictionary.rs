#![cfg(feature = "reqwest-support")]

extern crate reqwest;
extern crate urbandictionary;

use reqwest::Client;
use urbandictionary::ReqwestUrbanDictionaryRequester;

#[ignore]
#[tokio::test]
async fn test_define() {
    let resp = Client::new().define("england").await.unwrap();

    assert!(resp.is_some());
}

#[ignore]
#[tokio::test]
async fn test_definitions() {
    let resp = Client::new().definitions("england").await.unwrap();

    assert!(!resp.definitions.is_empty());
}
