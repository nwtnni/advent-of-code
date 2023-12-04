use std::collections::HashMap;
use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct GearRatios {
    symbols: HashMap<(usize, usize), u8>,
    unique: HashMap<(usize, usize), String>,
    all: HashMap<(usize, usize), (usize, usize, String)>,
}

impl Fro for GearRatios {
    fn fro(input: &str) -> Self {
        let mut symbols = HashMap::new();
        let mut unique = HashMap::new();

        input.trim().split('\n').enumerate().for_each(|(i, line)| {
            let mut j = 0;
            while j < line.len() {
                match line.as_bytes()[j] {
                    b'.' => j += 1,
                    b'0'..=b'9' => {
                        let mut dj = 1;

                        loop {
                            match j + dj + 1 > line.len()
                                || line[j..][..dj + 1].parse::<i64>().is_err()
                            {
                                true => break,
                                false => dj += 1,
                            }
                        }

                        unique.insert((i, j), String::from(&line[j..][..dj]));
                        j += dj;
                    }
                    other => {
                        symbols.insert((i, j), other);
                        j += 1;
                    }
                }
            }
        });

        let mut all = HashMap::new();
        for ((i, j), number) in unique.iter() {
            for dj in 0..number.len() {
                all.insert((*i, j + dj), (*i, *j, number.clone()));
            }
        }

        Self {
            symbols,
            unique,
            all,
        }
    }
}

impl Solution for GearRatios {
    fn one(self) -> i64 {
        self.unique
            .iter()
            .filter(|((i, j), n)| {
                if self.symbols.contains_key(&(*i, j.wrapping_sub(1)))
                    || self.symbols.contains_key(&(*i, j + n.len()))
                {
                    dbg!(n);
                    return true;
                }

                for j in j.saturating_sub(1)..=j + n.len() {
                    if self.symbols.contains_key(&(i.wrapping_sub(1), j))
                        || self.symbols.contains_key(&(i + 1, j))
                    {
                        dbg!(n);
                        return true;
                    }
                }

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

                let l = self.all.get(&(i.wrapping_sub(1), *j));
                let r = self.all.get(&(i.wrapping_add(1), *j));
                let u = self.all.get(&(*i, j.wrapping_sub(1)));
                let d = self.all.get(&(*i, j.wrapping_add(1)));

                let lu = self.all.get(&(i.wrapping_sub(1), j.wrapping_sub(1)));
                let ld = self.all.get(&(i.wrapping_sub(1), j.wrapping_add(1)));
                let ru = self.all.get(&(i.wrapping_add(1), j.wrapping_sub(1)));
                let rd = self.all.get(&(i.wrapping_add(1), j.wrapping_add(1)));

                let around = [l, r, u, d, lu, ld, ru, rd]
                    .into_iter()
                    .flatten()
                    .collect::<HashSet<_>>();

                if around.len() != 2 {
                    return None;
                }

                Some(
                    around
                        .into_iter()
                        .map(|(_, _, n)| n.parse::<i64>().unwrap())
                        .product::<i64>(),
                )
            })
            .sum()
    }
}
