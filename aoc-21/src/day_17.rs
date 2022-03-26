use std::cmp;

use aoc::*;

#[derive(Copy, Clone, Debug)]
pub struct TrickShot {
    x1: i64,
    x2: i64,
    y1: i64,
    y2: i64,
}

impl Fro for TrickShot {
    fn fro(input: &str) -> Self {
        let (a, b) = input
            .trim()
            .trim_start_matches("target area: ")
            .split_once(", ")
            .unwrap();

        let (x1, x2) = a.trim().trim_start_matches("x=").split_once("..").unwrap();
        let (y1, y2) = b.trim().trim_start_matches("y=").split_once("..").unwrap();

        // let x1 = cmp::min(x1, x2);
        // let x2 = cmp::max(x1, x2);
        // let y1 = cmp::min(y1, y2);
        // let y2 = cmp::max(y1, y2);

        Self {
            x1: i64::fro(x1),
            x2: i64::fro(x2),
            y1: i64::fro(y1),
            y2: i64::fro(y2),
        }
    }
}

impl Solution for TrickShot {
    fn one(self) -> i64 {
        let mut max = i64::MIN;

        for dx in 0..=self.x2 {
            for dy in -1000..=1000 {
                let mut probe = Probe { x: 0, y: 0, dx, dy };
                let mut height = i64::MIN;

                while probe.x < self.x1 && probe.y > self.y2 {
                    height = cmp::max(probe.y, height);
                    probe.next();
                }

                while probe.x <= self.x2 && probe.y >= self.y1 {
                    height = cmp::max(probe.y, height);
                    if probe.x >= self.x1
                        && probe.x <= self.x2
                        && probe.y >= self.y1
                        && probe.y <= self.y2
                    {
                        max = cmp::max(height, max);
                    }
                    probe.next();
                }
            }
        }

        max
    }

    fn two(self) -> i64 {
        let mut count = 0;

        for dx in 0..=self.x2 {
            for dy in -1000..=1000 {
                let mut probe = Probe { x: 0, y: 0, dx, dy };
                let mut height = i64::MIN;

                while probe.x < self.x1 && probe.y > self.y2 {
                    height = cmp::max(probe.y, height);
                    probe.next();
                }

                while probe.x <= self.x2 && probe.y >= self.y1 {
                    height = cmp::max(probe.y, height);
                    if probe.x >= self.x1
                        && probe.x <= self.x2
                        && probe.y >= self.y1
                        && probe.y <= self.y2
                    {
                        count += 1;
                        break;
                    }
                    probe.next();
                }
            }
        }

        count
    }
}

#[derive(Clone, Debug)]
struct Probe {
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
}

impl Probe {
    fn next(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
        self.dx += match self.dx.cmp(&0) {
            cmp::Ordering::Less => 1,
            cmp::Ordering::Equal => 0,
            cmp::Ordering::Greater => -1,
        };
        self.dy -= 1;
    }
}
