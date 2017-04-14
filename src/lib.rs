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
//! urbandictionary = "0.1"
//! ```
//!
//! And include it in your project:
//!
//! ```rust
//! extern crate urbandictionary;
//! ```
//!
//! ### Examples
//!
//! Retrieve a list of definitions for a word:
//!
//! ```rust,no_run
//! use urbandictionary::UrbanClient;
//!
//! let client = UrbanClient::new();
//! let definitions = client.definitions("cat");
//! ```
//!
//! Retrieve the top definition for a word:
//!
//! ```rust,no_run
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

#[macro_use] extern crate serde_derive;

extern crate hyper;
extern crate serde;
extern crate serde_json;

mod error;
mod model;

pub use error::{Error, Result};
pub use model::{Definition, Response};

use hyper::client::{Client, Response as HyperResponse};
use hyper::header::Connection;

/// A thin wrapper around a
#[derive(Debug, Default)]
pub struct UrbanClient {
    client: Client,
}

impl UrbanClient {
    /// Creates a new UrbanClient with a `hyper` Client instance.
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Attempt to retrieve the first `Definition` for a word.
    pub fn define(&self, word: &str) -> Result<Option<Definition>> {
        let mut request = self.request(word)?;

        Ok(if !request.definitions.is_empty() {
            Some(request.definitions.remove(0))
        } else {
            None
        })
    }

    /// Attempt to retrieve the definitions of a word.
    pub fn definitions(&self, word: &str) -> Result<Response> {
        self.request(word.into())
    }

    fn request(&self, word: &str) -> Result<Response> {
        // UrbanDictionary's API does not support HTTPS at this time
        let url = format!("http://api.urbandictionary.com/v0/define?term={}", word);

        let response = self.client.get(&url).header(Connection::close()).send()?;

        serde_json::from_reader::<HyperResponse, Response>(response).map_err(From::from)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn define() {
        let client = ::UrbanClient::new();
        assert!(client.define("cat").is_ok());
    }

    #[test]
    fn definitions() {
        let client = ::UrbanClient::new();
        assert!(client.define("cat").is_ok());
    }
}
