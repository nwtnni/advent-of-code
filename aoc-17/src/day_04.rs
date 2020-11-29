use std::collections::BTreeMap;
use std::collections::HashSet;
use std::str;

use aoc::*;

#[derive(Debug)]
pub struct HighEntropyPassphrases(Vec<String>);

impl str::FromStr for HighEntropyPassphrases {
    type Err = aoc::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        input
            .trim()
            .split('\n')
            .map(String::from)
            .collect::<Vec<_>>()
            .tap(HighEntropyPassphrases)
            .tap(Result::Ok)
    }
}

impl aoc::Solution for HighEntropyPassphrases {
    fn one(self) -> i64 {
        self.0
            .into_iter()
            .map(|phrases| {
                let mut seen = HashSet::new();
                for phrase in phrases.split_whitespace() {
                    if !seen.insert(phrase) {
                        return 0;
                    }
                }
                1
            })
            .sum()
    }

    fn two(self) -> i64 {
        self.0
            .into_iter()
            .map(|phrases| {
                let mut seen = HashSet::new();

                for phrase in phrases.split_whitespace() {
                    let mut chars = BTreeMap::new();
                    for char in phrase.chars() {
                        chars.entry(char)
                            .and_modify(|count| *count += 1)
                            .or_insert(1);
                    }
                    if !seen.insert(chars) {
                        return 0;
                    }
                }

                1
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use aoc::Solution;

    use super::*;

    #[test]
    fn test_p1_valid() {
        assert_eq!(HighEntropyPassphrases(vec![String::from("aa bb cc dd ee")]).one(), 1);
    }

    #[test]
    fn test_p1_invalid() {
        assert_eq!(HighEntropyPassphrases(vec![String::from("aa bb cc dd aa")]).one(), 0);
    }

    #[test]
    fn test_p1_different() {
        assert_eq!(HighEntropyPassphrases(vec![String::from("aa bb cc dd aaa")]).one(), 1);
    }

    #[test]
    fn test_p2_valid() {
        assert_eq!(HighEntropyPassphrases(vec![String::from("abcde fghij")]).two(), 1);
    }

    #[test]
    fn test_p2_invalid() {
        assert_eq!(HighEntropyPassphrases(vec![String::from("abcde xyz ecdab")]).two(), 0);
    }

    #[test]
    fn test_p2_hard_valid() {
        assert_eq!(HighEntropyPassphrases(vec![String::from("a ab abc abd abf abj")]).two(), 1);
    }

    #[test]
    fn test_p2_counter_valid() {
        assert_eq!(HighEntropyPassphrases(vec![String::from("iiii oiii ooii oooi oooo")]).two(), 1);
    }

    #[test]
    fn test_p2_counter_invalid() {
        assert_eq!(HighEntropyPassphrases(vec![String::from("oiii ioii iioi iiio")]).two(), 0);
    }
}
