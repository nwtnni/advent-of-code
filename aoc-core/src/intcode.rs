use std::ops;
use std::str;

use crate::*;

#[derive(Clone, Debug)]
pub struct Program {
    here: i64,
    data: Vec<i64>,
    init: Vec<i64>,
}

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Pos,
    Val,
}

pub enum Yield {
    Halt,
    Step,
    Input(i64),
    Output(i64),
}

impl Program {
    pub fn reset(&mut self) {
        self.here = 0;
        self.data
            .iter_mut()
            .zip(&self.init)
            .for_each(|(d, i)| *d = *i);
    }

    pub fn run_nv(&mut self, noun: i64, verb: i64) -> i64 {
        self[1] = noun;
        self[2] = verb;
        loop {
            match self.step() {
            | Yield::Halt => return self[0],
            | _ => continue,
            }
        }
    }

    pub fn run_io<I: FnMut() -> i64, O: FnMut(i64)>(&mut self, mut input: I, mut output: O) {
        loop {
            match self.step() {
            | Yield::Halt => return,
            | Yield::Step => continue,
            | Yield::Input(dst) => self[dst] = input(),
            | Yield::Output(src) => output(src),
            }
        }
    }

    pub fn input(&mut self, input: i64) -> Option<()> {
        loop {
            match self.step() {
            | Yield::Halt => return None,
            | Yield::Step => continue,
            | Yield::Input(i) => return Some(self[i] = input),
            | Yield::Output(_) => unreachable!(),
            }
        }
    
    }

    pub fn output(&mut self) -> Option<i64> {
        loop {
            match self.step() {
            | Yield::Halt => return None,
            | Yield::Step => continue,
            | Yield::Input(_) => unreachable!(),
            | Yield::Output(i) => return Some(i),
            }
        }
    }

    pub fn pipe(&mut self, input: i64) -> Option<i64> {
        self.input(input)?;
        self.output()
    }

    // Execute current instruction
    pub fn step(&mut self) -> Yield {
        match self.op() {
        | 1 => {
            let lhs = self.src(1);
            let rhs = self.src(2);
            let dst = self.dst(3);
            self[dst] = lhs + rhs;
            self.here += 4;
        }
        | 2 => {
            let lhs = self.src(1);
            let rhs = self.src(2);
            let dst = self.dst(3);
            self[dst] = lhs * rhs;
            self.here += 4;
        }
        | 3 => {
            let dst = self.dst(1);
            self.here += 2;
            return Yield::Input(dst);
        }
        | 4 => {
            let src = self.src(1);
            self.here += 2;
            return Yield::Output(src);
        }
        | 5 => {
            let cond = self.src(1);
            self.here = if cond != 0 { self.src(2) } else { self.here + 3 };
        }
        | 6 => {
            let cond = self.src(1);
            self.here = if cond == 0 { self.src(2) } else { self.here + 3 };
        }
        | 7 => {
            let lhs = self.src(1);
            let rhs = self.src(2);
            let dst = self.dst(3);
            self[dst] = (lhs < rhs) as i64;
            self.here += 4;
        }
        | 8 => {
            let lhs = self.src(1);
            let rhs = self.src(2);
            let dst = self.dst(3);
            self[dst] = (lhs == rhs) as i64;
            self.here += 4;
        }
        | 99 => {
            return Yield::Halt;
        }
        | _ => {
            unimplemented!()
        }
        }
        Yield::Step
    }

    fn op(&self) -> i64 {
        self[self.here] % 100
    }

    fn src(&self, parameter: i64) -> i64 {
        match self.mode(parameter) {
        | Mode::Pos => self[self[self.here + parameter]],
        | Mode::Val => self[self.here + parameter],
        }
    }

    fn dst(&self, parameter: i64) -> i64 {
        self[self.here + parameter]
    }

    fn mode(&self, parameter: i64) -> Mode {
        match self[self.here].digit(1 + parameter) {
        | 0 => Mode::Pos,
        | 1 => Mode::Val,
        | _ => unreachable!(),
        }
    }
}

impl Fro for Program {
    fn fro(input: &str) -> Self {
        let mut data = input.trim()
            .split(',')
            .map(|line| line.to::<i64>())
            .collect::<Vec<_>>();
        for _ in 0..1000 {
            data.push(0);
        }
        Program {
            here: 0,
            init: data.clone(),
            data,
        }
    }
}

impl ops::Index<i64> for Program {
    type Output = i64;
    fn index(&self, i: i64) -> &Self::Output {
        &self.data[i as usize]
    }
}

impl ops::IndexMut<i64> for Program {
    fn index_mut(&mut self, i: i64) -> &mut Self::Output {
        &mut self.data[i as usize]
    }
}
