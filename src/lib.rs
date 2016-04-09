// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Tutil is a toolbox for developing command line applications in Rust,
//! influenced by the [TTY library][tty] for Ruby. It provides modules for
//! common tasks such as colourisation, gathering information from the user,
//! confirmation prompts, and querying the system and terminal. It is not
//! intended to be a command line argument parser, rather, it is intended to be
//! a plumbing library for common tasks.
//!
//! This documentation is written in British English, however the API is
//! completely written in US English to match the design of the Rust standard
//! library and most crates in circulation. If you do find something in the API
//! that is written in British English file a [bug report][br] or submit a
//! [pull request][pr] correcting the spelling.
//!
//! [tty]: http://peter-murach.github.io/tty/
//! [br]: https://github.com/SShrike/tutil/issues
//! [pr]: https://github.com/SShrike/tutil/pulls

#![doc(html_root_url = "https://shrike.me/tutil/")]

#![cfg_attr(feature = "lints", feature(plugin))]
#![cfg_attr(feature = "lints", plugin(clippy))]

#![allow(doc_markdown)]

#![warn(missing_docs)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_extern_crates, unused_qualifications)]

#![cfg_attr(feature = "lints", allow(needless_return))]

pub mod crayon;
