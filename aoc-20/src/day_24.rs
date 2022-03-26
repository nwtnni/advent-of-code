use std::cmp;
use std::collections::HashMap;
use std::mem;

use aoc::*;

#[derive(Clone, Debug)]
pub struct LobbyLayout(Vec<Vec<Dir>>);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Dir {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

impl Fro for LobbyLayout {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let mut i = 0;
                let mut dirs = Vec::new();
                while i < line.len() {
                    if &line[i..i + 1] == "e" || &line[i..i + 1] == "w" {
                        dirs.push(Dir::fro(&line[i..i + 1]));
                        i += 1;
                    } else {
                        dirs.push(Dir::fro(&line[i..i + 2]));
                        i += 2;
                    }
                }
                dirs
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Fro for Dir {
    fn fro(input: &str) -> Self {
        match input {
            "e" => Dir::E,
            "se" => Dir::SE,
            "sw" => Dir::SW,
            "w" => Dir::W,
            "nw" => Dir::NW,
            "ne" => Dir::NE,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Color {
    B,
    W,
}

impl Color {
    fn flip(&mut self) {
        match self {
            Color::B => *self = Color::W,
            Color::W => *self = Color::B,
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::W
    }
}

impl Solution for LobbyLayout {
    fn one(self) -> i64 {
        let mut colors: HashMap<(i64, i64, i64), Color> = HashMap::new();

        for dirs in &self.0 {
            colors.entry(walk(dirs)).or_default().flip();
        }

        colors
            .values()
            .filter(|colors| **colors == Color::B)
            .count() as i64
    }

    fn two(self) -> i64 {
        let mut colors: HashMap<(i64, i64, i64), Color> = HashMap::new();
        let mut buffer = HashMap::new();

        for dirs in &self.0 {
            colors.entry(walk(dirs)).or_default().flip();
        }

        for _ in 0..100 {
            buffer.clear();

            let mut min_x = i64::MAX;
            let mut max_x = i64::MIN;

            let mut min_y = i64::MAX;
            let mut max_y = i64::MIN;

            let mut min_z = i64::MAX;
            let mut max_z = i64::MIN;

            for (x, y, z) in colors.keys() {
                min_x = cmp::min(*x, min_x);
                max_x = cmp::max(*x, max_x);

                min_y = cmp::min(*y, min_y);
                max_y = cmp::max(*y, max_y);

                min_z = cmp::min(*z, min_z);
                max_z = cmp::max(*z, max_z);
            }

            for x in min_x - 1..=max_x + 1 {
                for y in min_y - 1..=max_y + 1 {
                    for z in min_z - 1..=max_z + 1 {
                        let around = [
                            (0, 1, -1),
                            (0, -1, 1),
                            (1, 0, -1),
                            (-1, 0, 1),
                            (1, -1, 0),
                            (-1, 1, 0),
                        ]
                        .iter()
                        .flat_map(|(dx, dy, dz)| colors.get(&(x + dx, y + dy, z + dz)))
                        .filter(|color| **color == Color::B)
                        .count();

                        let color = colors.get(&(x, y, z)).copied().unwrap_or_default();

                        buffer.insert(
                            (x, y, z),
                            match (color, around) {
                                (Color::B, 0) | (Color::B, 3..=6) => Color::W,
                                (Color::W, 2) => Color::B,
                                (_, 0) => continue,
                                (color, _) => color,
                            },
                        );
                    }
                }
            }

            mem::swap(&mut buffer, &mut colors);
        }

        colors
            .values()
            .filter(|colors| **colors == Color::B)
            .count() as i64
    }
}

fn walk(dirs: &[Dir]) -> (i64, i64, i64) {
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    for dir in dirs {
        match dir {
            Dir::NE => {
                x += 1;
                z -= 1
            }
            Dir::SW => {
                x -= 1;
                z += 1
            }
            Dir::E => {
                x += 1;
                y -= 1
            }
            Dir::W => {
                x -= 1;
                y += 1
            }
            Dir::SE => {
                z += 1;
                y -= 1
            }
            Dir::NW => {
                z -= 1;
                y += 1
            }
        }
    }

    (x, y, z)
}
