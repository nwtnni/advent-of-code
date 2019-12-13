use std::collections::HashMap;
use std::collections::HashSet;

use aoc::*;

pub struct CarePackage(intcode::Program);

#[derive(Copy, Clone, Debug)]
enum Mode {
    X,
    XY(i64),
    ID(i64, i64),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Block {
    Empty = 0,
    Wall = 1,
    Block = 2,
    Paddle = 3,
    Ball = 4,
}

impl From<i64> for Block {
    fn from(id: i64) -> Self {
        match id {
        | 0 => Block::Empty,
        | 1 => Block::Wall,
        | 2 => Block::Block,
        | 3 => Block::Paddle,
        | 4 => Block::Ball,
        | _ => unreachable!(),
        }
    }
}

impl Fro for CarePackage {
    fn fro(input: &str) -> Self {
        CarePackage(intcode::Program::fro(input))
    }
}

impl Solution for CarePackage {
    fn one(mut self) -> i64 {
        let mut blocks = HashMap::<Pos, Block>::default();
        let mut mode = Mode::X;
        loop {
            use intcode::Yield::*;
            match (self.0.step(), mode)  {
            | (Step, _) => (),
            | (Halt, _) => break,
            | (Input(_), _) => unreachable!(),
            | (Output(x), Mode::X) => {
                mode = Mode::XY(x);
            }
            | (Output(y), Mode::XY(x)) => {
                mode = Mode::ID(x, y);
            }
            | (Output(id), Mode::ID(x, y)) => {
                mode = Mode::X;
                blocks.insert(Pos { x, y }, Block::from(id));
            }
            }
        }
        blocks.values()
            .filter(|block| **block == Block::Block)
            .count() as i64
    }

    fn two(mut self) -> i64 {

        self.0[0] = 2;

        let mut blocks = HashMap::<Pos, Block>::default();
        let mut ball = Pos { x: std::i64::MIN, y: std::i64::MAX };
        let mut paddle = HashSet::<Pos>::default();
        let mut mode = Mode::X;
        let mut score = 0;

        loop {
            use intcode::Yield::*;
            match (self.0.step(), mode)  {
            | (Step, _) => (),
            | (Halt, _) => break score,
            | (Input(d), _) => {
                let mid = paddle
                    .iter()
                    .map(|p| p.x)
                    .sum::<i64>()
                    / paddle.len() as i64;

                self.0[d] = if mid < ball.x { 1 } else if mid > ball.x { -1 } else { 0 };
            }
            | (Output(x), Mode::X) => {
                mode = Mode::XY(x);
            }
            | (Output(y), Mode::XY(x)) => {
                mode = Mode::ID(x, y);
            }
            | (Output(s), Mode::ID(-1, 0)) => {
                mode = Mode::X;
                score = s;
            }
            | (Output(0), Mode::ID(x, y)) => {
                mode = Mode::X;
                let pos = Pos { x, y };
                if let Some(Block::Paddle) = blocks.remove(&pos) {
                    paddle.remove(&pos);
                }
            }
            | (Output(id), Mode::ID(x, y)) => {
                mode = Mode::X;
                let pos = Pos { x, y };
                let block = Block::from(id);
                match block {
                | Block::Paddle => { paddle.insert(pos); },
                | Block::Ball => ball = pos,
                | _ => (),
                }
                blocks.insert(Pos { x, y }, Block::from(id));
            }
            }
        }
    }
}
