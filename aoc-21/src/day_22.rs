use std::cmp;
use std::collections::HashSet;
use std::iter;

use aoc::*;

#[derive(Clone, Debug)]
pub struct ReactorReboot(Vec<(bool, i64, i64, i64, i64, i64, i64)>);

impl Fro for ReactorReboot {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let on = line.starts_with("on");

                let next = if on { &line[3..] } else { &line[4..] };

                let mut iter = next
                    .trim()
                    .split(',')
                    .map(|coord| &coord[2..])
                    .flat_map(|coord| coord.split(".."))
                    .map(i64::fro);

                (
                    on,
                    iter.give(),
                    iter.give(),
                    iter.give(),
                    iter.give(),
                    iter.give(),
                    iter.give(),
                )
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for ReactorReboot {
    fn one(self) -> i64 {
        let mut ons = HashSet::new();

        for (on, x1, x2, y1, y2, z1, z2) in self.0 {
            for x in cmp::max(x1, -50)..=cmp::min(x2, 50) {
                for y in cmp::max(y1, -50)..=cmp::min(y2, 50) {
                    for z in cmp::max(z1, -50)..=cmp::min(z2, 50) {
                        if on {
                            ons.insert((x, y, z));
                        } else {
                            ons.remove(&(x, y, z));
                        }
                    }
                }
            }
        }

        ons.len() as i64
    }

    fn two(self) -> i64 {
        let mut cubes = Vec::<Cube>::new();
        let mut buffer = Vec::new();

        for (on, x1, x2, y1, y2, z1, z2) in self.0 {
            buffer.clear();

            let cube = Cube {
                x1,
                x2,
                y1,
                y2,
                z1,
                z2,
            };

            for existing in &cubes {
                buffer.extend(existing.subtract(&cube));
            }

            if on {
                buffer.push(cube);
            }

            std::mem::swap(&mut cubes, &mut buffer);
        }

        cubes.iter().map(|cube| cube.len()).sum::<i64>()
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Cube {
    x1: i64,
    y1: i64,
    z1: i64,
    x2: i64,
    y2: i64,
    z2: i64,
}

impl Cube {
    fn is_empty(&self) -> bool {
        self.x1 > self.x2 || self.y1 > self.y2 || self.z1 > self.z2
    }

    fn len(&self) -> i64 {
        (self.x2 - self.x1 + 1) * (self.y2 - self.y1 + 1) * (self.z2 - self.z1 + 1)
    }

    fn intersect(&self, other: &Self) -> Cube {
        Cube {
            x1: cmp::max(self.x1, other.x1),
            x2: cmp::min(self.x2, other.x2),

            y1: cmp::max(self.y1, other.y1),
            y2: cmp::min(self.y2, other.y2),

            z1: cmp::max(self.z1, other.z1),
            z2: cmp::min(self.z2, other.z2),
        }
    }

    fn subtract(&self, other: &Self) -> impl Iterator<Item = Cube> {
        let intersection = self.intersect(other);

        if intersection.is_empty() {
            return Or::L(iter::once(*self));
        }

        //       intersection
        // --- | ------------ | ---
        //  a         b          c
        let xa = (self.x1, intersection.x1 - 1);
        let xb = (intersection.x1, intersection.x2);
        let xc = (intersection.x2 + 1, self.x2);

        let ya = (self.y1, intersection.y1 - 1);
        let yb = (intersection.y1, intersection.y2);
        let yc = (intersection.y2 + 1, self.y2);

        let za = (self.z1, intersection.z1 - 1);
        let zb = (intersection.z1, intersection.z2);
        let zc = (intersection.z2 + 1, self.z2);

        [xa, xb, xc]
            .into_iter()
            .flat_map(move |x| {
                [ya, yb, yc]
                    .into_iter()
                    .flat_map(move |y| [za, zb, zc].into_iter().map(move |z| (x, y, z)))
            })
            .filter(move |&(x, y, z)| !(x == xb && y == yb && z == zb))
            .map(|(x, y, z)| Cube {
                x1: x.0,
                x2: x.1,
                y1: y.0,
                y2: y.1,
                z1: z.0,
                z2: z.1,
            })
            .filter(|cube| !cube.is_empty())
            .tap(Or::R)
    }
}
