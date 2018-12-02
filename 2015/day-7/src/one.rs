extern crate day_7;

use day_7::*;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let program = Program::parse(INPUT);
    let signals = program.run();
    println!("{}", signals["a"]);
}
