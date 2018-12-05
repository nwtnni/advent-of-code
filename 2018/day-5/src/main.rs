use std::usize;

const INPUT: &'static str = include_str!("input.txt");

fn react(mut polymer: Vec<&char>) -> usize {
    let mut change = Some(0);
    while change.is_some() {
        let start = change.take().unwrap();
        for i in start..polymer.len() - 1 {
            let a = *polymer[i];
            let b = *polymer[i + 1];
            if (a.to_ascii_uppercase() == b || a.to_ascii_lowercase() == b) && a != b {
                change = Some(if i == 0 { 0 } else { i - 1 });
                polymer.remove(i);
                polymer.remove(i);
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
