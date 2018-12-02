extern crate day_3;

use std::collections::HashSet as Set;

use day_3::*;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let mut seen: Set<Pos> = Set::default();
    let mut position = Pos::default();
    seen.insert(position);

    for dir in INPUT.chars().map(Dir::parse) {
        position.shift(dir);
        seen.insert(position);
    }
    
    println!("{}", seen.len());
}
