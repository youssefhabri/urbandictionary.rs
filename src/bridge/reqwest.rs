//! Support for the `reqwest` crate.
//!
//! # Examples
//!
//! Refer to the tymethods of [`UrbanDictionaryRequester`].
//!
//! [`UrbanDictionaryRequester`]: trait.UrbanDictionaryRequester.html

use crate::model::{Definition, Response};
use crate::Result;
use reqwest::Client;

/// Trait implemented on Reqwest's client for interaction with the
/// UrbanDictionary API.
#[async_trait::async_trait]
pub trait UrbanDictionaryRequester {
    /// Attempt to retrieve the first `Definition` for a word.
    ///
    /// # Examples
    ///
    /// Retrieve the definition of the word `"cat"`:
    ///
    /// ```rust,no_run,ignore
    /// extern crate reqwest;
    /// extern crate urbandictionary;
    ///
    /// # use std::error::Error;
    /// #
    /// # async fn try_main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use reqwest::Client;
    /// use urbandictionary::ReqwestUrbanDictionaryRequester;
    ///
    /// let client = Client::new();
    /// let response = client.define("cat").await?;
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
    async fn define(&self, word: &str) -> Result<Option<Definition>>;

    /// Attempt to retrieve the definitions for a word.
    ///
    /// # Examples
    ///
    /// Retrieve the definitions for the word `"cat"`:
    ///
    /// ```rust,no_run,ignore
    /// extern crate reqwest;
    /// extern crate urbandictionary;
    ///
    /// # use std::error::Error;
    /// #
    /// # async fn try_main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use reqwest::Client;
    /// use urbandictionary::ReqwestUrbanDictionaryRequester;
    ///
    /// let client = Client::new();
    /// let response = client.definitions("cat").await?;
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
    /// #     tokio::block_on(try_main()).unwrap();
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
    async fn definitions(&self, word: &str) -> Result<Response>;
}

#[async_trait::async_trait]
impl UrbanDictionaryRequester for Client {
    /// Attempt to retrieve the first `Definition` for a word.
    async fn define(&self, word: &str) -> Result<Option<Definition>> {
        self.definitions(word).await.map(|mut response| {
            if response.definitions.is_empty() {
                None
            } else {
                Some(response.definitions.remove(0))
            }
        })
    }

    /// Attempt to retrieve the definitions for a word.
    async fn definitions(&self, word: &str) -> Result<Response> {
        let uri = format!("http://api.urbandictionary.com/v0/define?term={}", word);
        let response = self.get(&uri).send().await?;
        response.json().await.map_err(From::from)
    }
}
