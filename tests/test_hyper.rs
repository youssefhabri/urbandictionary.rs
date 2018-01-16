#![cfg(feature = "hyper-support")]

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate urbandictionary;

use futures::Future;
use hyper::client::HttpConnector;
use hyper::{Body, Client};
use hyper_tls::HttpsConnector;
use tokio_core::reactor::{Core, Handle};
use urbandictionary::HyperUrbanDictionaryRequester;

#[inline]
fn client(handle: &Handle) -> Client<HttpsConnector<HttpConnector>, Body> {
    Client::configure()
        .connector(HttpsConnector::new(4, handle).unwrap())
        .build(handle)
}

#[ignore]
#[test]
fn test_define() {
    let mut core = Core::new().unwrap();
    let client = client(&core.handle());

    let done = client.define("cat").and_then(|resp| {
        assert!(resp.is_some());

        Ok(())
    }).or_else(|_| {
        assert!(false);

        Err(())
    });

    core.run(done).expect("core err");
}

#[ignore]
#[test]
fn test_definitions() {
    let mut core = Core::new().unwrap();
    let client = client(&core.handle());

    let done = client.definitions("cat").and_then(|resp| {
        assert!(resp.definitions.len() > 0);

        Ok(())
    }).or_else(|_| {
        assert!(false);

        Err(())
    });

    core.run(done).expect("core err");
}
