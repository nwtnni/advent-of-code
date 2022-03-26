use std::collections::HashMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct AdapterArray(Vec<i64>);

impl Fro for AdapterArray {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(i64::fro)
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for AdapterArray {
    fn one(mut self) -> i64 {
        self.0.push(0);
        self.0.push(self.0.iter().max().unwrap() + 3);
        self.0.sort();

        let mut one = 0;
        let mut three = 0;

        for window in self.0.windows(2) {
            if window[1] - window[0] == 1 {
                one += 1;
            }
            if window[1] - window[0] == 3 {
                three += 1;
            }
        }

        (one * three) as i64
    }

    fn two(mut self) -> i64 {
        self.0.sort();
        let mut memo = HashMap::new();
        recurse(0, &self.0, &mut memo)
    }
}

fn recurse<'a>(start: i64, adapters: &'a [i64], memo: &mut HashMap<&'a [i64], i64>) -> i64 {
    if adapters.is_empty() {
        return 1;
    }

    if let Some(count) = memo.get(adapters) {
        return *count;
    }

    let total = adapters
        .iter()
        .enumerate()
        .take_while(|(_, adapter)| **adapter <= start + 3)
        .map(|(index, adapter)| recurse(*adapter, &adapters[index + 1..], memo))
        .sum::<i64>();

    memo.insert(adapters, total);
    total
}
