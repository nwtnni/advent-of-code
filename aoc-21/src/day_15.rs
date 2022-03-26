use std::cmp;
use std::collections::HashSet;

use aoc::*;
use priority_queue::PriorityQueue;

#[derive(Clone, Debug)]
pub struct Chiton(Grid<i64>);

impl Fro for Chiton {
    fn fro(input: &str) -> Self {
        Self(Grid::fro(input))
    }
}

impl Solution for Chiton {
    fn one(self) -> i64 {
        let h = self.0.height();
        let w = self.0.width();
        let s = (0, 0);

        let mut seen = HashSet::new();
        let mut queue = PriorityQueue::new();
        queue.push(s, cmp::Reverse(0));

        while let Some(((i, j), cmp::Reverse(score))) = queue.pop() {
            if (i, j) == (h - 1, w - 1) {
                return score;
            }

            seen.insert((i, j));

            for (i, j) in adjacent(h, w, i, j) {
                if seen.contains(&(i, j)) {
                    continue;
                }

                queue.push_increase((i, j), cmp::Reverse(score + self.0[(i, j)]));
            }
        }

        unreachable!()
    }

    fn two(self) -> i64 {
        let h = self.0.height();
        let w = self.0.width();
        let s = (0, 0);

        let heuristic =
            |(i, j): (usize, usize), score: i64| (h * 5 - 1 - i) + (w * 5 - 1 - j) + score as usize;

        let mut seen = HashSet::new();
        let mut queue = PriorityQueue::new();
        queue.push((s, 0), cmp::Reverse(heuristic(s, 0)));

        while let Some((((i, j), score), _)) = queue.pop() {
            if (i, j) == (h * 5 - 1, w * 5 - 1) {
                return score;
            }

            seen.insert((i, j));

            for (i, j) in adjacent(h * 5, w * 5, i, j) {
                if seen.contains(&(i, j)) {
                    continue;
                }

                let delta = (i / h + j / w) as i64;
                let score = score
                    + match self.0[(i % h, j % w)] + delta {
                        x @ 1..=9 => x,
                        x => x - 9,
                    };

                queue.push_increase(((i, j), score), cmp::Reverse(heuristic((i, j), score)));
            }
        }

        unreachable!()
    }
}
