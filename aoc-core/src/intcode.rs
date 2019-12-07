use std::ops;
use std::str;

use crate::*;

#[derive(Clone, Debug)]
pub struct Program {
    here: i32,
    data: Vec<i32>,
    init: Vec<i32>,
}

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Pos,
    Val,
}

pub enum Yield {
    Halt,
    Step,
    Input(i32),
    Output(i32),
}

impl Program {
    pub fn reset(&mut self) {
        self.here = 0;
        self.data
            .iter_mut()
            .zip(&self.init)
            .for_each(|(d, i)| *d = *i);
    }

    pub fn run_nv(&mut self, noun: i32, verb: i32) -> i32 {
        self[1] = noun;
        self[2] = verb;
        loop {
            match self.step() {
            | Yield::Halt => return self[0],
            | _ => continue,
            }
        }
    }

    pub fn run_io<I: FnMut() -> i32, O: FnMut(i32)>(&mut self, mut input: I, mut output: O) {
        loop {
            match self.step() {
            | Yield::Halt => return,
            | Yield::Step => continue,
            | Yield::Input(dst) => self[dst] = input(),
            | Yield::Output(src) => output(src),
            }
        }
    }

    pub fn input(&mut self, input: i32) -> Option<()> {
        loop {
            match self.step() {
            | Yield::Halt => return None,
            | Yield::Step => continue,
            | Yield::Input(i) => return Some(self[i] = input),
            | Yield::Output(_) => unreachable!(),
            }
        }
    
    }

    pub fn output(&mut self) -> Option<i32> {
        loop {
            match self.step() {
            | Yield::Halt => return None,
            | Yield::Step => continue,
            | Yield::Input(_) => unreachable!(),
            | Yield::Output(i) => return Some(i),
            }
        }
    }

    pub fn pipe(&mut self, input: i32) -> Option<i32> {
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
            self[dst] = (lhs < rhs) as i32;
            self.here += 4;
        }
        | 8 => {
            let lhs = self.src(1);
            let rhs = self.src(2);
            let dst = self.dst(3);
            self[dst] = (lhs == rhs) as i32;
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

    fn op(&self) -> i32 {
        self[self.here] % 100
    }

    fn src(&self, parameter: i32) -> i32 {
        match self.mode(parameter) {
        | Mode::Pos => self[self[self.here + parameter]],
        | Mode::Val => self[self.here + parameter],
        }
    }

    fn dst(&self, parameter: i32) -> i32 {
        self[self.here + parameter]
    }

    fn mode(&self, parameter: i32) -> Mode {
        match self[self.here].digit(1 + parameter) {
        | 0 => Mode::Pos,
        | 1 => Mode::Val,
        | _ => unreachable!(),
        }
    }
}

impl Fro for Program {
    fn fro(input: &str) -> Self {
        let data = input.trim()
            .split(',')
            .map(|line| line.to::<i32>())
            .collect::<Vec<_>>();
        Program {
            here: 0,
            init: data.clone(),
            data,
        }
    }
}

impl ops::Index<i32> for Program {
    type Output = i32;
    fn index(&self, i: i32) -> &Self::Output {
        &self.data[i as usize]
    }
}

impl ops::IndexMut<i32> for Program {
    fn index_mut(&mut self, i: i32) -> &mut Self::Output {
        &mut self.data[i as usize]
    }
}
