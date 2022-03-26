use std::collections::HashSet;
use std::mem;

use aoc::*;

#[derive(Clone, Debug)]
pub struct PerfectlySphericalHousesInAVacuum(Vec<Dir>);

impl Fro for PerfectlySphericalHousesInAVacuum {
    fn fro(input: &str) -> Self {
        input
            .chars()
            .map(|char| match char {
                '^' => Dir::N,
                'v' => Dir::S,
                '>' => Dir::E,
                '<' => Dir::W,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for PerfectlySphericalHousesInAVacuum {
    fn one(self) -> i64 {
        self.0
            .iter()
            .chain(&[Dir::N])
            .scan(Pos::default(), |pos, dir| {
                Some(mem::replace(pos, pos.shift(*dir)))
            })
            .collect::<HashSet<_>>()
            .len() as i64
    }

    fn two(self) -> i64 {
        self.0
            .iter()
            .chain(&[Dir::N, Dir::N])
            .scan(
                (Pos::default(), Pos::default(), false),
                |(human, robot, flip), dir| {
                    if mem::replace(flip, !*flip) {
                        Some(mem::replace(human, human.shift(*dir)))
                    } else {
                        Some(mem::replace(robot, robot.shift(*dir)))
                    }
                },
            )
            .collect::<HashSet<_>>()
            .len() as i64
    }
}

#[cfg(test)]
mod tests {

    use aoc::Fro as _;
    use aoc::Solution as _;

    #[test]
    fn test_1_0() {
        assert_eq!(super::PerfectlySphericalHousesInAVacuum::fro(">").one(), 2)
    }

    #[test]
    fn test_1_1() {
        assert_eq!(
            super::PerfectlySphericalHousesInAVacuum::fro("^>v<").one(),
            4
        )
    }

    #[test]
    fn test_1_2() {
        assert_eq!(
            super::PerfectlySphericalHousesInAVacuum::fro("^v^v^v^v^v").one(),
            2
        )
    }

    #[test]
    fn test_2_0() {
        assert_eq!(super::PerfectlySphericalHousesInAVacuum::fro("^v").two(), 3)
    }

    #[test]
    fn test_2_1() {
        assert_eq!(
            super::PerfectlySphericalHousesInAVacuum::fro("^>v<").two(),
            3
        )
    }

    #[test]
    fn test_2_2() {
        assert_eq!(
            super::PerfectlySphericalHousesInAVacuum::fro("^v^v^v^v^v").two(),
            11
        )
    }
}
