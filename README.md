[![ci-badge][]][ci] [![license-badge][]][license] [![docs-badge][]][docs]

# urbandictionary.rs

Unofficial Rust crate for the Urbandictionary API.

[Documentation][docs]

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

[ci]: https://travis-ci.org/zeyla/urbandictionary.rs
[ci-badge]: https://travis-ci.org/zeyla/urbandictionary.svg?branch=master
[docs]: https://docs.rs/crate/urbandictionary
[docs-badge]: https://img.shields.io/badge/docs-online-2020ff.svg
[LICENSE.md]: https://github.com/zeyla/urbandictionary.rs/blob/master/LICENSE.md
[license]: https://opensource.org/licenses/ISC
[license-badge]: https://img.shields.io/badge/license-ISC-blue.svg?style=flat-square
