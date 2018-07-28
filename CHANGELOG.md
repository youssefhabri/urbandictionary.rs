# Change Log
All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [0.3.0] - 2017-01-16

### Changed

- Rewrite the library; now includes trait requester implementations for
hyper v0.11's Client and Reqwest v0.8's client

## [0.2.0] - 2017-08-02

### Changed

- The `define` and `definitions` functions now accepted a string slice;
- A new struct has been made which is a thin wrapper over a Hyper Client;
- Now uses serde_derive;
- Dependencies updated.

## [0.1.2] - 2017-01-15

### Added

### Changed

- `error` is no longer a public module; use the re-exports instead [BC break];
- models are now deserialized via serde_codegen;
- documentation is now enforced on all items.

## [0.1.1] - 2016-08-21

### Added

### Changed

- Make every field public

## [0.1.0] - 2016-08-21

Initial commit.


[0.2.0]: https://github.com/zeyla/urbandictionary.rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/zeyla/urbandictionary.rs/compare/v0.1.2...v0.2.0
[0.1.2]: https://github.com/zeyla/urbandictionary.rs/compare/v0.1.1...v0.1.2
[0.1.1]: https://github.com/zeyla/urbandictionary.rs/compare/v0.1.0...v0.1.1
