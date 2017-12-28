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

#[macro_use] extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[cfg(feature = "futures")]
extern crate futures;
#[cfg(feature = "hyper")]
extern crate hyper;
#[cfg(feature = "hyper-tls")]
extern crate hyper_tls;
#[cfg(feature = "reqwest")]
extern crate reqwest;

pub mod bridge;

mod error;
mod model;

pub use error::{Error, Result};
pub use model::{Definition, Response};

#[cfg(feature = "hyper-support")]
pub use bridge::hyper::UrbanDictionaryRequester as HyperUrbanDictionaryRequester;
#[cfg(feature = "reqwest-support")]
pub use bridge::reqwest::UrbanDictionaryRequester as ReqwestUrbanDictionaryRequester;
