extern crate tutil;

use tutil::crayon::Style;
use tutil::crayon::Color::*;

fn main() {
    println!("{}", Red.on(Black).blink().paint("Hello world!"));
}
