use std::str;
use std::collections::HashMap as Map;

#[derive(Debug)]
pub struct DoesntHeHaveInternElvesForThis(String);

impl str::FromStr for DoesntHeHaveInternElvesForThis {
    type Err = aoc::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DoesntHeHaveInternElvesForThis(s.to_owned()))
    }
}

fn is_vowel(c: &&char) -> bool {
    match c {
    | 'a' | 'e' | 'i' | 'o' | 'u' => true,
    | _ => false,
    }
}

fn is_repeated(cs: &&[char]) -> bool {
    cs[0] == cs[1]
}

fn is_sandwich(cs: &&[char]) -> bool {
    cs[0] == cs[2]
}

fn is_naughty(cs: &&[char]) -> bool {
    match cs {
    | ['a', 'b'] | ['c', 'd'] | ['p', 'q'] | ['x', 'y'] => true,
    | _ => false,
    }
}

impl aoc::Solution for DoesntHeHaveInternElvesForThis {
    fn one(self) -> i32 {
        self.0
            .split_whitespace()
            .filter(|string| {
                let chars = string.chars().collect::<Vec<_>>();
                let vowels = chars.iter().filter(is_vowel).count();
                let twice = chars.windows(2).filter(is_repeated).count();
                let naughty = chars.windows(2).filter(is_naughty).count();
                vowels >= 3 && twice >= 1 && naughty == 0
            })
            .count()
            as i32
    }

    fn two(self) -> i32 {
        self.0
            .split_whitespace()
            .filter(|string| {
                let chars = string.chars().collect::<Vec<_>>();
                let sandwich = chars.windows(3).filter(is_sandwich).count();
                let (_, repeat) = chars
                    .windows(2)
                    .enumerate()
                    .fold((Map::<&[char], usize>::default(), 0), |(mut seen, count), (i, cs)| {
                        match seen.get(cs) {
                        | Some(j) if i > j + 1 => { (seen, count + 1) }
                        | Some(_)              => { (seen, count) }
                        | None                 => { seen.insert(cs, i); (seen, count) }
                        }
                    });
                sandwich > 0 && repeat > 0
            })
            .count()
            as i32
    }
}
