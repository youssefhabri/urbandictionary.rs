//! Support for the `hyper` crate.

use futures::{Future, Stream, future};
use hyper::client::{Client, Connect};
use hyper::{Error as HyperError, Uri};
use serde_json;
use std::str::FromStr;
use ::model::{Definition, Response};
use ::Error;

/// Trait implemented on Hyper's client for interaction with the UrbanDictionary
/// API.
pub trait UrbanDictionaryRequester {
    /// Attempt to retrieve the first `Definition` for a word.
    fn define<T: AsRef<str>>(&self, word: T)
        -> Box<Future<Item = Option<Definition>, Error = Error>>;

    /// Attempt to retrieve the definitions of a word.
    fn definitions<T: AsRef<str>>(&self, word: T)
        -> Box<Future<Item = Response, Error = Error>>;
}

impl<B, C: Connect> UrbanDictionaryRequester for Client<C, B>
    where B: Stream<Error = HyperError> + 'static, B::Item: AsRef<[u8]> {
    /// Attempt to retrieve the first `Definition` for a word.
    fn define<T: AsRef<str>>(&self, word: T)
        -> Box<Future<Item = Option<Definition>, Error = Error>> {
        Box::new(define(self, word))
    }

    /// Attempt to retrieve the definitions of a word.
    #[inline]
    fn definitions<T: AsRef<str>>(&self, word: T)
        -> Box<Future<Item = Response, Error = Error>> {
        Box::new(definitions(self, word))
    }
}

/// Attempt to retrieve the first `Definition` for a word.
pub fn define<B, C, T> (
    client: &Client<C, B>,
    word: T,
) -> Box<Future<Item = Option<Definition>, Error = Error>>
    where C: Connect,
          B: Stream<Error = HyperError> + 'static,
          B::Item: AsRef<[u8]>,
          T: AsRef<str> {
    let url = format!(
        "http://api.urbandictionary.com/v0/define?term={}",
        word.as_ref(),
    );
    let uri = match Uri::from_str(&url) {
        Ok(v) => v,
        Err(why) => return Box::new(future::err(Error::Uri(why))),
    };

    Box::new(client.get(uri)
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
pub fn definitions<B, C, T>(
    client: &Client<C, B>,
    word: T,
) -> Box<Future<Item = Response, Error = Error>>
    where C: Connect,
          B: Stream<Error = HyperError> + 'static,
          B::Item: AsRef<[u8]>,
          T: AsRef<str> {
    let url = format!(
        "http://api.urbandictionary.com/v0/define?term={}",
        word.as_ref(),
    );
    let uri = match Uri::from_str(&url) {
        Ok(v) => v,
        Err(why) => return Box::new(future::err(Error::Uri(why))),
    };

    Box::new(client.get(uri)
        .and_then(|res| res.body().concat2())
        .map_err(From::from)
        .and_then(|body| serde_json::from_slice(&body).map_err(From::from)))
}
