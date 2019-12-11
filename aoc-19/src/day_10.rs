use std::collections::BTreeMap;
use std::collections::BinaryHeap;
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

impl MonitoringStation {
    fn visible(&self, a: Pos) -> i64 {

        // Collect asteroids visible from A
        let mut visible = self.0.clone();
        visible.retain(|p| *p != a);

        for b in self.0.iter().filter(|p| **p != a) {

            // Remove asteroids occluded by B
            visible.retain(|c| {

                // B can never occlude itself
                if b == c { return true }

                // Dot product is given by
                // |AB| * |AC| * cos(θ) = AB ⋅ AC
                //
                // cos(θ) == ±1 ⇒ AB || AC
                let ab = Pos { x: b.x - a.x, y: b.y - a.y };
                let ac = Pos { x: c.x - a.x, y: c.y - a.y };
                let dot = (ab.x * ac.x + ab.y * ac.y) as f64;

                // Facing opposite directions, cannot occlude
                if dot.is_sign_negative() {
                    return true;
                }

                let dist_ab = ((ab.x.pow(2) + ab.y.pow(2)) as f64).sqrt();
                let dist_ac = ((ac.x.pow(2) + ac.y.pow(2)) as f64).sqrt();

                // C is visible if cos(θ) != 1 or if C is closer than B
                Approx(dot) != Approx(dist_ab * dist_ac) || dist_ac < dist_ab
            });
        }

        // Count number of remaining candidates
        visible.len() as i64
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

        // Vertical vector
        let sv = Pos { x: 0, y: -1 };

        // Primary data structure: ordered map from
        // unique angles to heaps of asteroids, sorted
        // by their proximity to the monitoring station.
        let mut laser = BTreeMap::<Approx, BinaryHeap<Asteroid>>::new();

        for a in self.0.into_iter().filter(|p| *p != s) {

            // Compute relative vector from station S to asteroid A
            let sa = Pos { x: a.x - s.x, y: a.y - s.y };

            // sin(θ) ∝ SV X SA
            let sin = (sv.x * sa.y - sa.x * sv.y) as f64;

            // cos(θ) ∝ SV ⋅ SA
            let cos = (sv.x * sa.x + sv.y * sa.y) as f64;

            // Map θ from (-π, π] to [0, 2π)
            let theta = match f64::atan2(sin, cos) {
            | theta if theta.is_sign_negative() => theta + 2.0 * std::f64::consts::PI,
            | theta => theta,
            };

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
