use std::usize;

const INPUT: &'static str = include_str!("input.txt");

fn react(mut polymer: Vec<&char>) -> usize {
    let mut change = true;
    while change {
        change = false;
        for i in 0..polymer.len() - 1 {
            let a = *polymer[i];
            let b = *polymer[i + 1];
            if (a.to_ascii_uppercase() == b || a.to_ascii_lowercase() == b) && a != b {
                polymer.remove(i);
                polymer.remove(i);
                change = true;
                break
            }
        }
    }
    polymer.len()
}

fn main() {
    
    let polymer = INPUT.trim()
        .chars()
        .collect::<Vec<_>>();

    let mut min = usize::max_value();

    for c in "abcdefghijklmnopqrstuvwxyz".chars() {
        let cc = c.to_ascii_uppercase();
        let polymer = polymer.iter()
            .filter(|x| **x != c && **x != cc)
            .collect::<Vec<_>>();
        let remaining = react(polymer);
        min = usize::min(min, remaining);
    }

    println!("{}", min);

}
