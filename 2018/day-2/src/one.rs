use std::collections::HashMap as Map;

const INPUT: &'static str = include_str!("input.txt");

fn main() {

    let mut twos = 0;
    let mut threes = 0;

    for id in INPUT.trim().split_whitespace() {
        
        let mut counts: Map<char, usize> = Map::default();
        for c in id.chars() {
            counts.entry(c)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
        
        if counts.values().any(|count| *count == 2) { twos += 1 }
        if counts.values().any(|count| *count == 3) { threes += 1 }
    }

    println!("{}", twos * threes);
}
