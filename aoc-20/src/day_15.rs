use std::collections::HashMap;
use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct RambunctiousRecitation(Vec<i64>);

impl Fro for RambunctiousRecitation {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split(',')
            .map(i64::fro)
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for RambunctiousRecitation {
    fn one(self) -> i64 {
        let mut spoken = HashMap::new();
        let mut last = 0;

        for i in 0..self.0.len() {
            spoken.insert(self.0[i], (vec![i as i64], 1));
            last = self.0[i];
        }

        for i in self.0.len()..2020 {
            match spoken.get(&last) {
            | Some((_, 1)) => {
                last = 0;
            }
            | Some((indices, _)) => {
                last = (i - 1) as i64 - indices[indices.len() - 2];
            }
            | None => {
                unreachable!()
            }
            }

            spoken
                .entry(last)
                .and_modify(|(index, count)| {
                    index.push(i as i64);
                    *count += 1;
                })
                .or_insert((vec![i as i64], 1));

            dbg!(last);
        }

        last
    }

    fn two(self) -> i64 {
        let mut spoken = HashMap::new();
        let mut last = 0;

        for i in 0..self.0.len() {
            spoken.insert(self.0[i], (vec![i as i64], 1));
            last = self.0[i];
        }

        for i in self.0.len()..30000000 {
            match spoken.get(&last) {
            | Some((_, 1)) => {
                last = 0;
            }
            | Some((indices, _)) => {
                last = (i - 1) as i64 - indices[indices.len() - 2];
            }
            | None => {
                unreachable!()
            }
            }

            spoken
                .entry(last)
                .and_modify(|(index, count)| {
                    index.push(i as i64);
                    *count += 1;
                })
                .or_insert((vec![i as i64], 1));
        }

        last
    }
}
