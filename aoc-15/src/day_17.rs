use std::collections::HashMap;
use std::iter;

use aoc::*;

#[derive(Clone, Debug)]
pub struct NoSuchThingAsTooMuch(Vec<i64>);

impl Fro for NoSuchThingAsTooMuch {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(i64::fro)
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

fn recurse<'a>(containers: &'a [i64], total: i64, used: i64) -> Box<dyn Iterator<Item = i64> + 'a> {
    match containers {
        [] if total > 0 => Box::new(iter::empty()),
        [] => Box::new(iter::once(used)),
        [container, containers @ ..] => {
            if *container <= total {
                Box::new(
                    recurse(containers, total - container, used + 1)
                        .chain(recurse(containers, total, used)),
                )
            } else {
                recurse(containers, total, used)
            }
        }
    }
}

impl Solution for NoSuchThingAsTooMuch {
    fn one(self) -> i64 {
        recurse(&self.0, 150, 0).count() as i64
    }

    fn two(self) -> i64 {
        let mut counter = HashMap::new();
        for used in recurse(&self.0, 150, 0) {
            *counter.entry(used).or_insert(0) += 1;
        }
        counter
            .iter()
            .min_by_key(|(used, _)| *used)
            .map(|(_, count)| *count)
            .unwrap()
    }
}
