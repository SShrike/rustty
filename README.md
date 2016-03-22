# Tutil

[![Travis CI][travis-ci-badge]][travis-ci]
[![Gitter][gitter-badge]][gitter]

Tutil is a toolbox for developing command line applications in Rust, influenced
by the [TTY library][tty] for Ruby which intends to reach feature parity with
the majority of TTY's features to the extent that is practical in addition to
having an extensive and thorough test suite.

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

## Installation

Add the following to your `Cargo.toml` under the dependencies section:

```toml
[dependencies]
tutil = "^0.1.0"
```

<!-- Links -->
[tty]: http://peter-murach.github.io/tty/
<!-- Badge links and SVGs -->
[travis-ci]: https://travis-ci.org/SShrike/tutil
[travis-ci-badge]: https://img.shields.io/travis/SShrike/tutil.svg
[gitter]: https://gitter.im/SShrike/tutil
[gitter-badge]: https://img.shields.io/gitter/room/SShrike/tutil.svg
