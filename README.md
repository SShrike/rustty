# Tutil

[![Travis CI][travis-ci-badge]][travis-ci]
[![Coveralls][coveralls-badge]][coveralls]
[![Clippy][clippy-badge]][clippy]
[![Crates.io][crates-io-badge]][crates-io]
[![Gitter][gitter-badge]][gitter]
[![License][license-badge]][license]

Tutil is a toolbox for developing command line applications in Rust, influenced
by the [TTY library][tty] for Ruby which intends to reach feature parity with
the majority of TTY's features to the extent that is practical in addition to
having an extensive and thorough test suite.

The documentation can by accessed on my [website](https://shrike.me/tutil).

## Features

Implemented features will be marked with a tick (✔) and unimplemented features
will be marked with a cross (✗).

- Terminal output colourisation. ✔
- Terminal output paging. ✗
- Terminal ASCII and Unicode tables. ✗
- System detection utilities. ✗
- Command detection utilities. ✗
- Text manipulation (wrapping and truncation). ✗
- Terminal progress bars. ✗
- Terminal spinners. ✗
- User input prompts. ✗
- Windows support. ✗

There is a [tracking issue][ti] for these which may be more up-to-date.

## Installation

Add the following to your `Cargo.toml` under the dependencies section:

```toml
[dependencies]
tutil = "^0.1.0"
```

<!-- Links -->
[ti]: https:://github.com/SShrike/tutil/issues/1
[tty]: http://peter-murach.github.io/tty/
<!-- Badge links and SVGs -->
[travis-ci]: https://travis-ci.org/SShrike/tutil
[travis-ci-badge]: https://img.shields.io/travis/SShrike/tutil.svg
[coveralls]: https://coveralls.io/github/SShrike/tutil
[coveralls-badge]: https://img.shields.io/coveralls/SShrike/tutil.svg
[clippy]: https://clippy.bashy.io/github/SShrike/tutil/master/log
[clippy-badge]: https://clippy.bashy.io/github/SShrike/tutil/master/badge.svg
[crates-io]: https://crates.io/crates/tutil
[crates-io-badge]: https://img.shields.io/crates/v/tutil.svg
[gitter]: https://gitter.im/SShrike/tutil
[gitter-badge]: https://img.shields.io/gitter/room/SShrike/tutil.svg
[license]: https://www.mozilla.org/en-GB/MPL/2.0/
[license-badge]: https://img.shields.io/crates/l/tutil.svg
