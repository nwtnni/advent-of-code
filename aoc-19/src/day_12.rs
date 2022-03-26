use std::collections::HashSet;

use aoc::*;

pub const MOONS: usize = 4;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct TheNBodyProblem([[i64; 3]; MOONS]);

impl Fro for TheNBodyProblem {
    fn fro(input: &str) -> Self {
        let mut moons = [[0; 3]; MOONS];
        for (m, line) in input.trim().split('\n').enumerate() {
            let mut iter = line
                .trim_start_matches('<')
                .trim_end_matches('>')
                .split(',');
            let x = iter.give().split('=').last().unwrap().to::<i64>();
            let y = iter.give().split('=').last().unwrap().to::<i64>();
            let z = iter.give().split('=').last().unwrap().to::<i64>();
            moons[m] = [x, y, z];
        }
        TheNBodyProblem(moons)
    }
}

impl Solution for TheNBodyProblem {
    fn one(self) -> i64 {
        let mut pos = self.0;
        let mut vel = [[0; 3]; MOONS];

        for _ in 0..1000 {
            for a in 0..pos.len() {
                for b in a + 1..pos.len() {
                    for axis in 0..3 {
                        let pa = pos[a][axis];
                        let pb = pos[b][axis];
                        let delta = if pa < pb {
                            1
                        } else if pa == pb {
                            0
                        } else {
                            -1
                        };
                        vel[a][axis] += delta;
                        vel[b][axis] -= delta;
                    }
                }
            }
            for (p, v) in pos.iter_mut().zip(&vel) {
                for axis in 0..3 {
                    p[axis] += v[axis];
                }
            }
        }

        pos.iter()
            .zip(&vel)
            .map(|(p, v)| {
                p.iter().map(|x| x.abs()).sum::<i64>() * v.iter().map(|x| x.abs()).sum::<i64>()
            })
            .sum()
    }

    fn two(self) -> i64 {
        let mut seen = HashSet::new();
        let mut pos = self.0;
        let mut vel = [[0; 3]; MOONS];
        let mut cycle = [0; 3];

        for axis in 0..3 {
            seen.clear();
            seen.insert((pos, vel));

            for step in 1.. {
                for a in 0..pos.len() {
                    for b in a + 1..pos.len() {
                        let pa = pos[a][axis];
                        let pb = pos[b][axis];
                        let delta = if pa < pb {
                            1
                        } else if pa == pb {
                            0
                        } else {
                            -1
                        };
                        vel[a][axis] += delta;
                        vel[b][axis] -= delta;
                    }
                }

                for a in 0..pos.len() {
                    pos[a][axis] += vel[a][axis];
                }

                if !seen.insert((pos, vel)) {
                    cycle[axis] = step;
                    break;
                }
            }
        }

        cycle.iter().copied().fold(1, lcm)
    }
}
