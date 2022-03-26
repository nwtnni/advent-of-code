use std::collections::HashMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct DoesntHeHaveInternElvesForThis(String);

impl Fro for DoesntHeHaveInternElvesForThis {
    fn fro(input: &str) -> Self {
        Self(input.to_owned())
    }
}

fn is_vowel(c: &&char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}

fn is_repeated(cs: &&[char]) -> bool {
    cs[0] == cs[1]
}

fn is_sandwich(cs: &&[char]) -> bool {
    cs[0] == cs[2]
}

fn is_naughty(cs: &&[char]) -> bool {
    matches!(cs, ['a', 'b'] | ['c', 'd'] | ['p', 'q'] | ['x', 'y'])
}

impl Solution for DoesntHeHaveInternElvesForThis {
    fn one(self) -> i64 {
        self.0
            .split_whitespace()
            .filter(|string| {
                let chars = string.chars().collect::<Vec<_>>();
                let vowels = chars.iter().filter(is_vowel).count();
                let twice = chars.windows(2).filter(is_repeated).count();
                let naughty = chars.windows(2).filter(is_naughty).count();
                vowels >= 3 && twice >= 1 && naughty == 0
            })
            .count() as i64
    }

    fn two(self) -> i64 {
        self.0
            .split_whitespace()
            .filter(|string| {
                let chars = string.chars().collect::<Vec<_>>();
                let sandwich = chars.windows(3).filter(is_sandwich).count();
                let (_, repeat) = chars.windows(2).enumerate().fold(
                    (HashMap::<&[char], usize>::default(), 0),
                    |(mut seen, count), (i, cs)| {
                        if i > *seen.entry(cs).or_insert(i) + 1 {
                            (seen, count + 1)
                        } else {
                            (seen, count)
                        }
                    },
                );
                sandwich > 0 && repeat > 0
            })
            .count() as i64
    }
}
