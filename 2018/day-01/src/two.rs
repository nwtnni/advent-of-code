use std::str::FromStr;
use std::collections::HashSet as Set;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let frequencies = INPUT.split_whitespace()
        .map(isize::from_str)
        .filter_map(Result::ok)
        .cycle();

    let mut seen = Set::new();
    let mut current = 0;

    for frequency in frequencies {
        current += frequency; 
        if seen.contains(&current) {
            break
        } else {
            seen.insert(current);
        }
    }

    println!("{}", current);
}
