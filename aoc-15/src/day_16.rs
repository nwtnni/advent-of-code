use std::collections::HashMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct AuntSue(Vec<HashMap<String, i64>>);

impl Fro for AuntSue {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .filter_map(|line| {
                let (_, traits) = line.split_once(": ")?;
                traits
                    .split(", ")
                    .filter_map(|r#trait| r#trait.split_once(": "))
                    .map(|(r#trait, count)| (String::from(r#trait), i64::fro(count)))
                    .collect::<HashMap<_, _>>()
                    .tap(Option::Some)
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

fn needle() -> HashMap<String, i64> {
    let mut needle = HashMap::new();
    needle.insert(String::from("children"), 3);
    needle.insert(String::from("cats"), 7);
    needle.insert(String::from("samoyeds"), 2);
    needle.insert(String::from("pomeranians"), 3);
    needle.insert(String::from("akitas"), 0);
    needle.insert(String::from("vizslas"), 0);
    needle.insert(String::from("goldfish"), 5);
    needle.insert(String::from("trees"), 3);
    needle.insert(String::from("cars"), 2);
    needle.insert(String::from("perfumes"), 1);
    needle
}

impl Solution for AuntSue {
    fn one(self) -> i64 {
        let needle = needle();
        self.0
            .iter()
            .enumerate()
            .filter(|(_, traits)| traits.iter().all(|(key, value)| needle[key] == *value))
            .next()
            .map(|(index, _)| index as i64 + 1)
            .unwrap()
    }

    fn two(self) -> i64 {
        let needle = needle();
        self.0
            .iter()
            .enumerate()
            .filter(|(_, traits)| {
                traits.iter().all(|(key, value)| match key.as_str() {
                    "cats" | "trees" => *value > needle[key],
                    "pomeranians" | "goldfish" => *value < needle[key],
                    _ => *value == needle[key],
                })
            })
            .next()
            .map(|(index, _)| index as i64 + 1)
            .unwrap()
    }
}
