use std::collections::BTreeMap;
use std::collections::BinaryHeap;
use std::collections::BTreeSet;
use std::cmp;

use aoc::*;

#[derive(Clone, Debug)]
pub struct MonitoringStation(Vec<Pos>);

impl Fro for MonitoringStation {
    fn fro(input: &str) -> Self {
        let mut asteroids = Vec::new();
        for (row, line) in input.split_whitespace().enumerate() {
            for (col, char) in line.chars().enumerate() {
                if char == '#' {
                    asteroids.push(Pos { x: col as i64, y: row as i64 });
                }
            }
        }
        MonitoringStation(asteroids)
    }
}

// Vertical vector
const X_AXIS: Pos = Pos { x: 1, y:  0 };
const Y_AXIS: Pos = Pos { x: 0, y: -1 };

/// Compute the angle between vectors AB and AC,
/// treating AB as the zero axis.
fn angle(ab: Pos, ac: Pos) -> f64 {
    // sin(θ) ∝ AB X AC
    let sin = (ab.x * ac.y - ac.x * ab.y) as f64;

    // cos(θ) ∝ AB ⋅ AC
    let cos = (ab.x * ac.x + ab.y * ac.y) as f64;

    // Map θ from (-π, π] to [0, 2π)
    match f64::atan2(sin, cos) {
    | theta if theta.is_sign_negative() => theta + 2.0 * std::f64::consts::PI,
    | theta => theta,
    }
}

impl MonitoringStation {
    fn visible(&self, a: Pos) -> i64 {
        self.0
            .iter()
            .filter(|p| **p != a)
            .map(|b| {
                let ab = Pos { x: b.x - a.x, y: b.y - a.y };
                angle(X_AXIS, ab)
            })
            .map(Approx)
            .collect::<BTreeSet<_>>()
            .len() as i64
    }
}

#[derive(Copy, Clone, Debug)]
struct Approx(f64);

impl PartialEq for Approx {
    fn eq(&self, other: &Approx) -> bool {
        (self.0 - other.0).abs() < 0.00005
    }
}

impl Eq for Approx {}

impl PartialOrd for Approx {
    fn partial_cmp(&self, other: &Approx) -> Option<cmp::Ordering> {
        if self.eq(other) {
            Some(cmp::Ordering::Equal)
        } else {
            self.0.partial_cmp(&other.0)
        }
    }
}

impl Ord for Approx {
    fn cmp(&self, other: &Approx) -> cmp::Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl Solution for MonitoringStation {
    fn one(self) -> i64 {
        self.0
            .iter()
            .map(|a| self.visible(*a))
            .max()
            .unwrap()
    }

    fn two(self) -> i64 {

        /// Asteroid sorted by distance to station
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
        struct Asteroid {
            d: cmp::Reverse<i64>,
            p: Pos,
        }

        let s = self.0
            .iter()
            .max_by_key(|a| self.visible(**a))
            .copied()
            .unwrap();

        // Primary data structure: ordered map from
        // unique angles to heaps of asteroids, sorted
        // by their proximity to the monitoring station.
        let mut laser = BTreeMap::<Approx, BinaryHeap<Asteroid>>::new();

        for a in self.0.into_iter().filter(|p| *p != s) {

            // Compute relative vector from station S to asteroid A
            let sa = Pos { x: a.x - s.x, y: a.y - s.y };

            let theta = angle(Y_AXIS, sa);

            // Group A in sorted order by distance to S
            // with other asteroids along angle θ.
            laser.entry(Approx(theta))
                .or_insert_with(BinaryHeap::new)
                .push(Asteroid {
                    p: a,
                    d: cmp::Reverse(sa.x.pow(2) + sa.y.pow(2)),
                });
        }

        // Vaporize in a circle, destroying the closest
        // asteroid along each angle per iteration.
        let mut vaporized = 0;
        loop {
            for radial in laser.values_mut() {
                if let Some(asteroid) = radial.pop() {
                    vaporized += 1;
                    if vaporized == 200 {
                        return asteroid.p.x * 100 + asteroid.p.y;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use aoc::*;
    use super::MonitoringStation;

    #[test]
    fn test_1_8() {
        let map = MonitoringStation::fro(str::trim("
            .#..#
            .....
            #####
            ....#
            ...##
        "));
        assert_eq!(map.one(), 8);
    }

    #[test]
    fn test_1_33() {
        let map = MonitoringStation::fro(str::trim("
            ......#.#.
            #..#.#....
            ..#######.
            .#.#.###..
            .#..#.....
            ..#....#.#
            #..#....#.
            .##.#..###
            ##...#..#.
            .#....####
        "));
        assert_eq!(map.one(), 33);
    }

    #[test]
    fn test_1_210() {
        let map = MonitoringStation::fro(str::trim("
            .#..##.###...#######
            ##.############..##.
            .#.######.########.#
            .###.#######.####.#.
            #####.##.#.##.###.##
            ..#####..#.#########
            ####################
            #.####....###.#.#.##
            ##.#################
            #####.##.###..####..
            ..######..##.#######
            ####.##.####...##..#
            .#####..#.######.###
            ##...#.##########...
            #.##########.#######
            .####.#.###.###.#.##
            ....##.##.###..#####
            .#.#.###########.###
            #.#.#.#####.####.###
            ###.##.####.##.#..##
        "));
        assert_eq!(map.one(), 210);
    }

    #[test]
    fn test_2_210() {
        let map = MonitoringStation::fro(str::trim("
            .#..##.###...#######
            ##.############..##.
            .#.######.########.#
            .###.#######.####.#.
            #####.##.#.##.###.##
            ..#####..#.#########
            ####################
            #.####....###.#.#.##
            ##.#################
            #####.##.###..####..
            ..######..##.#######
            ####.##.####...##..#
            .#####..#.######.###
            ##...#.##########...
            #.##########.#######
            .####.#.###.###.#.##
            ....##.##.###..#####
            .#.#.###########.###
            #.#.#.#####.####.###
            ###.##.####.##.#..##
        "));
        assert_eq!(map.two(), 802);
    }
}
