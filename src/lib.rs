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
//! Using `hyper` with the `hyper-tls` HTTPS connector, retrieve a list of
//! definitions for a word and print the example of the second definition if it
//! exists:
//!
//! ```rust,no_run
//! # #[cfg(feature = "hyper-support")]
//! extern crate hyper;
//! # #[cfg(feature = "hyper-support")]
//! extern crate hyper_tls;
//! # #[cfg(feature = "hyper-support")]
//! extern crate tokio_core;
//!
//! # use std::error::Error;
//! #
//! # #[cfg(feature = "hyper-support")]
//! # fn try_main() -> Result<(), Box<Error>> {
//! #
//! use hyper::client::{Client, HttpConnector};
//! use hyper_tls::HttpsConnector;
//! use tokio_tore::reactor::Core;
//! use urbandictionary::HyperUrbanDictionaryRequester;
//!
//! let mut core = Core::new()?;
//! let client = Client::configure()
//!     .connector(HttpsConnector::new(4, &core.handle())?)
//!     .build(&core.handle());
//!
//! let done = client.definitions("cat").and_then(|response| {
//!     if let Some(definition) = response.definitions.get(1) {
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
//! #    #[cfg(feature = "hyper-support")]
//! #    try_main().unwrap();
//! # }
//! ```
//!
//! Using reqwest, print the definition of the word `"cat"`:
//!
//! ```rust,no_run
//! # #[cfg(feature = "reqwest-support")]
//! #
//! extern crate reqwest;
//! extern crate urbandictionary;
//!
//! # use std::error::Error;
//! #
//! # #[cfg(feature = "reqwest-support")]
//! # fn try_main() -> Result<(), Box<Error>> {
//! #
//!
//! use reqwest::Client;
//! use urbandictionary::ReqwestUrbanDictionaryRequester;
//!
//! let client = Client::new();
//! let response = client.define("cat")?;
//!
//! if let Some(definition) = response {
//!     println!("The definition of cat is: {}", definition.definition);
//! } else {
//!     println!("No definition found");
//! }
//! #     Ok(())
//! # }
//! #
//! # fn main() {
//! #     #[cfg(feature = "reqwest-support")]
//! #     try_main().unwrap();
//! # }
//! ```
//!
//! ### License
//!
//! License info in [LICENSE.md]. Long story short, ISC.
//!
//! [ci]: https://travis-ci.org/zeyla/urbandictionary.rs
//! [ci-badge]: https://travis-ci.org/zeyla/urbandictionary.rs.svg?branch=master
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
