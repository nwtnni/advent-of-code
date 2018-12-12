extern crate day_02;

use day_02::*;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let ribbon = INPUT.split_whitespace()
        .map(Present::parse)
        .map(Present::ribbon)
        .sum::<usize>();

    println!("{}", ribbon);
}
