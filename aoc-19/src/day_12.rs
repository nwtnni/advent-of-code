use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Placeholder(Vec<Vec3>);

type Vec3 = [i64; 3];

impl Fro for Placeholder {
    fn fro(input: &str) -> Self {
        let mut moons = Vec::new();
        for line in input.trim().split('\n') {
            let mut iter = line.trim_start_matches('<').trim_end_matches('>').split(',');
            let x = iter.give().split('=').last().unwrap().to::<i64>();
            let y = iter.give().split('=').last().unwrap().to::<i64>();
            let z = iter.give().split('=').last().unwrap().to::<i64>();
            moons.push([x, y, z]);
        }
        Placeholder(moons)
    }
}

impl Solution for Placeholder {
    fn one(self) -> i64 {
        let mut pos = self.0;
        let mut vel = vec![[0; 3]; pos.len()];

        for _ in 0..1000 {
            for a in 0..pos.len() {
                for b in a + 1..pos.len() {
                    let ma = pos[a];
                    let mb = pos[b];

                    for axis in 0..3 {
                        let delta = if ma[axis] < mb[axis] { 1 } else if ma[axis] == mb[axis] { 0 } else { -1 };
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
            .map(|(p, v)|
                p.iter().map(|x| x.abs()).sum::<i64>() *
                v.iter().map(|x| x.abs()).sum::<i64>()
            )
            .sum()
    }

    fn two(self) -> i64 {
        let mut seen = HashSet::new();
        let mut pos = self.0;
        let mut vel = vec![[0; 3]; pos.len()];

        let mut xs = [0; 3];

        for axis in 0..3 {
            seen.clear();

            let p = pos.iter().copied().map(|p| p[axis]);
            let v = vel.iter().copied().map(|v| v[axis]);
            seen.insert(p.chain(v).collect::<Vec<_>>());

            for step in 1.. {
                for a in 0..pos.len() {
                    for b in a + 1..pos.len() {
                        let ma = pos[a][axis];
                        let mb = pos[b][axis];

                        let delta = if ma < mb { 1 } else if ma == mb { 0 } else { -1 };
                        vel[a][axis] += delta;
                        vel[b][axis] -= delta;
                    }
                }

                for a in 0..pos.len() {
                    pos[a][axis] += vel[a][axis];
                }

                let p = pos.iter().copied().map(|p| p[axis]);
                let v = vel.iter().copied().map(|v| v[axis]);
                if !seen.insert(p.chain(v).collect::<Vec<_>>()) {
                    xs[axis] = step;
                    break;
                }
            }
        }

        xs.iter()
            .copied()
            .fold(1, lcm)
    }
}
