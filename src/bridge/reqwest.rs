//! Support for the `reqwest` crate.

use reqwest::Client;
use serde_json;
use ::model::{Definition, Response};
use ::Result;

/// Trait implemented on Reqwest's client for interaction with the
/// UrbanDictionary API.
pub trait UrbanDictionaryRequester {
    /// Attempt to retrieve the first `Definition` for a word.
    fn define(&self, word: &str) -> Result<Option<Definition>>;

    /// Attempt to retrieve the definitions for a word.
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
