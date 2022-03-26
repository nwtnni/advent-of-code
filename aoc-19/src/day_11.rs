use std::collections::HashMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct SpacePolice {
    pos: Pos,
    dir: Dir,
    paint: HashMap<Pos, Paint>,
    program: intcode::Program,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Paint {
    B = 0,
    W = 1,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Mode {
    Paint,
    Turn,
}

impl Mode {
    fn toggle(&mut self) {
        *self = match self {
            Mode::Paint => Mode::Turn,
            Mode::Turn => Mode::Paint,
        };
    }
}

fn turn(dir: Dir, way: i64) -> Dir {
    match (dir, way) {
        (Dir::N, 0) => Dir::W,
        (Dir::N, 1) => Dir::E,
        (Dir::S, 0) => Dir::E,
        (Dir::S, 1) => Dir::W,
        (Dir::W, 0) => Dir::S,
        (Dir::W, 1) => Dir::N,
        (Dir::E, 0) => Dir::N,
        (Dir::E, 1) => Dir::S,
        _ => unreachable!(),
    }
}

impl Fro for SpacePolice {
    fn fro(input: &str) -> Self {
        SpacePolice {
            pos: Pos::default(),
            dir: Dir::N,
            paint: HashMap::default(),
            program: intcode::Program::fro(input),
        }
    }
}

impl SpacePolice {
    fn run(&mut self, default: Paint) {
        let mut mode = Mode::Paint;
        loop {
            use intcode::Yield::*;
            match (self.program.step(), mode) {
                (Halt, _) => break,
                (Input(i), _) => {
                    let paint = self.paint.get(&self.pos).unwrap_or(&default);
                    self.program[i] = *paint as i64;
                }
                (Output(i), Mode::Paint) => {
                    self.paint
                        .insert(self.pos, if i == 0 { Paint::B } else { Paint::W });
                    mode.toggle();
                }
                (Output(i), Mode::Turn) => {
                    self.dir = turn(self.dir, i);
                    self.pos.shift_mut(self.dir);
                    mode.toggle();
                }
                (Step, _) => (),
            }
        }
    }
}

impl Solution for SpacePolice {
    fn one(mut self) -> i64 {
        self.run(Paint::B);
        self.paint.len() as i64
    }

    fn two(mut self) -> i64 {
        self.run(Paint::W);
        for y in (-100..100).rev() {
            for x in -100..100 {
                match self.paint.get(&Pos { x, y }) {
                    Some(Paint::W) => print!("█"),
                    _ => print!(" "),
                }
            }
            println!();
        }
        0
    }
}
