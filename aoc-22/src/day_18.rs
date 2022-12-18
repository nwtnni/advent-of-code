use std::cmp;
use std::collections::HashMap;

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
        todo!()
    }
}
