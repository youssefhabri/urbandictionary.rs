extern crate hyper;
extern crate serde_json;

pub mod error;

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
