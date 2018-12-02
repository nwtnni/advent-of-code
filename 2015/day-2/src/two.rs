extern crate day_2;

use day_2::*;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let ribbon = INPUT.split_whitespace()
        .map(Present::parse)
        .map(Present::ribbon)
        .sum::<usize>();

    println!("{}", ribbon);
}
