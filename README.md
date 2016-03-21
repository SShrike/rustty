# RusTTY

RusTTY is a toolbox for developing command line applications in Rust, influenced
by the [TTY library][tty] for Ruby. RusTTY intends to reach feature parity with
the majority of TTY's features to the extent that is practical as well as have
an extensive and thorough test suite.

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
rustty = "^0.1.0"
```

[tty]: http://peter-murach.github.io/tty/
