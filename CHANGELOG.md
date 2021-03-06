# Change Log
All notable changes to this project will be documented in this file.
This project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased] - ∞

## Changed
- Improved the tests for `tutil::screen::unix`.
- Refactored `tutil::screen::unix` slightly.
- Slightly improved the documentaton for `tutil::screen`.
- Improved the example code.

## [0.2.0] - 2016-08-26

### Added
- A provision script for the Vagrant environment.
- A Vagrant environment for testing on FreeBSD locally.
- Continuous integration for macOS.
- An EditorConfig configuration file, in order to keep code style consistent.
- The `tutil::screen` module, supporting both Unix and Windows.
- Windows CI using AppVeyor.

### Changed
- Replaced the Gitter room with a
  [Matrix room](https://vector.im/beta/#/room/#tutil:matrix.org).
- The `doc_markdown` code lint has been disabled.

### Fixed
- The `unknown_lints` warning, which was triggered due to a missing
  `#[cfg_attr()]`.

## [0.1.1] - 2016-03-23

### Added
- A documentation link in `Cargo.toml` and README.

## 0.1.0 - 2016-03-22

### Added
- The `tutil::pastel` module for terminal colourisation and styling.

[0.1.1]: https://github.com/SShrike/tutil/compare/v0.1.0...v0.1.1
[0.2.0]: https://github.com/SShrike/tutil/compare/v0.1.1...v0.2.0
[Unreleased]: https://github.com/SShrike/tutil/compare/v0.2.0...master
