extern crate tutil;

use tutil::crayon::Style;
use tutil::crayon::Color::*;

use tutil::screen;

fn main() {
    println!("{}", Red.on(Black).blink().paint("Hello world!"));
    
    let size = screen::size().unwrap();
    println!("The screen size is {}x{}.", size.0, size.1);
}
