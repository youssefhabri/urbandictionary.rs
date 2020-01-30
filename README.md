[![ci-badge][]][ci] [![license-badge][]][license] [![docs-badge][]][docs] [![rust badge]][rust link]

# urbandictionary.rs

Unofficial Rust crate for the Urbandictionary API.

[Documentation][docs]

### Installation

This library requires at least Rust 1.21.

Add the following dependency to your Cargo.toml:

```toml
urbandictionary = "0.3"
```

And include it in your project:

```rust
extern crate urbandictionary;
```

### Examples

Using `hyper` with the `hyper-tls` HTTPS connector, retrieve a list of
definitions for a word and print the example of the second definition if it
exists:

```rust
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate urbandictionary;

use futures::Future;
use hyper::client::{Client, HttpConnector};
use hyper_tls::HttpsConnector;
use std::error::Error;
use tokio_core::reactor::Core;
use urbandictionary::HyperUrbanDictionaryRequester;

fn try_main() -> Result<(), Box<Error>> {
    let mut core = Core::new()?;
    let client = Client::configure()
        .connector(HttpsConnector::new(4, &core.handle())?)
        .build(&core.handle());

    let done = client.definitions("cat").and_then(|response| {
        if let Some(definition) = response.definitions.get(1) {
            println!("Examples: {}", definition.example);
        }

        Ok(())
    }).map_err(|_| ());

    core.run(done).expect("Error running core");

    Ok(())
}

fn main() {
    try_main().unwrap();
}
```

Using reqwest, print the definition of the word `"cat"`:

```rust
extern crate reqwest;
extern crate urbandictionary;

use reqwest::blocking::Client;
use std::error::Error;
use urbandictionary::ReqwestUrbanDictionaryRequester;

fn try_main() -> Result<(), Box<Error>> {
    let client = Client::new();
    let response = client.define("cat")?;

    if let Some(definition) = response {
        println!("The definition of cat is: {}", definition.definition);
    } else {
        println!("No definition found");
    }

    Ok(())
}

fn main() {
    try_main().unwrap();
}
```

### License

License info in [LICENSE.md]. Long story short, ISC.

[ci]: https://travis-ci.org/zeyla/urbandictionary.rs
[ci-badge]: https://img.shields.io/travis/zeyla/urbandictionary.rs.svg?style=flat-square
[docs]: https://docs.rs/crate/urbandictionary
[docs-badge]: https://img.shields.io/badge/docs-online-2020ff.svg?style=flat-square
[license.md]: https://github.com/zeyla/urbandictionary.rs/blob/master/LICENSE.md
[license]: https://opensource.org/licenses/ISC
[license-badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=flat-square
[rust badge]: https://img.shields.io/badge/rust-1.21+-93450a.svg?style=flat-square
[rust link]: https://blog.rust-lang.org/2017/10/12/Rust-1.21.html
