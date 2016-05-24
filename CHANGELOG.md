# Change Log
All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased] - ∞

### Added
- A provision script for the Vagrant environment.
- A Vagrant environment for testing on FreeBSD locally.
- The initial implementation of the `tutil::screen` module.
- The caching of Cargo dependencies on Travis CI.
- Continuous integration for OS X.
- An EditorConfig configuration file, in order to keep code style consistent.

### Changed
- A Git checkout of [libc](https://github.com/rust-lang/libc) is now used until
  the latest fixes for FreeBSD are released on crates.io.
- Replaced the Gitter room with a
  [Matrix room](https://vector.im/beta/#/room/#tutil:matrix.org).
- Code coverage is now only uploaded for CI builds on stable Rust.
- The `doc_markdown` code lint has been disabled.

### Fixed
- The `unknown_lints` warning, which was triggered due to a missing
  `#[cfg_attr()]`.

## [0.1.1] - 2016-03-23

### Added
- Documentation link in `Cargo.toml` and README.

## 0.1.0 - 2016-03-22

### Added
- `tutil::pastel` module for terminal colourisation and styling.

[0.1.1]: https://github.com/SShrike/tutil/compare/v0.1.0...v0.1.1
[Unreleased]: https://github.com/SShrike/tutil/compare/v0.1.1...master
