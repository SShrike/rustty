extern crate rustty;

use rustty::crayon::Style;
use rustty::crayon::Color::*;

fn main() {
    println!("{}", Red.on(Black).blink().paint("Hello world!"));
}
