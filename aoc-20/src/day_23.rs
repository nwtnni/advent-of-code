use std::collections::HashMap;
use std::collections::VecDeque;

use aoc::*;

#[derive(Clone, Debug)]
pub struct CrabCups(VecDeque<i64>);

impl Fro for CrabCups {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .chars()
            .map(|char| i64::fro(&char.to_string()))
            .collect::<VecDeque<_>>()
            .tap(Self)
    }
}

impl Solution for CrabCups {
    fn one(mut self) -> i64 {
        let min = self.0.iter().copied().min().unwrap();

        let max = self.0.iter().copied().max().unwrap();

        for _ in 0..100 {
            let current = self.0.pop_front().unwrap();
            let three = [
                self.0.pop_front().unwrap(),
                self.0.pop_front().unwrap(),
                self.0.pop_front().unwrap(),
            ];

            let mut target = current - 1;

            let index = loop {
                if let Some(index) = self.0.iter().copied().position(|label| label == target) {
                    break index;
                } else {
                    target -= 1;
                    if target < min {
                        target = max;
                    }
                }
            };

            self.0.insert(index + 1, three[2]);
            self.0.insert(index + 1, three[1]);
            self.0.insert(index + 1, three[0]);
            self.0.push_back(current);
        }

        let count = self.0.len();

        self.0
            .iter()
            .cycle()
            .skip_while(|label| **label != 1)
            .skip(1)
            .take(count - 1)
            .map(|label| label.to_string())
            .collect::<String>()
            .to::<i64>()
    }

    fn two(mut self) -> i64 {
        let max = self.0.iter().copied().max().unwrap();

        const MAX: i64 = 1_000_000;
        const ITERATIONS: usize = 10_000_000;

        self.0.extend(max + 1..=MAX as i64);

        let label_to_index = self
            .0
            .iter()
            .copied()
            .enumerate()
            .map(|(index, label)| (label, index))
            .collect::<HashMap<_, _>>();

        let mut index_to_index = vec![0usize; MAX as usize];
        let index_to_label = &self.0;

        for i in 0..MAX as usize {
            let curr = i;
            let next = (i + 1) % (MAX as usize);
            index_to_index[curr] = next;
        }

        let mut current_index = 0;
        let mut current_label = index_to_label[current_index];

        //          +-----+     +-----+     +-----+     +-----+     +-----+     +-----+
        //          |     v     |     v     |     v     |     v     |     v     |     v
        // |           |           |           |           |    ...    |           |           |
        //       ^           ^            ^          ^                       ^
        //    current      first       second      third                  target
        for _ in 0..ITERATIONS {
            let first_index = index_to_index[current_index];
            let first_label = index_to_label[first_index];

            let second_index = index_to_index[first_index];
            let second_label = index_to_label[second_index];

            let third_index = index_to_index[second_index];
            let third_label = index_to_label[third_index];

            let mut target_label = match (current_label - 1).rem_euclid(MAX) {
                0 => MAX,
                label => label,
            };

            while [first_label, second_label, third_label].contains(&target_label) {
                target_label = match (target_label - 1).rem_euclid(MAX) {
                    0 => MAX,
                    label => label,
                };
            }

            let target_index = label_to_index[&target_label];

            //                      +-----+     +-----+     +-----------------------------+
            //                      |     |     |     |     |                             |
            //          +-----------|-----|-----|-----|-----|-----+                       +
            //          |           |     v     |     v     |     v                       v
            // |           |           |           |           |    ...    |           |           |
            //       ^        ^  ^            ^          ^                       ^   |
            //    current     | first      second      third                  target |
            //                |                                                      |
            //                +------------------------------------------------------+
            //
            //
            index_to_index[current_index] = index_to_index[third_index];
            index_to_index[third_index] = index_to_index[target_index];
            index_to_index[target_index] = first_index;

            current_index = index_to_index[current_index];
            current_label = index_to_label[current_index];
        }

        while current_label != 1 {
            current_index = index_to_index[current_index];
            current_label = index_to_label[current_index];
        }

        let first_index = index_to_index[current_index];
        let second_index = index_to_index[first_index];

        index_to_label[first_index] * index_to_label[second_index]
    }
}
