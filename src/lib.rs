//! [![ci-badge][]][ci] [![license-badge][]][license] [![docs-badge][]][docs]
//!
//! # urbandictionary.rs
//!
//! Unofficial Rust crate for the Urbandictionary API.
//!
//! [Documentation][docs]
//!
//! ### Installation
//!
//! Add the following dependency to your Cargo.toml:
//!
//! ```toml
//! urbandictionary = "0.2"
//! ```
//!
//! And include it in your project:
//!
//! ```rust,no_run
//! extern crate urbandictionary;
//! ```
//!
//! ### Examples
//!
//! Retrieve a list of definitions for a word:
//!
//! ```rust,ignore
//! extern crate hyper;
//! extern crate hyper_tls;
//! extern crate tokio_core;
//!
//! use hyper::client::{Client, HttpConnector};
//! use hyper_tls::HttpsConnector;
//! use tokio_tore::reactor::Core;
//! use urbandictionary::UrbanDictionaryRequester;
//!
//! # use std::error::Error;
//! #
//! # fn try_main() -> Result<(), Box<Error>> {
//! #
//! let mut core = Core::new()?;
//! let client = Client::configure()
//!     .connector(HttpsConnector::new(4, &core.handle())?)
//!     .build(&core.handle());
//!
//! let done = client.definitions("cat").and_then(|response| {
//!     if let Some(definition) = response.definitions.first() {
//!         println!("Examples: {}", definition.example);
//!     }
//!
//!     Ok(())
//! }).map_err(|_| ());
//!
//! core.run(done)?;
//! #     Ok(())
//! # }
//! #
//! # fn main() {
//! #     try_main().unwrap();
//! # }
//! ```
//!
//! Retrieve the top definition for a word:
//!
//! ```rust,ignore
//! use urbandictionary::UrbanClient;
//!
//! let client = UrbanClient::new();
//!
//! let definition = client.define("cat");
//! ```
//!
//! ### License
//!
//! License info in [LICENSE.md]. Long story short, ISC.
//!
//! [ci]: https://travis-ci.org/zeyla/urbandictionary.rs
//! [ci-badge]: https://travis-ci.org/zeyla/urbandictionary.svg?branch=master
//! [docs]: https://docs.rs/crate/urbandictionary
//! [docs-badge]: https://img.shields.io/badge/docs-online-2020ff.svg
//! [LICENSE.md]: https://github.com/zeyla/urbandictionary.rs/blob/master/LICENSE.md
//! [license]: https://opensource.org/licenses/ISC
//! [license-badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=flat-square
#![deny(missing_docs)]

#[macro_use]
extern crate serde_derive;

extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate serde;
extern crate serde_json;

mod error;
mod model;

pub use error::{Error, Result};
pub use model::{Definition, Response};

use futures::{Future, Stream, future};
use hyper::client::{Client, HttpConnector};
use hyper::{Body, Uri};
use hyper_tls::HttpsConnector;
use std::str::FromStr;

/// Trait implemented on HTTP client(s) for interaction with the UrbanDictionary
/// API.
pub trait UrbanDictionaryRequester {
    /// Attempt to retrieve the first `Definition` for a word.
    fn define<'a>(&'a self, word: &'a str)
        -> Box<Future<Item = Option<Definition>, Error = Error> + 'a>;

    /// Attempt to retrieve the definitions of a word.
    fn definitions<'a>(&'a self, word: &'a str)
        -> Box<Future<Item = Response, Error = Error> + 'a>;
}

impl UrbanDictionaryRequester for Client<HttpsConnector<HttpConnector>, Body> {
    /// Attempt to retrieve the first `Definition` for a word.
    fn define<'a>(&'a self, word: &'a str)
        -> Box<Future<Item = Option<Definition>, Error = Error> + 'a> {
        let url = format!(
            "http://api.urbandictionary.com/v0/define?term={}",
            word,
        );
        let uri = match Uri::from_str(&url) {
            Ok(v) => v,
            Err(why) => return Box::new(future::err(Error::Uri(why))),
        };

        Box::new(future::ok(uri)
            .and_then(move |uri| self.get(uri))
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
    fn definitions<'a>(&'a self, word: &'a str)
        -> Box<Future<Item = Response, Error = Error> + 'a> {
        let url = format!(
            "http://api.urbandictionary.com/v0/define?term={}",
            word,
        );
        let uri = match Uri::from_str(&url) {
            Ok(v) => v,
            Err(why) => return Box::new(future::err(Error::Uri(why))),
        };

        Box::new(future::ok(uri)
            .and_then(move |uri| self.get(uri))
            .and_then(|res| res.body().concat2())
            .map_err(From::from)
            .and_then(|body| serde_json::from_slice(&body).map_err(From::from)))
    }
}
