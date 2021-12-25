use std::collections::HashMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct NoMatterHowYouSliceIt(Vec<Fabric>);

#[derive(Copy, Clone, Debug)]
struct Fabric {
    x: i64,
    y: i64,
    w: i64,
    h: i64,
}

impl Fro for NoMatterHowYouSliceIt {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(Fabric::fro)
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Fro for Fabric {
    fn fro(input: &str) -> Self {
        let (_, claim) = input.trim().split_once(" @ ").unwrap();

        let (pos, dim) = claim.split_once(": ").unwrap();

        let (x, y) = pos.split_once(',').unwrap();
        let (w, h) = dim.split_once('x').unwrap();

        Self {
            x: i64::fro(x),
            y: i64::fro(y),
            w: i64::fro(w),
            h: i64::fro(h),
        }
    }
}

impl Solution for NoMatterHowYouSliceIt {
    fn one(self) -> i64 {
        let mut claimed = HashMap::new();

        for fabric in &self.0 {
            for y in fabric.y..fabric.y + fabric.h {
                for x in fabric.x..fabric.x + fabric.w {
                    match claimed.get_mut(&(x, y)) {
                        None => claimed.insert((x, y), false).tap(drop),
                        Some(twice) => *twice = true,
                    }
                }
            }
        }

        claimed.values().filter(|twice| **twice).count() as i64
    }

    fn two(self) -> i64 {
        let mut claimed = HashMap::new();
        let mut clean = vec![true; self.0.len()];

        for (claim, fabric) in self.0.iter().enumerate() {
            for y in fabric.y..fabric.y + fabric.h {
                for x in fabric.x..fabric.x + fabric.w {
                    match claimed.get_mut(&(x, y)) {
                        None => {
                            claimed.insert((x, y), Some(claim));
                        }
                        Some(previous) => {
                            if let Some(id) = previous {
                                clean[*id] = false;
                            }
                            clean[claim] = false;
                            *previous = None;
                        }
                    }
                }
            }
        }

        clean
            .into_iter()
            .position(std::convert::identity)
            .map(|claim| claim as i64 + 1)
            .unwrap()
    }
}

#[cfg(test)]
const EXAMPLE: &str = "\
    #1 @ 1,3: 4x4\n\
    #2 @ 3,1: 4x4\n\
    #3 @ 5,5: 2x2\n\
";

#[test]
fn test_1_0() {
    assert_eq!(NoMatterHowYouSliceIt::fro(EXAMPLE).one(), 4);
}

#[test]
fn test_2_0() {
    assert_eq!(NoMatterHowYouSliceIt::fro(EXAMPLE).two(), 3);
}
