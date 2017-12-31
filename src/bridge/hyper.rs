//! Support for the `hyper` crate.

use futures::{Future, Stream, future};
use hyper::client::{Client, HttpConnector};
use hyper::{Body, Uri};
use hyper_tls::HttpsConnector;
use serde_json;
use std::str::FromStr;
use ::model::{Definition, Response};
use ::Error;

/// Trait implemented on Hyper's client for interaction with the UrbanDictionary
/// API.
pub trait UrbanDictionaryRequester {
    /// Attempt to retrieve the first `Definition` for a word.
    fn define<'a, S: Into<String>>(&'a self, word: S)
        -> Box<Future<Item = Option<Definition>, Error = Error> + 'a>;

    /// Attempt to retrieve the definitions of a word.
    fn definitions<'a, S: Into<String>>(&'a self, word: S)
        -> Box<Future<Item = Response, Error = Error> + 'a>;
}

impl UrbanDictionaryRequester for Client<HttpsConnector<HttpConnector>, Body> {
    /// Attempt to retrieve the first `Definition` for a word.
    #[inline]
    fn define<'a, S: Into<String>>(&'a self, word: S)
        -> Box<Future<Item = Option<Definition>, Error = Error> + 'a> {
        define(self, word)
    }

    /// Attempt to retrieve the definitions of a word.
    #[inline]
    fn definitions<'a, S: Into<String>>(&'a self, word: S)
        -> Box<Future<Item = Response, Error = Error> + 'a> {
        definitions(self, word)
    }
}

/// Attempt to retrieve the first `Definition` for a word.
pub fn define<'a, S: Into<String>>(
    client: &'a Client<HttpsConnector<HttpConnector>, Body>,
    word: S,
) -> Box<Future<Item = Option<Definition>, Error = Error> + 'a> {
    let word = word.into();

    let url = format!(
        "http://api.urbandictionary.com/v0/define?term={}",
        word,
    );
    let uri = match Uri::from_str(&url) {
        Ok(v) => v,
        Err(why) => return Box::new(future::err(Error::Uri(why))),
    };

    Box::new(future::ok(uri)
        .and_then(move |uri| client.get(uri))
        .and_then(|res| res.body().concat2())
        .map_err(From::from)
        .and_then(|body| serde_json::from_slice::<Response>(&body).map_err(From::from))
        .map(|mut resp| if !resp.definitions.is_empty() {
            Some(resp.definitions.remove(0))
        } else {
            None
        }))
}

/// Attempt to retrieve the definitions of a word.
pub fn definitions<'a, S: Into<String>>(
    client: &'a Client<HttpsConnector<HttpConnector>, Body>,
    word: S,
) -> Box<Future<Item = Response, Error = Error> + 'a> {
    let word = word.into();

    let url = format!(
        "http://api.urbandictionary.com/v0/define?term={}",
        word,
    );
    let uri = match Uri::from_str(&url) {
        Ok(v) => v,
        Err(why) => return Box::new(future::err(Error::Uri(why))),
    };

    Box::new(future::ok(uri)
        .and_then(move |uri| client.get(uri))
        .and_then(|res| res.body().concat2())
        .map_err(From::from)
        .and_then(|body| serde_json::from_slice(&body).map_err(From::from)))
}
