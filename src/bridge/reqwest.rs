//! Support for the `reqwest` crate.
//!
//! # Examples
//!
//! Refer to the tymethods of [`UrbanDictionaryRequester`].
//!
//! [`UrbanDictionaryRequester`]: trait.UrbanDictionaryRequester.html

use reqwest::Client;
use serde_json;
use crate::model::{Definition, Response};
use crate::Result;

/// Trait implemented on Reqwest's client for interaction with the
/// UrbanDictionary API.
pub trait UrbanDictionaryRequester {
    /// Attempt to retrieve the first `Definition` for a word.
    ///
    /// # Examples
    ///
    /// Retrieve the definition of the word `"cat"`:
    ///
    /// ```rust,no_run
    /// extern crate reqwest;
    /// extern crate urbandictionary;
    ///
    /// # use std::error::Error;
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// #
    /// use reqwest::Client;
    /// use urbandictionary::ReqwestUrbanDictionaryRequester;
    ///
    /// let client = Client::new();
    /// let response = client.define("cat")?;
    ///
    /// if let Some(definition) = response {
    ///     println!("The definition of cat is: {}", definition.definition);
    /// } else {
    ///     println!("No definition found");
    /// }
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Json`] if there was an error deserializing the response
    /// body.
    ///
    /// Returns [`Error::Reqwest`] if there was an error sending the request.
    ///
    /// [`Error::Json`]: ../../enum.Error.html#variant.Json
    /// [`Error::Reqwest`]: ../../enum.Error.html#variant.Reqwest
    fn define(&self, word: &str) -> Result<Option<Definition>>;

    /// Attempt to retrieve the definitions for a word.
    ///
    /// # Examples
    ///
    /// Retrieve the definitions for the word `"cat"`:
    ///
    /// ```rust,no_run
    /// extern crate reqwest;
    /// extern crate urbandictionary;
    ///
    /// # use std::error::Error;
    /// #
    /// # fn try_main() -> Result<(), Box<Error>> {
    /// #
    /// use reqwest::Client;
    /// use urbandictionary::ReqwestUrbanDictionaryRequester;
    ///
    /// let client = Client::new();
    /// let response = client.definitions("cat")?;
    ///
    /// if let Some(definition) = response.definitions.first() {
    ///     println!(
    ///         "The first definition of cat is: {}",
    ///         definition.definition,
    ///     );
    /// } else {
    ///     println!("No definitions found");
    /// }
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`Error::Json`] if there was an error deserializing the response
    /// body.
    ///
    /// Returns [`Error::Reqwest`] if there was an error sending the request.
    ///
    /// [`Error::Json`]: ../../enum.Error.html#variant.Json
    /// [`Error::Reqwest`]: ../../enum.Error.html#variant.Reqwest
    fn definitions(&self, word: &str) -> Result<Response>;
}

impl UrbanDictionaryRequester for Client {
    /// Attempt to retrieve the first `Definition` for a word.
    fn define(&self, word: &str) -> Result<Option<Definition>> {
        self.definitions(word).map(|mut response| {
            if response.definitions.is_empty() {
                None
            } else {
                Some(response.definitions.remove(0))
            }
        })
    }

    /// Attempt to retrieve the definitions for a word.
    fn definitions(&self, word: &str) -> Result<Response> {
        let uri = format!("http://api.urbandictionary.com/v0/define?term={}", word);

        let response = self.get(&uri).send()?;

        serde_json::from_reader(response).map_err(From::from)
    }
}
