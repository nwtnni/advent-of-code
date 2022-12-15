use std::cmp;
use std::collections::HashSet;

use aoc::*;
use range_collections::AbstractRangeSet;
use range_collections::RangeSet2;

#[derive(Clone, Debug)]
pub struct BeaconExclusionZone(Vec<(Pos, Pos)>);

impl Fro for BeaconExclusionZone {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let (a, b) = line.split_once(": closest beacon is at x=").unwrap();
                let (a1, a2) = a
                    .trim_start_matches("Sensor at x=")
                    .split_once(", y=")
                    .unwrap();
                let (b1, b2) = b.split_once(", y=").unwrap();
                (
                    Pos {
                        x: i64::fro(a1),
                        y: i64::fro(a2),
                    },
                    Pos {
                        x: i64::fro(b1),
                        y: i64::fro(b2),
                    },
                )
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for BeaconExclusionZone {
    fn one(self) -> i64 {
        let mut unique = HashSet::new();

        for (sensor, beacon) in &self.0 {
            let distance = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();

            for dx in 0.. {
                if dx + (sensor.y - 2000000).abs() > distance {
                    break;
                } else {
                    unique.insert(sensor.x + dx);
                    unique.insert(sensor.x - dx);
                }
            }
        }

        for (_, beacon) in &self.0 {
            if beacon.y == 2000000 {
                unique.remove(&beacon.x);
            }
        }

        unique.len() as i64
    }

    fn two(self) -> i64 {
        let mut circles = Vec::new();

        for (sensor, beacon) in &self.0 {
            let distance = (sensor.x - beacon.x).abs() + (sensor.y - beacon.y).abs();
            circles.push((sensor, beacon, distance));
        }

        circles.sort();

        const BOUND: i64 = 4000000;

        let all = RangeSet2::from(0..BOUND + 1);

        for y in 0..=BOUND {
            let mut set = RangeSet2::empty();

            circles.iter().for_each(|(sensor, _, distance)| {
                let dy = (sensor.y - y).abs();
                let dx = distance - dy;

                set.union_with(&RangeSet2::from(
                    cmp::max(0, sensor.x - dx)..(cmp::min(BOUND + 1, sensor.x + dx + 1)),
                ));
            });

            if set != all {
                return (set.boundaries()[1]) * 4000000 + y;
            }
        }

        unreachable!()
    }
}
