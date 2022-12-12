use std::cmp;
use std::collections::VecDeque;

use aoc::*;

#[derive(Clone, Debug)]
pub struct HillClimbingAlgorithm(Vec<Vec<u8>>);

impl Fro for HillClimbingAlgorithm {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| line.bytes().collect())
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for HillClimbingAlgorithm {
    fn one(mut self) -> i64 {
        // let mut queue = PriorityQueue::new();
        // let mut seen = HashSet::new();

        let mut start = (0, 0);
        let mut finish = (0, 0);

        for i in 0..self.0.len() {
            for j in 0..self.0[0].len() {
                if self.0[i][j] == b'S' {
                    self.0[i][j] = b'a';
                    start = (i, j);
                }

                if self.0[i][j] == b'E' {
                    self.0[i][j] = b'z';
                    finish = (i, j);
                }
            }
        }

        // queue.push((0, start), cmp::Reverse((b'z' - b'a') as i64));
        // queue.push(
        //     (0, start),
        //     cmp::Reverse(cmp::min((b'z' - b'a') as i64, dist(start, finish))),
        // );

        // while let Some(((steps, (i, j)), cmp::Reverse(_))) = queue.pop() {
        //     dbg!(steps);
        //     if (i, j) == finish {
        //         return steps;
        //     }

        //     seen.insert((i, j));

        //     for (y, x) in [
        //         if i < self.0.len() - 1 {
        //             Some((i + 1, j))
        //         } else {
        //             None
        //         },
        //         if i > 0 { Some((i - 1, j)) } else { None },
        //         if j < self.0[0].len() - 1 {
        //             Some((i, j + 1))
        //         } else {
        //             None
        //         },
        //         if j > 0 { Some((i, j - 1)) } else { None },
        //     ]
        //     .into_iter()
        //     .flatten()
        //     {
        //         if seen.contains(&(y, x)) {
        //             continue;
        //         }

        //         if self.0[i][j] + 1 >= self.0[y][x] {
        //             queue.push_increase(
        //                 (steps + 1, (y, x)),
        //                 // cmp::Reverse(steps + (b'z' - self.0[y][x]) as i64),
        //                 cmp::Reverse(cmp::min(
        //                     steps + (b'z' - self.0[y][x]) as i64,
        //                     steps + dist((y, x), finish),
        //                 )),
        //             );
        //         }
        //     }
        // }

        // let mut seen = HashSet::new();
        let mut flood = VecDeque::new();
        let mut distance = vec![vec![i64::MAX; self.0[0].len()]; self.0.len()];

        flood.push_back(finish);
        distance[finish.0][finish.1] = 0;

        while let Some((i, j)) = flood.pop_front() {
            if start == (i, j) {
                return distance[i][j];
            }

            for (y, x) in [
                if i < self.0.len() - 1 {
                    Some((i + 1, j))
                } else {
                    None
                },
                if i > 0 { Some((i - 1, j)) } else { None },
                if j < self.0[0].len() - 1 {
                    Some((i, j + 1))
                } else {
                    None
                },
                if j > 0 { Some((i, j - 1)) } else { None },
            ]
            .into_iter()
            .flatten()
            {
                if self.0[y][x] + 1 >= self.0[i][j] {
                    println!(
                        "Setting {},{} to {}",
                        y,
                        x,
                        cmp::min(distance[y][x], distance[i][j].saturating_add(1))
                    );
                    if distance[i][j].saturating_add(1) < distance[y][x] {
                        distance[y][x] = distance[i][j].saturating_add(1);
                        flood.push_back((y, x));
                    }
                    // distance[y][x] = cmp::min(distance[y][x], distance[i][j].saturating_add(1));
                }

                // if !seen.contains(&(y, x)) {
                //     flood.push_back((y, x));
                // }
            }
        }

        unreachable!()
    }

    fn two(mut self) -> i64 {
        let mut start = (0, 0);
        let mut finish = (0, 0);

        for i in 0..self.0.len() {
            for j in 0..self.0[0].len() {
                if self.0[i][j] == b'S' {
                    self.0[i][j] = b'a';
                    start = (i, j);
                }

                if self.0[i][j] == b'E' {
                    self.0[i][j] = b'z';
                    finish = (i, j);
                }
            }
        }
        let mut flood = VecDeque::new();
        let mut distance = vec![vec![i64::MAX; self.0[0].len()]; self.0.len()];

        flood.push_back(finish);
        distance[finish.0][finish.1] = 0;

        while let Some((i, j)) = flood.pop_front() {
            for (y, x) in [
                if i < self.0.len() - 1 {
                    Some((i + 1, j))
                } else {
                    None
                },
                if i > 0 { Some((i - 1, j)) } else { None },
                if j < self.0[0].len() - 1 {
                    Some((i, j + 1))
                } else {
                    None
                },
                if j > 0 { Some((i, j - 1)) } else { None },
            ]
            .into_iter()
            .flatten()
            {
                if self.0[y][x] + 1 >= self.0[i][j] {
                    println!(
                        "Setting {},{} to {}",
                        y,
                        x,
                        cmp::min(distance[y][x], distance[i][j].saturating_add(1))
                    );
                    if distance[i][j].saturating_add(1) < distance[y][x] {
                        distance[y][x] = distance[i][j].saturating_add(1);
                        flood.push_back((y, x));
                    }
                    // distance[y][x] = cmp::min(distance[y][x], distance[i][j].saturating_add(1));
                }

                // if !seen.contains(&(y, x)) {
                //     flood.push_back((y, x));
                // }
            }
        }

        let mut min = i64::MAX;

        for i in 0..self.0.len() {
            for j in 0..self.0[0].len() {
                if self.0[i][j] == b'a' {
                    min = cmp::min(distance[i][j], min);
                }
            }
        }
        min
    }
}

fn dist((i, j): (usize, usize), (y, x): (usize, usize)) -> i64 {
    (cmp::max(i, y) - cmp::min(i, y) + cmp::max(j, x) - cmp::min(j, x)) as i64
}
