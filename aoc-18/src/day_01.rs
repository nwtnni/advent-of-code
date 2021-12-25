use std::collections::HashSet;
use std::iter;

use aoc::*;

#[derive(Clone, Debug)]
pub struct ChronalCalibration(Vec<i64>);

impl Fro for ChronalCalibration {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(i64::fro)
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for ChronalCalibration {
    fn one(self) -> i64 {
        self.0.iter().sum()
    }

    fn two(self) -> i64 {
        let mut seen = HashSet::new();

        self.0
            .iter()
            .copied()
            .cycle()
            .tap(|cycle| iter::once(0).chain(cycle))
            .scan(0, |frequency, change| {
                *frequency += change;
                Some(*frequency)
            })
            .find_map(|frequency| match seen.insert(frequency) {
                false => Some(frequency),
                true => None,
            })
            .unwrap()
    }
}

#[test]
fn test_1_0() {
    assert_eq!(ChronalCalibration::fro("+1\n+1\n+1").one(), 3);
}

#[test]
fn test_1_1() {
    assert_eq!(ChronalCalibration::fro("+1\n+1\n-2").one(), 0);
}

#[test]
fn test_1_2() {
    assert_eq!(ChronalCalibration::fro("-1\n-2\n-3").one(), -6);
}

#[test]
fn test_2_0() {
    assert_eq!(ChronalCalibration::fro("+1\n-1").two(), 0);
}

#[test]
fn test_2_1() {
    assert_eq!(ChronalCalibration::fro("+3\n+3\n+4\n-2\n-4").two(), 10);
}

#[test]
fn test_2_2() {
    assert_eq!(ChronalCalibration::fro("-6\n+3\n+8\n+5\n-6").two(), 5);
}

#[test]
fn test_2_3() {
    assert_eq!(ChronalCalibration::fro("+7\n+7\n-2\n-7\n-4").two(), 14);
}
