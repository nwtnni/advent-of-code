use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct BoilingBoulders(Vec<(i64, i64, i64)>);

impl Fro for BoilingBoulders {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let mut iter = line.split(',');
                (
                    i64::fro(iter.give()),
                    i64::fro(iter.give()),
                    i64::fro(iter.give()),
                )
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for BoilingBoulders {
    fn one(self) -> i64 {
        let mut faces = HashMap::new();

        for (x, y, z) in self.0 {
            let mut face = 0b0011_1111i64;

            match faces.get_mut(&(x + 1, y, z)) {
                None => (),
                Some(other) => {
                    face &= 0b0001_1111;
                    *other &= 0b0010_1111;
                }
            }

            match faces.get_mut(&(x - 1, y, z)) {
                None => (),
                Some(other) => {
                    face &= 0b0010_1111;
                    *other &= 0b0001_1111;
                }
            }

            match faces.get_mut(&(x, y + 1, z)) {
                None => (),
                Some(other) => {
                    face &= 0b0011_0111;
                    *other &= 0b0011_1011;
                }
            }

            match faces.get_mut(&(x, y - 1, z)) {
                None => (),
                Some(other) => {
                    face &= 0b0011_1011;
                    *other &= 0b0011_0111;
                }
            }

            match faces.get_mut(&(x, y, z + 1)) {
                None => (),
                Some(other) => {
                    face &= 0b0011_1101;
                    *other &= 0b0011_1110;
                }
            }

            match faces.get_mut(&(x, y, z - 1)) {
                None => (),
                Some(other) => {
                    face &= 0b0011_1110;
                    *other &= 0b0011_1101;
                }
            }

            faces.insert((x, y, z), face);
        }

        faces.values().map(|face| (*face).count_ones()).sum::<u32>() as i64
    }

    fn two(self) -> i64 {
        let mut faces = HashMap::new();

        for (x, y, z) in self.0 {
            let mut face = 0b0011_1111;

            match faces.get_mut(&(x + 1, y, z)) {
                None => (),
                Some(other) => {
                    face &= 0b0001_1111;
                    *other &= 0b0010_1111;
                }
            }

            match faces.get_mut(&(x - 1, y, z)) {
                None => (),
                Some(other) => {
                    face &= 0b0010_1111;
                    *other &= 0b0001_1111;
                }
            }

            match faces.get_mut(&(x, y + 1, z)) {
                None => (),
                Some(other) => {
                    face &= 0b0011_0111;
                    *other &= 0b0011_1011;
                }
            }

            match faces.get_mut(&(x, y - 1, z)) {
                None => (),
                Some(other) => {
                    face &= 0b0011_1011;
                    *other &= 0b0011_0111;
                }
            }

            match faces.get_mut(&(x, y, z + 1)) {
                None => (),
                Some(other) => {
                    face &= 0b0011_1101;
                    *other &= 0b0011_1110;
                }
            }

            match faces.get_mut(&(x, y, z - 1)) {
                None => (),
                Some(other) => {
                    face &= 0b0011_1110;
                    *other &= 0b0011_1101;
                }
            }

            faces.insert((x, y, z), face);
        }

        faces.retain(|_, face| *face > 0);

        let mut min_x = i64::MAX;
        let mut max_x = i64::MIN;

        let mut min_y = i64::MAX;
        let mut max_y = i64::MIN;

        let mut min_z = i64::MAX;
        let mut max_z = i64::MIN;

        for (x, y, z) in faces.keys().copied() {
            min_x = cmp::min(x - 1, min_x);
            max_x = cmp::max(x + 1, max_x);

            min_y = cmp::min(y - 1, min_y);
            max_y = cmp::max(y + 1, max_y);

            min_z = cmp::min(z - 1, min_z);
            max_z = cmp::max(z + 1, max_z);
        }

        let mut total = 0;
        let mut stack = Vec::new();
        let mut seen = HashSet::new();

        stack.push((min_x, min_y, min_z));

        while let Some((x, y, z)) = stack.pop() {
            if x < min_x
                || y < min_y
                || z < min_z
                || x > max_x
                || y > max_y
                || z > max_z
                || faces.contains_key(&(x, y, z))
                || !seen.insert((x, y, z))
            {
                continue;
            }

            match faces.get(&(x + 1, y, z)) {
                Some(face) if face & 0b0001_0000 > 0 => total += 1,
                Some(_) => (),
                None => stack.push((x + 1, y, z)),
            }

            match faces.get(&(x - 1, y, z)) {
                Some(face) if face & 0b0010_0000 > 0 => total += 1,
                Some(_) => (),
                None => stack.push((x - 1, y, z)),
            }

            match faces.get(&(x, y + 1, z)) {
                Some(face) if face & 0b0000_0100 > 0 => total += 1,
                Some(_) => (),
                None => stack.push((x, y + 1, z)),
            }

            match faces.get(&(x, y - 1, z)) {
                Some(face) if face & 0b0000_1000 > 0 => total += 1,
                Some(_) => (),
                None => stack.push((x, y - 1, z)),
            }

            match faces.get(&(x, y, z + 1)) {
                Some(face) if face & 0b0000_0001 > 0 => total += 1,
                Some(_) => (),
                None => stack.push((x, y, z + 1)),
            }

            match faces.get(&(x, y, z - 1)) {
                Some(face) if face & 0b0000_0010 > 0 => total += 1,
                Some(_) => (),
                None => stack.push((x, y, z - 1)),
            }
        }

        total
    }
}
