#![cfg(feature = "hyper-support")]

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate urbandictionary;

use hyper::client::HttpConnector;
use hyper::{Body, Client};
use urbandictionary::HyperUrbanDictionaryRequester;

#[inline]
fn client() -> Client<HttpConnector, Body> {
    Client::builder().build(HttpConnector::new())
}

#[ignore]
#[tokio::test]
async fn test_define() {
    let client = client();
    let resp = client.define("cat").await.unwrap();

    assert!(resp.is_some());
}

#[ignore]
#[tokio::test]
async fn test_definitions() {
    let client = client();
    let resp = client.definitions("cat").await.unwrap();

    assert!(!resp.definitions.is_empty());
}
