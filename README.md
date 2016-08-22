[travis-badge]: https://img.shields.io/travis/zeyla/urbandictionary.rs.svg?style=flat-square
[travis]: https://travis-ci.org/zeyla/urbandictionary.rs
[license-badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=flat-square
[license]: https://opensource.org/licenses/ISC
[docs-badge]: https://img.shields.io/badge/docs-online-2020ff.svg
[docs]: https://docs.austinhellyer.me/urbandictionary/

[![travis-badge][]][travis] [![license-badge][]][license] [![docs-badge][]][docs]

# urbandictionary.rs

Unofficial Rust crate for the Urbandictionary API.

[Documentation](http://docs.austinhellyer.me/urbandictionary)


### Installation

Add the following dependency to your Cargo.toml:

```toml
urbandictionary = "0.1"
```

And include it in your project:

```rust
extern crate urbandictionary;
```


### Examples

Retrieve a list of definitions for a word:

```rust
let definitions = urbandictionary::definitions("cat");
```

Retrieve the top definition for a word:

```rust
let definition = urbandictionary::define("cat");
```


### License

License info in [LICENSE.md]. Long story short, ISC.

[LICENSE.md]: https://github.com/zeyla/urbandictionary.rs/blob/master/LICENSE.md
