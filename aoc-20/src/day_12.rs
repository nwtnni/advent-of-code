use aoc::*;

#[derive(Clone, Debug)]
pub struct RainRisk(Vec<(Move, i64)>);

#[derive(Copy, Clone, Debug)]
enum Move {
    N,
    S,
    E,
    W,
    L,
    R,
    F,
}

impl Fro for RainRisk {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let r#move = match &line[0..1] {
                    "N" => Move::N,
                    "S" => Move::S,
                    "E" => Move::E,
                    "W" => Move::W,
                    "L" => Move::L,
                    "R" => Move::R,
                    "F" => Move::F,
                    _ => unreachable!(),
                };

                let dist = line[1..].trim().to::<i64>();
                (r#move, dist)
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for RainRisk {
    fn one(self) -> i64 {
        let dirs = [Dir::E, Dir::N, Dir::W, Dir::S];

        let mut dir = 0;
        let mut pos = Pos::default();

        for (r#move, dist) in self.0 {
            match (r#move, dirs[dir]) {
                (Move::N, _) | (Move::F, Dir::N) => {
                    pos = Pos {
                        x: pos.x,
                        y: pos.y + dist,
                    }
                }
                (Move::S, _) | (Move::F, Dir::S) => {
                    pos = Pos {
                        x: pos.x,
                        y: pos.y - dist,
                    }
                }
                (Move::E, _) | (Move::F, Dir::E) => {
                    pos = Pos {
                        x: pos.x + dist,
                        y: pos.y,
                    }
                }
                (Move::W, _) | (Move::F, Dir::W) => {
                    pos = Pos {
                        x: pos.x - dist,
                        y: pos.y,
                    }
                }
                (Move::L, _) => dir = (dir + dist as usize / 90) % 4,
                (Move::R, _) => dir = (dir + 4 - dist as usize / 90) % 4,
            }
        }

        pos.x.abs() + pos.y.abs()
    }

    fn two(self) -> i64 {
        let mut pos = Pos::default();
        let mut way = Pos { x: 10, y: 1 };

        for (r#move, dist) in self.0 {
            match r#move {
                Move::N => {
                    way = Pos {
                        x: way.x,
                        y: way.y + dist,
                    }
                }
                Move::S => {
                    way = Pos {
                        x: way.x,
                        y: way.y - dist,
                    }
                }
                Move::E => {
                    way = Pos {
                        x: way.x + dist,
                        y: way.y,
                    }
                }
                Move::W => {
                    way = Pos {
                        x: way.x - dist,
                        y: way.y,
                    }
                }
                Move::L => rotate_way(&mut way, dist),
                Move::R => rotate_way(&mut way, -dist),
                Move::F => {
                    pos = Pos {
                        x: pos.x + way.x * dist,
                        y: pos.y + way.y * dist,
                    }
                }
            }
        }

        pos.x.abs() + pos.y.abs()
    }
}

/// ```text
/// [ a b   [ x   = (ax + by)i + (cx + dy)j
///   c d ]   y ]
///
///    0: [ 1 0       xi + yj
///         0 1 ]
///
///  τ/4: [ 0 -1     -yi + xj
///         1  0 ]
///
///  τ/2: [ -1  0    -xi - yj
///          0 -1 ]
///
/// 3τ/4: [  0  1     yi - xj
///         -1  0 ]
/// ```
fn rotate_way(way: &mut Pos, by: i64) {
    match by / 90 {
        0 => (),
        1 | -3 => {
            *way = Pos {
                x: -way.y,
                y: way.x,
            }
        }
        2 | -2 => {
            *way = Pos {
                x: -way.x,
                y: -way.y,
            }
        }
        3 | -1 => {
            *way = Pos {
                x: way.y,
                y: -way.x,
            }
        }
        _ => unreachable!(),
    }
}
