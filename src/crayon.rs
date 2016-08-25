// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A module for handling terminal output styling.
//!
//! It would be fair to say that this module ~~is a rip off of~~ is *heavily*
//! influenced by [ogham][ogham]'s [ansi_term][ansiterm] crate and
//! [peter-murach][peter]'s [Pastel][pastel] library. However if there is
//! something original of mine in here it would be true-colour support at the
//! very least.
//!
//! # OS Support
//!
//! * Linux
//! * OS X
//! * FreeBSD
//!
//! Most other POSIX/*nix systems will probably work as well.
//!
//! Windows support is planned.
//!
//! # Basic Usage
//!
//! The two main data structures in this module are `Style`, which holds
//! stylistic information such as the foreground colour, the background colour
//! and other properties such as if the text should be bold, blinking, etc. and
//! `ANSIString`, which is string paired with a `Style`.
//!
//! In order to format a string, call the `paint()` method on a `Style` or
//! `Color`. For example, here is how you get red text and blue text:
//!
//! ```
//! use tutil::crayon::Color::{Red, Blue};
//!
//! println!("{}", Red.paint("Hello world in red!"));
//! println!("{}", Blue.paint("Hello world in blue!"));
//! ```
//!
//! It is worth noting that `paint()` does not actually return a string with the
//! escape codes surrounding it, but instead returns a `StyledString` that has
//! an implementation of `Display` that will return the escape codes as well as
//! the string when formatted.
//!
//! In the case that you *do* want the escape codes, you can convert the
//! `StyledString` to a string:
//!
//! ```
//! use std::string::ToString;
//! use tutil::crayon::Color::Blue;
//!
//! let string = Blue.paint("Hello!").to_string(); // => "\x1b[34mTEST\x1b[0m"
//! ```
//!
//! # Advanced Usage
//!
//! For complex styling you can construct a `Style` object rather than operating
//! directly on a `Color`:
//!
//! ```
//! use tutil::crayon::Style;
//! use tutil::crayon::Color::{Red, Blue};
//!
//! // Red blinking text on a black background:
//! println!("This will be {} and this will be {}.",
//!          Style::new().foreground(Red).bold().paint("red and bold"),
//!          Style::new().foreground(Blue).italic().paint("blue and italic"));
//! ```
//!
//! The same styling methods that you can use on a `Style` have been implemented
//! on the `Color` struct, allowing you to skip creating an empty `Style` with
//! `Style::new()`:
//!
//! ```
//! use tutil::crayon::Color::{Red, Blue};
//!
//! // Red blinking text on a black background:
//! println!("This will be {} and this will be {}.",
//!          Red.bold().paint("red and bold"),
//!          Blue.italic().paint("blue and italic"));
//! ```
//!
//! [ogham]: https://github.com/ogham
//! [ansiterm]: https://github.com/ogham/rust-ansi-term
//! [peter]: https://github.com/peter-murach
//! [pastel]: https://github.com/peter-murach/pastel

use std::fmt;
use std::ops::Deref;
use std::borrow::Cow;
use std::default::Default;

use self::Color::*;

/// A string coupled with a `Style` in order to display it in a terminal.
///
/// It can be turned into a string with the `.to_string()` method.
#[derive(Debug, Clone, PartialEq)]
pub struct StyledString<'a> {
    string: Cow<'a, str>,
    style: Style,
}

impl<'a> fmt::Display for StyledString<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: Convert the `try!()` calls to the `?` operator once it is
        //       stable.
        try!(self.style.write_prefix(f));
        try!(write!(f, "{}", self.string));
        self.style.write_suffix(f)
    }
}

impl<'a, S> From<S> for StyledString<'a>
    where S: Into<Cow<'a, str>>
{
    fn from(string: S) -> StyledString<'a> {
        StyledString { string: string.into(), style: Style::default() }
    }
}

impl<'a> Deref for StyledString<'a> {
    type Target = str;

    fn deref(&self) -> &str {
        self.string.deref()
    }
}

/// A `Color` is a specific ANSI colour name which can refer to either the
/// foreground or background.
///
/// It can also be a custom value from 0 to 255 via the `Fixed(u8)` variant for
/// terminals that have 256-colour. True-colour is also supported via the
/// `Rgb(u8, u8, u8)` variant.
///
/// For further reading visit this [Wikipedia page][wp].
///
/// [wp]: https://en.wikipedia.org/wiki/ANSI_escape_code#Colors
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    /// Foreground code `30`, background code `40`.
    Black,

    /// Foreground code `31`, background code `41`.
    Red,

    /// Foreground code `32`, background code `42`.
    Green,

    /// Foreground code `33`, background code `43`.
    Yellow,

    /// Foreground code `34`, background code `44`.
    Blue,

    /// Foreground code `35`, background code `45`.
    Purple,

    /// Foreground code `36`, background code `46`.
    Cyan,

    /// Foreground code `37`, background code `47`.
    White,

    /// A value from 0 to 255 for use on terminals that have 256-colour support.
    ///
    /// * 0 to 7 are the `Black` through `White` variants. These colours can
    ///   usually be changed in the terminal emulator.
    /// * 8 to 15 are brighter versions of the aforementioned colours. These
    ///   colours can also usually be changed in the terminal emulator,
    ///   however, it also could be configured to use the original colours and
    ///   show the text in bold.
    /// * 16 to 231 contain several palettes of bright colours, arranged in six
    ///   squares measuring six by six each.
    /// * 232 to 255 are shades of grey from black to white.
    ///
    /// You can view this [colour chart][cc] for a visual representation.
    /// [cc]: https://upload.wikimedia.org/wikipedia/en/1/15/Xterm_256color_chart.svg
    Fixed(u8),

    /// A red value, green value and blue value, each between 0 to 255 (the RGB
    /// colour model) for use on terminals that have true-colour support.
    ///
    /// Please note that true-colour support on many terminal emulators is
    /// patchy, however a fair share of the most common terminal emulators
    /// support it, such as:
    ///
    /// * [Gnome Terminal][gterminal]
    /// * [Konsole][konsole]
    /// * [Terminator][terminator]
    /// * [iTerm 2][iterm2]
    ///
    /// For an up-to-date list of true-colour support visit:
    /// https://gist.github.com/XVilka/8346728.
    ///
    /// [gterminal]: https://wiki.gnome.org/Apps/Terminal
    /// [konsole]: https://konsole.kde.org/
    /// [terminator]: http://gnometerminator.blogspot.co.nz
    /// [iterm2]: https://www.iterm2.com/
    Rgb(u8, u8, u8),
}

impl Color {
    /// Convenience method for creating a `StyledString` with the foreground set
    /// without having to manually create a `Style` or use `<color>.normal().paint()`.
    pub fn paint<'a, S>(self, string: S) -> StyledString<'a>
        where S: Into<Cow<'a, str>>
    {
        StyledString { string: string.into(), style: self.normal() }
    }

    /// Returns a `Style` with the foreground colour set to this colour.
    pub fn normal(self) -> Style {
        Style { foreground: Some(self), ..Style::default() }
    }

    /// The same as `Color::normal()`, but also sets the background colour.
    pub fn on(self, background: Color) -> Style {
        Style { foreground: Some(self), background: Some(background), ..Style::default() }
    }

    /// Returns a `Style` with the 'bold' property set and the foreground colour
    /// set to this colour.
    pub fn bold(self) -> Style {
        Style { foreground: Some(self), bold: true, ..Style::default() }
    }

    /// Returns a `Style` with the 'dimmed' property set and the foreground
    /// colour set to this colour.
    pub fn dimmed(self) -> Style {
        Style { foreground: Some(self), dimmed: true, ..Style::default() }
    }

    /// Returns a `Style` with the 'italic' property set and the foreground
    /// colour set to this colour.
    pub fn italic(self) -> Style {
        Style { foreground: Some(self), italic: true, ..Style::default() }
    }

    /// Returns a `Style` with the 'underline' property set and the foreground
    /// colour set to this colour.
    pub fn underline(self) -> Style {
        Style { foreground: Some(self), underline: true, ..Style::default() }
    }

    /// Returns a `Style` with the 'blink' property set and the foreground
    /// colour set to this colour.
    pub fn blink(self) -> Style {
        Style { foreground: Some(self), blink: true, ..Style::default() }
    }

    /// Returns a `Style` with the 'reverse' property set and the foreground
    /// colour set to this colour.
    pub fn reverse(self) -> Style {
        Style { foreground: Some(self), reverse: true, ..Style::default() }
    }

    /// Returns a `Style` with the 'hidden' property set and the foreground
    /// colour set to this colour.
    pub fn hidden(self) -> Style {
        Style { foreground: Some(self), hidden: true, ..Style::default() }
    }

    fn write_foreground_code(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Black => write!(f, "30"),
            Red => write!(f, "31"),
            Green => write!(f, "32"),
            Yellow => write!(f, "33"),
            Blue => write!(f, "34"),
            Purple => write!(f, "35"),
            Cyan => write!(f, "36"),
            White => write!(f, "37"),
            Fixed(n) => write!(f, "38;5;{}", &n),
            Rgb(r, g, b) => write!(f, "38;2;{};{};{}", &r, &g, &b),
        }
    }

    fn write_background_code(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Black => write!(f, "40"),
            Red => write!(f, "41"),
            Green => write!(f, "42"),
            Yellow => write!(f, "43"),
            Blue => write!(f, "44"),
            Purple => write!(f, "45"),
            Cyan => write!(f, "46"),
            White => write!(f, "47"),
            Fixed(n) => write!(f, "48;5;{}", &n),
            Rgb(r, g, b) => write!(f, "48;2;{};{};{}", &r, &g, &b),
        }
    }
}

/// A collection of properties that are used to format a string.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    foreground: Option<Color>,
    background: Option<Color>,
    bold: bool,
    dimmed: bool,
    italic: bool,
    underline: bool,
    blink: bool,
    reverse: bool,
    hidden: bool,
}

impl Style {
    /// Creates a new `Style` without any formatting.
    pub fn new() -> Style {
        Style::default()
    }

    /// Applies the `Style` to a string, yielding a `StyledString`.
    pub fn paint<'a, S>(self, string: S) -> StyledString<'a>
        where S: Into<Cow<'a, str>>
    {
        StyledString { string: string.into(), style: self }
    }

    /// Sets the foreground to the given colour.
    pub fn foreground(&self, color: Color) -> Style {
        Style { foreground: Some(color), ..*self }
    }

    /// Sets the background to the given colour.
    pub fn background(&self, color: Color) -> Style {
        Style { background: Some(color), ..*self }
    }

    /// Applies the 'bold' property.
    pub fn bold(&self) -> Style {
        Style { bold: true, ..*self }
    }

    /// Applies the 'dimmed' property.
    pub fn dimmed(&self) -> Style {
        Style { dimmed: true, ..*self }
    }

    /// Applies the 'italic' property.
    pub fn italic(&self) -> Style {
        Style { italic: true, ..*self }
    }

    /// Applies the 'underline' property.
    pub fn underline(&self) -> Style {
        Style { underline: true, ..*self }
    }

    /// Applies the 'blink' property.
    pub fn blink(&self) -> Style {
        Style { blink: true, ..*self }
    }

    /// Applies the 'reverse' property.
    pub fn reverse(&self) -> Style {
        Style { reverse: true, ..*self }
    }

    /// Applies the 'hidden' property.
    pub fn hidden(&self) -> Style {
        Style { hidden: true, ..*self }
    }

    /// Returns true if this `Style` has no colours or properties set.
    fn is_plain(self) -> bool {
        self == Style::default()
    }

    /// Write any ANSI escape codes that go before the given text, such as
    /// colour or style codes.
    fn write_prefix(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::fmt::Write;

        if self.is_plain() {
            return Ok(());
        }

        try!(write!(f, "\x1b["));
        let mut written_anything = false;

        {
            let mut write_char = |c| {
                if written_anything {
                    try!(f.write_char(';'));
                }
                written_anything = true;
                try!(f.write_char(c));
                Ok(())
            };

            if self.bold { try!(write_char('1')); }
            if self.dimmed { try!(write_char('2')); }
            if self.italic { try!(write_char('3')); }
            if self.underline { try!(write_char('4')); }
            if self.blink { try!(write_char('5')); }
            if self.reverse { try!(write_char('6')); }
            if self.hidden { try!(write_char('7')); }
        }

        if let Some(fg) = self.foreground {
            if written_anything { try!(f.write_char(';')); }
            written_anything = true;

            try!(fg.write_foreground_code(f));
        }

        if let Some(bg) = self.background {
            if written_anything { try!(f.write_char(';')); }

            try!(bg.write_background_code(f));
        }

        try!(f.write_char('m'));
        Ok(())
    }

    /// Write any ANSI escape codes that go after the given text, typically the
    /// reset code.
    fn write_suffix(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_plain() {
            return Ok(());
        } else {
            write!(f, "\x1b[0m")
        }
    }
}

impl Default for Style {
    fn default() -> Style {
        Style {
            foreground: None,
            background: None,
            bold: false,
            dimmed: false,
            italic: false,
            underline: false,
            blink: false,
            reverse: false,
            hidden: false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use super::Color::*;

    // Convenience macro for creating test cases.
    macro_rules! test {
        ($name: ident: $style: expr; $input: expr => $result: expr) => {
            #[test]
            fn $name() {
                assert_eq!($style.paint($input).to_string(), $result.to_string())
            }
        }
    }

    test!(plain:             Style::default(); "TEST" => "TEST");
    test!(black:             Black;            "TEST" => "\x1b[30mTEST\x1b[0m");
    test!(black_background:  White.on(Black);  "TEST" => "\x1b[37;40mTEST\x1b[0m");
    test!(red:               Red;              "TEST" => "\x1b[31mTEST\x1b[0m");
    test!(red_background:    Black.on(Red);    "TEST" => "\x1b[30;41mTEST\x1b[0m");
    test!(green:             Green;            "TEST" => "\x1b[32mTEST\x1b[0m");
    test!(green_background:  Black.on(Green);  "TEST" => "\x1b[30;42mTEST\x1b[0m");
    test!(yellow:            Yellow;           "TEST" => "\x1b[33mTEST\x1b[0m");
    test!(yellow_background: Black.on(Yellow); "TEST" => "\x1b[30;43mTEST\x1b[0m");
    test!(blue:              Blue;             "TEST" => "\x1b[34mTEST\x1b[0m");
    test!(blue_background:   Black.on(Blue);   "TEST" => "\x1b[30;44mTEST\x1b[0m");
    test!(purple:            Purple;           "TEST" => "\x1b[35mTEST\x1b[0m");
    test!(purple_background: Black.on(Purple); "TEST" => "\x1b[30;45mTEST\x1b[0m");
    test!(cyan:              Cyan;             "TEST" => "\x1b[36mTEST\x1b[0m");
    test!(cyan_background:   Black.on(Cyan);   "TEST" => "\x1b[30;46mTEST\x1b[0m");
    test!(white:             White;            "TEST" => "\x1b[37mTEST\x1b[0m");
    test!(white_background:  Black.on(White);  "TEST" => "\x1b[30;47mTEST\x1b[0m");
    test!(fixed:             Fixed(220);       "TEST" => "\x1b[38;5;220mTEST\x1b[0m");
    test!(fixed_background:  Fixed(220).on(Fixed(245));
          "TEST" => "\x1b[38;5;220;48;5;245mTEST\x1b[0m");
    test!(rgb: Rgb(105, 245, 238);
          "TEST" => "\x1b[38;2;105;245;238mTEST\x1b[0m");
    test!(rgb_background: Black.on(Rgb(100, 245, 238));
          "TEST" => "\x1b[30;48;2;100;245;238mTEST\x1b[0m");

    test!(bold:      Style::new().bold();      "TEST" => "\x1b[1mTEST\x1b[0m");
    test!(dimmed:    Style::new().dimmed();    "TEST" => "\x1b[2mTEST\x1b[0m");
    test!(italic:    Style::new().italic();    "TEST" => "\x1b[3mTEST\x1b[0m");
    test!(underline: Style::new().underline(); "TEST" => "\x1b[4mTEST\x1b[0m");
    test!(blink:     Style::new().blink();     "TEST" => "\x1b[5mTEST\x1b[0m");
    test!(reverse:   Style::new().reverse();   "TEST" => "\x1b[6mTEST\x1b[0m");
    test!(hidden:    Style::new().hidden();    "TEST" => "\x1b[7mTEST\x1b[0m");
}
