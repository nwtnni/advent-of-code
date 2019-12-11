use std::collections::HashMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct Placeholder {
    pos: Pos,
    dir: Dir,
    paint: HashMap<Pos, Paint>,
    program: intcode::Program,
}

fn turn(dir: Dir, way: i64) -> Dir {
    match (dir, way) {
    | (Dir::N, 0) => Dir::W,
    | (Dir::N, 1) => Dir::E,

    | (Dir::S, 0) => Dir::E,
    | (Dir::S, 1) => Dir::W,

    | (Dir::W, 0) => Dir::S,
    | (Dir::W, 1) => Dir::N,

    | (Dir::E, 0) => Dir::N,
    | (Dir::E, 1) => Dir::S,
    | _ => unreachable!(),
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Paint {
    B = 0,
    W = 1,
}

impl Fro for Placeholder {
    fn fro(input: &str) -> Self {
        Placeholder {
            pos: Pos::default(),
            dir: Dir::N,
            paint: HashMap::default(),
            program: intcode::Program::fro(input),
        }
    }
}

impl Solution for Placeholder {
    fn one(mut self) -> i64 {
        let mut first = true;
        loop {
            use intcode::Yield::*;
            match self.program.step() {
            | Halt => return self.paint.len() as i64,
            | Input(i) => {
                let p = *self.paint.get(&self.pos).unwrap_or(&Paint::B) as i64;
                self.program[i] = p;
            }
            | Output(i) if first => {
                first = !first;
                self.paint.insert(self.pos, if i == 0 { Paint::B } else { Paint::W });
            }
            | Output(i) => {
                first = !first;
                self.dir = turn(self.dir, i);
                self.pos.shift_mut(self.dir);
            }
            | Step => (),
            }
        }
    }

    fn two(mut self) -> i64 {
        let mut first = true;
        loop {
            use intcode::Yield::*;
            match self.program.step() {
            | Halt => break,
            | Input(i) => {
                const P: Pos = Pos { x: 0, y: 0 };
                let p = if self.pos == P {
                    *self.paint.get(&self.pos).unwrap_or(&Paint::W) as i64
                } else {
                    *self.paint.get(&self.pos).unwrap_or(&Paint::B) as i64
                };
                self.program[i] = p;
            }
            | Output(i) if first => {
                first = !first;
                self.paint.insert(self.pos, if i == 0 { Paint::B } else { Paint::W });
            }
            | Output(i) => {
                first = !first;
                self.dir = turn(self.dir, i);
                self.pos.shift_mut(self.dir);
            }
            | Step => (),
            }
        }

        for y in (-100..100).rev() {
            for x in -100..100 {
                match self.paint.get(&Pos { x, y }) {
                | Some(Paint::W) => print!("█"),
                | _ => print!(" "),
                }
            }
            println!("");
        }

        0
    }
}
