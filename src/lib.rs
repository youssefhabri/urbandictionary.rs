extern crate hyper;
extern crate serde_json;

#[macro_use]
mod utils;

pub mod error;

pub use error::{Error, Result};
use hyper::header::Connection;
use hyper::Client;
use serde_json::Value;
use utils::*;

/// A definition and example for a word, including metadata such as the author's
/// name and the definition's rating.
#[derive(Clone, Debug)]
pub struct Definition {
    pub author: String,
    pub definition: String,
    pub example: String,
    pub id: u64,
    pub permalink: String,
    pub thumbs_down: u64,
    pub thumbs_up: u64,
    pub word: String,
}

impl Definition {
    fn decode(value: Value) -> Result<Definition> {
        let mut value = try!(into_map(value));

        Ok(Definition {
            author: try!(remove(&mut value, "author").and_then(into_string)),
            definition: try!(remove(&mut value, "definition").and_then(into_string)),
            example: try!(remove(&mut value, "example").and_then(into_string)),
            id: req!(try!(remove(&mut value, "defid")).as_u64()),
            permalink: try!(remove(&mut value, "permalink").and_then(into_string)),
            thumbs_down: req!(try!(remove(&mut value, "thumbs_down")).as_u64()),
            thumbs_up: req!(try!(remove(&mut value, "thumbs_up")).as_u64()),
            word: try!(remove(&mut value, "word").and_then(into_string)),
        })
    }
}

/// A full response for a word, including the related tags and a list of
/// `Definition`s.
#[derive(Clone, Debug)]
pub struct Response {
    pub definitions: Vec<Definition>,
    pub tags: Vec<String>,
}

impl Response {
    fn decode(value: Value) -> Result<Response> {
        let mut value = try!(into_map(value));

        Ok(Response {
            definitions: try!(decode_array(try!(remove(&mut value, "list")), Definition::decode)),
            tags: try!(remove(&mut value, "tags").and_then(|v| decode_array(v, into_string))),
        })
    }
}

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

    Response::decode(try!(serde_json::from_reader(response)))
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
