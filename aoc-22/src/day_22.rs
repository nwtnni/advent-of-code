use std::collections::HashMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct MonkeyMap {
    size: i64,
    grid: HashMap<Pos, Tile>,
    path: Vec<Move>,
}

#[derive(Clone, Debug)]
enum Move {
    Turn(bool),
    Forward(i64),
}

#[derive(Clone, Debug)]
enum Tile {
    Wall,
    Floor,
}

impl Fro for MonkeyMap {
    fn fro(input: &str) -> Self {
        let mut iter = input.trim_end().split("\n\n");

        let mut grid = HashMap::new();
        let mut size = None;

        for (y, row) in iter.give().split('\n').enumerate() {
            let mut start = None;
            let mut end = 0;

            for (x, col) in row.chars().enumerate() {
                let pos = Pos {
                    x: x as i64,
                    y: y as i64,
                };

                let cell = match col {
                    ' ' => continue,
                    '#' => Tile::Wall,
                    '.' => Tile::Floor,
                    _ => unreachable!(),
                };

                start.get_or_insert(x);
                end = x + 1;

                grid.insert(pos, cell);
            }

            size.get_or_insert_with(|| end - start.unwrap());
        }

        let mut path = Vec::new();
        let mut iter = iter.give().chars().peekable();
        let mut buffer = String::new();

        while let Some(next) = iter.peek() {
            if next.is_numeric() {
                buffer.clear();

                loop {
                    match iter.peek().copied() {
                        Some(next) if next.is_numeric() => {
                            buffer.push(next);
                            iter.next();
                        }
                        _ => break,
                    }
                }

                path.push(Move::Forward(i64::fro(&buffer)));
            } else {
                match iter.next() {
                    Some('L') => path.push(Move::Turn(false)),
                    Some('R') => path.push(Move::Turn(true)),
                    _ => unreachable!(),
                }
            }
        }

        Self {
            size: size.unwrap() as i64,
            grid,
            path,
        }
    }
}

impl Solution for MonkeyMap {
    fn one(self) -> i64 {
        dbg!(&self);
        let mut pos = Pos {
            y: 0,
            x: self.size * 2,
        };
        let mut dir = Dir::E;

        for r#move in &self.path {
            dbg!((pos, dir));
            match r#move {
                Move::Turn(clockwise) => dir.rotate_mut(*clockwise),
                Move::Forward(count) => {
                    for _ in 0..*count {
                        let next = self.wrap(pos, dir);

                        match self.grid[&next] {
                            Tile::Wall => break,
                            Tile::Floor => pos = next,
                        }
                    }
                }
            }
        }

        1000 * (pos.y + 1)
            + 4 * (pos.x + 1)
            + match dir {
                Dir::N => 3,
                Dir::S => 1,
                Dir::W => 2,
                Dir::E => 0,
            }
    }

    fn two(self) -> i64 {
        todo!()
    }
}

impl MonkeyMap {
    fn wrap(&self, _pos: Pos, _dir: Dir) -> Pos {
        todo!()
    }

    #[allow(unused)]
    fn region(&self, pos: Pos) -> u8 {
        if pos.y < self.size {
            0
        } else if pos.y < self.size * 2 {
            if pos.x < self.size {
                1
            } else if pos.x < self.size * 2 {
                2
            } else {
                3
            }
        } else if pos.x < self.size * 2 {
            4
        } else {
            5
        }
    }
}
