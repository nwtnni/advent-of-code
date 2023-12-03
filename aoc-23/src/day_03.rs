use std::collections::HashMap;
use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct GearRatios {
    symbols: HashMap<(usize, usize), u8>,
    digits: HashMap<(usize, usize), String>,
    original: HashMap<(usize, usize), String>,
}

impl Fro for GearRatios {
    fn fro(input: &str) -> Self {
        let mut symbols = HashMap::new();
        let mut digits = HashMap::new();
        input
            .trim()
            .split('\n')
            .enumerate()
            .for_each(|(line_index, line)| {
                let mut i = 0;
                while i < line.as_bytes().len() {
                    match line.as_bytes()[i] {
                        b'.' => i += 1,
                        b'0'..=b'9' => {
                            let mut j = i + 1;
                            while j <= line.as_bytes().len() && line[i..j].parse::<i64>().is_ok() {
                                j += 1;
                            }
                            digits.insert((line_index, i), dbg!(String::from(&line[i..j - 1])));
                            i += j - 1 - i;
                        }
                        other => {
                            dbg!(((line_index, i), other));
                            symbols.insert((line_index, i), other);
                            i += 1;
                        }
                    }
                }
            });

        let mut original = HashMap::new();
        input
            .trim()
            .split('\n')
            .enumerate()
            .for_each(|(line_index, line)| {
                let mut i = 0;
                while i < line.as_bytes().len() {
                    match line.as_bytes()[i] {
                        b'0'..=b'9' => {
                            let mut j = i + 1;
                            while j <= line.as_bytes().len() && line[i..j].parse::<i64>().is_ok() {
                                j += 1;
                            }
                            let digit = String::from(&line[i..j - 1]);
                            for j in i..j - 1 {
                                dbg!((line_index, j), &digit);
                                original.insert((line_index, j), digit.clone());
                            }
                            i += j - 1 - i;
                        }
                        _ => {
                            i += 1;
                        }
                    }
                }
            });

        Self {
            symbols,
            digits,
            original,
        }
    }
}

impl Solution for GearRatios {
    fn one(self) -> i64 {
        self.digits
            .iter()
            .filter(|((i, j), n)| {
                if j.checked_sub(1)
                    .filter(|j| self.symbols.contains_key(&(*i, *j)))
                    .is_some()
                    || self.symbols.contains_key(&(*i, j + n.as_bytes().len()))
                {
                    dbg!(n);
                    return true;
                }

                for j in j.saturating_sub(1)..j + n.as_bytes().len() + 1 {
                    if i.checked_sub(1)
                        .filter(|i| self.symbols.contains_key(&(*i, j)))
                        .is_some()
                        || self.symbols.contains_key(&(i + 1, j))
                    {
                        dbg!(n);
                        return true;
                    }
                }

                dbg!(n);
                false
            })
            .map(|(_, n)| n.parse::<i64>().unwrap())
            .sum()
    }

    fn two(self) -> i64 {
        self.symbols
            .iter()
            .filter_map(|((i, j), s)| {
                if *s != b'*' {
                    return None;
                }

                let lu = self.original.get(&(i.wrapping_sub(1), j.wrapping_sub(1)));
                let ld = self.original.get(&(i.wrapping_sub(1), j.wrapping_add(1)));
                let ru = self.original.get(&(i.wrapping_add(1), j.wrapping_sub(1)));
                let rd = self.original.get(&(i.wrapping_add(1), j.wrapping_add(1)));

                let l = self.original.get(&(i.wrapping_sub(1), *j));
                let r = self.original.get(&(i.wrapping_add(1), *j));
                let u = self.original.get(&(*i, j.wrapping_sub(1)));
                let d = self.original.get(&(*i, j.wrapping_add(1)));

                if [l, r, u, d, lu, ld, ru, rd]
                    .into_iter()
                    .flatten()
                    .collect::<HashSet<_>>()
                    .len()
                    == 2
                {
                    dbg!((i, j));
                    Some(
                        [l, r, u, d, lu, ld, ru, rd]
                            .into_iter()
                            .flatten()
                            .collect::<HashSet<_>>()
                            .into_iter()
                            .map(|n| n.parse::<i64>().unwrap())
                            .product::<i64>(),
                    )
                } else {
                    None
                }
            })
            .sum()
    }
}
