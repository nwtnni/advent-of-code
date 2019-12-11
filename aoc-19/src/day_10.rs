use std::collections::BTreeMap;
use std::collections::BinaryHeap;
use std::cmp;

use aoc::*;

#[derive(Clone, Debug)]
pub struct Placeholder(Vec<Pos>);

impl Fro for Placeholder {
    fn fro(input: &str) -> Self {
        let mut asteroids = Vec::new();
        for (row, line) in input.split_whitespace().enumerate() {
            for (col, char) in line.chars().enumerate() {
                if char == '#' {
                    asteroids.push(Pos { x: col as i64, y: row as i64 });
                }
            }
        }
        Placeholder(asteroids)
    }
}

impl Placeholder {
    fn station(&self) -> (i64, Pos) {
        let mut max = 0;
        let mut station = Pos::default();

        for a in &self.0 {

            // Asteroids visible from A
            let mut visible = self.0.clone();
            visible.retain(|p| p != a);

            for b in self.0.iter().filter(|p| *p != a) {

                // Remove asteroids occluded by B
                visible.retain(|c| {
                    if b == c { return true }

                    // Dot product is given by
                    // |AB| * |AC| * cos(θ) = AB ⋅ AC
                    //
                    // If cos(θ) == ±1 then the lines are parallel
                    let ab = Pos { x: b.x - a.x, y: b.y - a.y };
                    let ac = Pos { x: c.x - a.x, y: c.y - a.y };

                    let dot = (ab.x * ac.x + ab.y * ac.y) as f64;

                    // Not same direction
                    if dot.is_sign_negative() {
                        return true;
                    }

                    let dist_ab = ((ab.x.pow(2) + ab.y.pow(2)) as f64).sqrt();
                    let dist_ac = ((ac.x.pow(2) + ac.y.pow(2)) as f64).sqrt();

                    (dot - dist_ab * dist_ac).abs() > 0.0005 || dist_ac < dist_ab
                });
            }

            if visible.len() > max {
                station = *a;
                max = visible.len();
            }
        }
        (max as i64, station)
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

impl Solution for Placeholder {
    fn one(self) -> i64 {
        self.station().0
    }

    fn two(self) -> i64 {
        let (_, s) = self.station();

        let mut circle = BTreeMap::new();
        let sv = Pos { x: 0, y: -1 };

        for a in self.0.into_iter().filter(|p| *p != s) {
            let sa = Pos { x: a.x - s.x, y: a.y - s.y };
            let cos = (sv.x * sa.x + sv.y * sa.y) as f64;
            let sin = (sv.x * sa.y - sa.x * sv.y) as f64;
            let mut theta = f64::atan2(sin, cos);
            if theta.is_sign_negative() {
                theta += 2.0 * std::f64::consts::PI;
            }
            let distance = sa.x.pow(2) + sa.y.pow(2);

            #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
            struct Asteroid {
                d: i64,
                p: Pos,
            }

            circle
                .entry(Approx(theta))
                .or_insert_with(BinaryHeap::new)
                .push(cmp::Reverse(Asteroid { p: a, d: distance }));
        }

        let mut vaporized = 0;
        loop {
            for radial in circle.values_mut() {
                if let Some(cmp::Reverse(asteroid)) = radial.pop() {
                    vaporized += 1;
                    if vaporized == 200 {
                        return asteroid.p.x * 100 + asteroid.p.y;
                    }
                }
            }
        }
    }
}

mod test {
    use aoc::*;
    use super::Placeholder;

    #[test]
    fn test_1_8() {
        let map = Placeholder::fro(str::trim("
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
        let map = Placeholder::fro(str::trim("
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
        let map = Placeholder::fro(str::trim("
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
        let map = Placeholder::fro(str::trim("
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
