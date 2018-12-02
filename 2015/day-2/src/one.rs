const INPUT: &'static str = include_str!("input.txt");

extern crate day_2;

use day_2::*;

fn main() {
    let area = INPUT.split_whitespace()
        .map(Present::parse)
        .map(Present::wrapping)
        .sum::<usize>();

    println!("{}", area);
}
