# Tutil

[![Coveralls][coveralls-badge]][coveralls]
[![Crates.io][crates-io-badge]][crates-io]
[![Chat Room][chat-room-badge]][chat-room]
[![License][license-badge]][license]

| Build Status |                                            |
|--------------|--------------------------------------------|
| Travis       | [![Travis CI][travis-ci-badge]][travis-ci] |
| AppVeyor     | [![AppVeyor][appveyor-badge]][appveyor]    |

Tutil is a toolbox for developing command line applications in Rust, influenced
by the [TTY library][tty] for Ruby which intends to reach feature parity with
the majority of TTY's features to the extent that is practical in addition to
having an extensive and thorough test suite.

Tutil intends to support Linux, BSD, macOS (formerly known as OS X) and Windows.
However, in this early stage of development, Tutil in its entirety probably only
works on Linux.

The stable documentation can by accessed [here](https://docs.rs/tutil).

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
[ti]: https://github.com/SShrike/tutil/issues/1
[tty]: http://piotrmurach.github.io/tty/
<!-- Badge links and SVGs -->
[travis-ci]: https://travis-ci.org/SShrike/tutil
[travis-ci-badge]: https://img.shields.io/travis/SShrike/tutil.svg
[appveyor]: https://ci.appveyor.com/project/SShrike/tutil
[appveyor-badge]: https://img.shields.io/appveyor/ci/SShrike/tutil.svg
[coveralls]: https://coveralls.io/github/SShrike/tutil
[coveralls-badge]: https://img.shields.io/coveralls/SShrike/tutil.svg
[crates-io]: https://crates.io/crates/tutil
[crates-io-badge]: https://img.shields.io/crates/v/tutil.svg
[chat-room]: https://vector.im/beta/#/room/#tutil:matrix.org
[chat-room-badge]: https://img.shields.io/badge/chat-%23tutil%3Amatrix.org-00B4B7.svg
[license]: https://www.mozilla.org/en-GB/MPL/2.0/
[license-badge]: https://img.shields.io/crates/l/tutil.svg
