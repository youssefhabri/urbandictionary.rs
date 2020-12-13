//! Support for the `hyper` crate.

use crate::{
    model::{Definition, Response},
    Result,
};
use hyper::{
    body::to_bytes,
    client::{connect::Connect, Client},
    Uri,
};
use serde_json;
use std::str::FromStr;

/// Trait implemented on Hyper's client for interaction with the UrbanDictionary
/// API.
#[async_trait::async_trait]
pub trait UrbanDictionaryRequester {
    /// Attempt to retrieve the first `Definition` for a word.
    ///
    /// # Examples
    ///
    /// Using the `hyper-tls` HTTPS connector, print the definition for the word
    /// `"cat"`, if it exists:
    ///
    /// ```rust,no_run
    /// extern crate futures;
    /// extern crate hyper;
    /// extern crate hyper_tls;
    /// extern crate tokio_core;
    /// extern crate urbandictionary;
    ///
    /// # use std::error::Error;
    /// #
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use futures::Future;
    /// use hyper::client::{Client, HttpConnector};
    /// use hyper_tls::HttpsConnector;
    /// use tokio_core::reactor::Core;
    /// use urbandictionary::HyperUrbanDictionaryRequester;
    ///
    /// let mut core = Core::new()?;
    /// let client = Client::configure()
    ///     .connector(HttpsConnector::new(4)?)
    ///     .build(&core.handle());
    ///
    /// let done = client.define("cat").map(|definition| {
    ///     if let Some(definition) = definition {
    ///         println!("Examples: {}", definition.example);
    ///     }
    ///
    ///     ()
    /// }).map_err(|_| ());
    ///
    /// core.run(done).expect("Error running core");
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    async fn define<T: AsRef<str> + Send>(&self, word: T) -> Result<Option<Definition>>;

    /// Attempt to retrieve the definitions of a word.
    ///
    /// # Examples
    ///
    /// Using the `hyper-tls` HTTPS connector, print the second definition for
    /// the word `"cat"`, if it exists:
    ///
    /// ```rust,no_run
    /// extern crate futures;
    /// extern crate hyper;
    /// extern crate hyper_tls;
    /// extern crate tokio_core;
    /// extern crate urbandictionary;
    ///
    /// # use std::error::Error;
    /// #
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// #
    /// use futures::Future;
    /// use hyper::client::{Client, HttpConnector};
    /// use hyper_tls::HttpsConnector;
    /// use tokio_core::reactor::Core;
    /// use urbandictionary::HyperUrbanDictionaryRequester;
    ///
    /// let mut core = Core::new()?;
    /// let client = Client::configure()
    ///     .connector(HttpsConnector::new(4)?)
    ///     .build(&core.handle());
    ///
    /// let done = client.definitions("cat").map(|response| {
    ///     if let Some(definition) = response.definitions.get(1) {
    ///         println!("Examples: {}", definition.example);
    ///     }
    ///
    ///     ()
    /// }).map_err(|_| ());
    ///
    /// core.run(done).expect("Error running core");
    /// #     Ok(())
    /// # }
    /// #
    /// # fn main() {
    /// #     try_main().unwrap();
    /// # }
    /// ```
    async fn definitions<T: AsRef<str> + Send>(&self, word: T) -> Result<Response>;
}

#[async_trait::async_trait]
impl<C> UrbanDictionaryRequester for Client<C>
where
    C: Connect + Clone + Send + Sync + 'static,
{
    /// Attempt to retrieve the first `Definition` for a word.
    async fn define<T: AsRef<str> + Send>(&self, word: T) -> Result<Option<Definition>> {
        define(self, word).await
    }

    /// Attempt to retrieve the definitions of a word.
    #[inline]
    async fn definitions<T: AsRef<str> + Send>(&self, word: T) -> Result<Response> {
        definitions(self, word).await
    }
}

/// Attempt to retrieve the first `Definition` for a word.
pub async fn define<C, T>(client: &Client<C>, word: T) -> Result<Option<Definition>>
where
    C: Connect + Clone + Send + Sync + 'static,
    T: AsRef<str>,
{
    let mut definitions = definitions(client, word).await?;

    Ok(if !definitions.definitions.is_empty() {
        Some(definitions.definitions.remove(0))
    } else {
        None
    })
}

/// Attempt to retrieve the definitions of a word.
pub async fn definitions<C, T>(client: &Client<C>, word: T) -> Result<Response>
where
    C: Connect + Clone + Send + Sync + 'static,
    T: AsRef<str>,
{
    let url = format!(
        "http://api.urbandictionary.com/v0/define?term={}",
        word.as_ref(),
    );
    let uri = Uri::from_str(&url)?;
    let resp = client.get(uri).await?;
    let bytes = to_bytes(resp.into_body()).await?;

    serde_json::from_slice(&bytes.to_vec()).map_err(From::from)
}
