// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Functions for detecting the terminal screen size.
//!
//! # OS Support
//!
//! The following operating systems are supported:
//!
//! * Linux
//! * macOS
//! * FreeBSD
//! * Windows

use std::fmt;

/// Represents the width of a terminal.
#[derive(Debug)]
pub struct Width(pub u16);

impl fmt::Display for Width {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Width(width) = *self;

        try!(write!(f, "{}", width));
        Ok(())
    }
}

/// Represents the height of a terminal.
#[derive(Debug)]
pub struct Height(pub u16);

impl fmt::Display for Height {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Height(height) = *self;

        try!(write!(f, "{}", height));
        Ok(())
    }
}

#[cfg(unix)]
mod unix;

#[cfg(unix)]
pub use self::unix::size;
#[cfg(unix)]
pub use self::unix::width;
#[cfg(unix)]
pub use self::unix::height;

#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use self::windows::size;
#[cfg(windows)]
pub use self::windows::width;
#[cfg(windows)]
pub use self::windows::height;

#[cfg(test)]
mod test {
    // TODO: Test the `fmt::Display` implementations for `Width` and `Height`.
}
