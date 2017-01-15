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
//! ```rust
//! let definitions = urbandictionary::definitions("cat");
//! ```
//!
//! Retrieve the top definition for a word:
//!
//! ```rust
//! let definition = urbandictionary::define("cat");
//! ```
//!
//! ### License
//!
//! License info in [LICENSE.md]. Long story short, ISC.
//!
//! [ci]: https://travis-ci.org/zeyla/urbandictionary.rs
//! [ci-badge]: https://travis-ci.org/zeyla/urbandictionary.svg?branch=master
//! [docs]: https://docs.austinhellyer.me/urbandictionary
//! [docs-badge]: https://img.shields.io/badge/docs-online-2020ff.svg
//! [LICENSE.md]: https://github.com/zeyla/urbandictionary.rs/blob/master/LICENSE.md
//! [license]: https://opensource.org/licenses/ISC
//! [license-badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=flat-square
#![deny(missing_docs)]

extern crate hyper;
extern crate serde_json;

mod error;

mod model {
    include!(concat!(env!("OUT_DIR"), "/model.rs"));
}

pub use error::{Error, Result};
pub use model::*;

use hyper::header::Connection;
use hyper::client::{Client, Response as HyperResponse};

/// Attempt to retrieve the first `Definition` for a word.
pub fn define<S: Into<String>>(word: S) -> Result<Option<Definition>> {
    let mut request = try!(request(word.into()));

    Ok(if request.definitions.len() > 0 {
        Some(request.definitions.remove(0))
    } else {
        None
    })
}

/// Attempt to retrieve the definitions of a word.
pub fn definitions<S: Into<String>>(word: S) -> Result<Response> {
    request(word.into())
}

fn request(word: String) -> Result<Response> {
    let client = Client::new();
    // UrbanDictionary's API does not support HTTPS at this time
    let url = format!("http://api.urbandictionary.com/v0/define?term={}", word);

    let response = try!(client.get(&url)
        .header(Connection::close())
        .send());

    Ok(serde_json::from_reader::<HyperResponse, Response>(response)?)
}

#[cfg(test)]
mod tests {
    #[test]
    fn define() {
        assert!(::definitions("cat").is_ok());
    }

    #[test]
    fn definitions() {
        assert!(::define("cat".to_owned()).is_ok());
    }
}
