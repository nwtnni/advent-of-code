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

        for pos in self.0.iter().map(|dirs| net(&dirs)) {
            colors.entry(pos)
                .or_default()
                .flip();
        }

        colors
            .values()
            .filter(|colors| **colors == Color::B)
            .count()
            as i64
    }

    fn two(self) -> i64 {
        let mut colors: HashMap<(i64, i64, i64), Color> = HashMap::new();
        let mut buffer = HashMap::new();

        for pos in self.0.iter().map(|dirs| net(&dirs)) {
            colors.entry(pos)
                .or_default()
                .flip();
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
                        let mut around = 0;

                        for dx in -1i64..=1 {
                            for dy in -1i64..=1 {
                                for dz in -1i64..=1 {
                                    if dx.abs() + dy.abs() + dz.abs() != 2 {
                                        continue;
                                    }
                                    if dx.signum() + dy.signum() + dz.signum() != 0 {
                                        continue;
                                    }
                                    if colors.get(&(x + dx, y + dy, z + dz)).copied().unwrap_or_default() == Color::B {
                                        around += 1;
                                    }
                                }
                            }
                        }

                        let pos = (x, y, z);
                        let color = colors.get(&pos).copied().unwrap_or_default();
                        buffer.insert(
                            pos,
                            match (color, around) {
                                (Color::B, 0) | (Color::B, 3..=6) => Color::W,
                                (Color::W, 2) => Color::B,
                                (_, 0) => continue,
                                (color, _) => color,
                            }
                        );
                    }
                }
            }

            mem::swap(&mut buffer, &mut colors);
        }

        colors
            .values()
            .filter(|colors| **colors == Color::B)
            .count()
            as i64
    }
}

fn net(dirs: &Vec<Dir>) -> (i64, i64, i64) {
    let mut counter = HashMap::new();

    for dir in dirs {
        counter
            .entry(dir)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    let ne = counter.get(&Dir::NE).unwrap_or(&0);
    x += ne;
    z -= ne;

    let sw = counter.get(&Dir::SW).unwrap_or(&0);
    x -= sw;
    z += sw;

    let e = counter.get(&Dir::E).unwrap_or(&0);
    x += e;
    y -= e;

    let w = counter.get(&Dir::W).unwrap_or(&0);
    x -= w;
    y += w;

    let se = counter.get(&Dir::SE).unwrap_or(&0);
    z += se;
    y -= se;

    let nw = counter.get(&Dir::NW).unwrap_or(&0);
    z -= nw;
    y += nw;

    (x, y, z)
}
