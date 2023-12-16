use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct TheFloorWillBeLava(Vec<Vec<Option<Tile>>>);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Tile {
    F,
    B,
    H,
    V,
}

impl Fro for TheFloorWillBeLava {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                line.chars()
                    .map(|char| match char {
                        '|' => Some(Tile::V),
                        '-' => Some(Tile::H),
                        '/' => Some(Tile::F),
                        '\\' => Some(Tile::B),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for TheFloorWillBeLava {
    fn one(self) -> i64 {
        self.score(Pos::from_i_j(0, 0), Dir::E)
    }

    fn two(self) -> i64 {
        let h = self.0.len() as i64;
        let w = self.0[0].len() as i64;

        (0..h)
            .flat_map(|i| {
                [
                    (Pos::from_i_j(i, 0), Dir::E),
                    (Pos::from_i_j(i, w - 1), Dir::W),
                ]
            })
            .chain((0..w).flat_map(|j| {
                [
                    (Pos::from_i_j(0, j), Dir::S),
                    (Pos::from_i_j(h - 1, j), Dir::N),
                ]
            }))
            .map(|(pos, dir)| self.score(pos, dir))
            .max()
            .unwrap()
    }
}
impl TheFloorWillBeLava {
    fn score(&self, pos: Pos, dir: Dir) -> i64 {
        let mut visited = HashSet::new();
        self.go(pos, dir, &mut visited);
        visited
            .iter()
            .map(|(p, _)| *p)
            .collect::<HashSet<_>>()
            .len() as i64
    }

    fn go(&self, mut pos: Pos, mut dir: Dir, visited: &mut HashSet<(Pos, Dir)>) {
        loop {
            let Some(tile) = self
                .0
                .get(pos.i() as usize)
                .and_then(|row| row.get(pos.j() as usize))
            else {
                return;
            };

            if !visited.insert((pos, dir)) {
                return;
            }

            match tile {
                None => {
                    pos.shift_mut(dir);
                }
                Some(Tile::F) => {
                    dir = match dir {
                        Dir::N => Dir::E,
                        Dir::S => Dir::W,
                        Dir::W => Dir::S,
                        Dir::E => Dir::N,
                    };

                    pos.shift_mut(dir);
                }
                Some(Tile::B) => {
                    dir = match dir {
                        Dir::N => Dir::W,
                        Dir::S => Dir::E,
                        Dir::W => Dir::N,
                        Dir::E => Dir::S,
                    };

                    pos.shift_mut(dir);
                }
                Some(Tile::H) => match dir {
                    Dir::N | Dir::S => {
                        self.go(pos.shift(Dir::W), Dir::W, visited);
                        self.go(pos.shift(Dir::E), Dir::E, visited);
                    }
                    Dir::W | Dir::E => {
                        pos.shift_mut(dir);
                    }
                },
                Some(Tile::V) => match dir {
                    Dir::N | Dir::S => {
                        pos.shift_mut(dir);
                    }
                    Dir::W | Dir::E => {
                        self.go(pos.shift(Dir::N), Dir::N, visited);
                        self.go(pos.shift(Dir::S), Dir::S, visited);
                    }
                },
            }
        }
    }
}
