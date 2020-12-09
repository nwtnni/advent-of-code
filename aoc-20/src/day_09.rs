use std::collections::HashSet;
use std::collections::VecDeque;

use aoc::*;

#[derive(Clone)]
pub struct EncodingError(Vec<i64>);

impl Fro for EncodingError {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(i64::fro)
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for EncodingError {
    fn one(self) -> i64 {
        let mut set = HashSet::<i64>::new();
        let mut queue = VecDeque::new();

        for i in 0..25 {
            queue.push_back(self.0[i]);
            set.insert(self.0[i]);
        }

        'outer: for i in 25..self.0.len() {

            let search = self.0[i];

            for item in &queue {
                if set.contains(&(search - item)) {
                    set.remove(&queue.pop_front().unwrap());
                    set.insert(search);
                    queue.push_back(search);
                    continue 'outer;
                }
            }

            return search;
        }

        unreachable!()
    }

    fn two(self) -> i64 {
        let sum = self.clone().one();
        for i in 0..self.0.len() {
            for j in i + 2..self.0.len() {
                if self.0[i..j].iter().sum::<i64>() == sum {
                    let min = self.0[i..j].iter().min().unwrap();
                    let max = self.0[i..j].iter().max().unwrap();
                    return min + max;
                }
            }
        }
        unreachable!()
    }
}
