extern crate tutil;

use tutil::crayon::Style;
use tutil::crayon::Color::*;

use tutil::screen;

fn main() {
    println!("{}", Red.on(Black).blink().paint("Hello world!"));
    println!("{:?}", screen::size());
}
