// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Windows implementation of `tutil::screen`.
//!
//! Currently there are no tests written for this implementation.

use super::{Width, Height};

use winapi::{DWORD, HANDLE};
use winapi::{STD_INPUT_HANDLE, STD_OUTPUT_HANDLE};
use winapi::{CONSOLE_SCREEN_BUFFER_INFO, COORD, SMALL_RECT};
use kernel32::{GetStdHandle, GetConsoleScreenBufferInfo};

/// Returns the terminal screen size.
///
/// Returns `None` if the screen size is `(0, 0)` or is not able to be
/// determined.
pub fn size() -> Option<(Width, Height)> {
    // Retrieve a handle to STDOUT.
    let handle: HANDLE = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) };

    // An empty COORD struct for use with CONSOLE_SCREEN_BUFFER_INFO.
    let coord = COORD { X: 0, Y: 0 };

    // An empty CONSOLE_SCREEN_BUFFER_INFO struct for use with
    // GetConsoleScreenBufferInfo.
    let mut csbi = CONSOLE_SCREEN_BUFFER_INFO {
        // A COORD structure that contains the size of the console screen
        // buffer, in character columns and rows.
        dwSize: coord.clone(),

        // A COORD structure that contains the column and row coordinates of
        // the cursor in the console screen buffer.
        dwCursorPosition: coord.clone(),

        // The attributes of the characters written to a screen buffer by the
        // WriteFile and WriteConsole functions, or echoed to a screen buffer
        // by the ReadFile and ReadConsole functions.
        wAttributes: 0,

        // A SMALL_RECT structure that contains the console screen buffer
        // coordinates of the upper-left and lower-right corners of the
        // display window.
        srWindow: SMALL_RECT{ Left: 0, Top: 0, Right: 0, Bottom: 0 },

        // A COORD structure that contains the maximum size of the console
        // window, in character columns and rows, given the current screen
        // buffer size and font and the screen size.
        dwMaximumWindowSize: coord,
    };

    let success: bool = unsafe {
        GetConsoleScreenBufferInfo(hand, &mut csbi) != 0
    };

    if success {
        let width = Width((csbi.srWindow.Right - csbi.srWindow.Left + 1) as u16);
        let height = Height((csbi.srWindow.Bottom - csbi.srWindow.Top + 1) as u16);
        Some((width, height))
    } else {
        None
    }
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
    // TODO: Write tests.
}
