// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Unix implementation of `tutil::screen`, should work on Linux, FreeBSD and OS X.

use super::{Width, Height};

use std::os::raw::c_ushort;
use libc::{ioctl, isatty, STDOUT_FILENO, TIOCGWINSZ};

/// The struct required by the `TIOCGWINSZ` syscall; specified in the following
/// [man page](http://www.delorie.com/djgpp/doc/libc/libc_495.html).
#[derive(Debug)]
struct WinSize {
    /// Rows, in characters.
    ws_row: c_ushort,
    /// Columns, in characters.
    ws_col: c_ushort,
    /// Horizontal size, in pixels.
    ws_xpixel: c_ushort,
    /// Vertical size, in pixels.
    ws_ypixel: c_ushort,
}

/// Returns the terminal screen size.
///
/// Returns `None` if the screen size is `(0, 0)` or is not able to be
/// determined.
pub fn size() -> Option<(Width, Height)> {
    let is_tty = unsafe { isatty(STDOUT_FILENO) == 1 };

    if !is_tty { return None; }

    let (width, height) = unsafe {
        let mut winsize = WinSize { ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0 };
        ioctl(STDOUT_FILENO, TIOCGWINSZ, &mut winsize);

        (winsize.ws_col, winsize.ws_row)
    };

    Some((Width(width), Height(height)))
}

/// Returns the terminal screen width.
///
/// Returns `None` if the terminal width is detected as being <= 0 columns or is
/// not able to be determined at all.
pub fn width() -> Option<Width> {
    let size = size();

    if let Some((Width(width), Height(_))) = size {
        Some(Width(width))
    } else {
        None
    }
}

/// Returns the terminal height.
///
/// Returns `None` if the terminal height is detected as being <= 0 rows or is
/// not able to be determined at all.
pub fn height() -> Option<Height> {
    let size = size();

    if let Some((Width(_), Height(height))) = size {
        Some(Height(height))
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::super::{Width, Height};

    use std::process::{Command, Stdio};

    #[cfg(target_os = "linux")]
    fn create_command() -> Command {
        let mut cmd = Command::new("stty");
        cmd.arg("--file");
        cmd.arg("/dev/stderr");
        cmd.arg("size");
        cmd.stderr(Stdio::inherit());
        cmd
    }

    #[cfg(any(target_os = "freebsd", target_os = "macos"))]
    fn create_command() -> Command {
        let mut cmd = Command::new("stty");
        cmd.arg("-f");
        cmd.arg("/dev/stderr");
        cmd.arg("size");
        cmd.stderr(Stdio::inherit());
        cmd
    }


    #[test]
    fn correct_size() {
        let output = create_command().output().unwrap();
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(output.status.success());

        let cols = u16::from_str_radix(stdout.split_whitespace().last().unwrap(), 10).unwrap();
        let rows = u16::from_str_radix(stdout.split_whitespace().next().unwrap(), 10).unwrap();

        if let Some((Width(width), Height(height))) = size() {
            assert_eq!(width, cols);
            assert_eq!(height, rows);
        }
    }

    #[test]
    fn correct_width() {
        let output = create_command().output().unwrap();
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(output.status.success());

        let cols = u16::from_str_radix(stdout.split_whitespace().last().unwrap(), 10).unwrap();

        if let Some((Width(width), Height(_))) = size() {
            assert_eq!(width, cols);
        }
    }

    #[test]
    fn correct_height() {
       let output = create_command().output().unwrap();
       let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(output.status.success());

        let rows = u16::from_str_radix(stdout.split_whitespace().next().unwrap(), 10).unwrap();

        if let Some((Width(_), Height(height))) = size() {
            assert_eq!(height, rows);
        }
    }
}
