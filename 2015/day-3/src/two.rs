extern crate day_3;

use std::collections::HashSet as Set;

use day_3::*;

const INPUT: &'static str = include_str!("input.txt");

fn main() {

    let mut seen: Set<Pos> = Set::default();
    let mut santa = Pos::default();
    let mut robot = Pos::default();
    let mut move_santa = true;

    seen.insert(santa);

    for dir in INPUT.chars().map(Dir::parse) {
        if move_santa {
            santa.shift(dir);
            seen.insert(santa);
        } else {
            robot.shift(dir);
            seen.insert(robot);
        }
        move_santa = !move_santa;
    }

    println!("{}", seen.len());
}
