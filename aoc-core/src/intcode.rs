use std::fmt;
use std::iter;
use std::ops;
use std::str;

use crate::*;

pub struct Program {
    data: Vec<i32>,
    iter: Box<dyn Iterator<Item = i32>>,
}

impl Clone for Program {
    fn clone(&self) -> Self {
        Program {
            data: self.data.clone(),
            iter: Box::new(iter::empty()),
        }
    }
}

impl fmt::Debug for Program {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Program")
            .field("data", &self.data)
            .finish()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Mode {
    Pos,
    Val,
}

impl Program {
    pub fn run_with_noun_verb(mut self, noun: i32, verb: i32) -> i32 {
        self[1] = noun;
        self[2] = verb;
        self.run()
    }

    pub fn run_with_input<I: Iterator<Item = i32> + 'static>(mut self, input: I) -> i32 {
        self.iter = Box::new(input);
        self.run()
    }

    pub fn run(mut self) -> i32 {
        let mut ip = 0;
        while let Some(next) = self.step(ip) {
            ip = next;
        }
        self[0]
    }

    // Execute current instruction and return next instruction pointer
    fn step(&mut self, ip: i32) -> Option<i32> {
        match self[ip] % 10 {
        | 1 => {
            let lhs = self.src(ip, 1);
            let rhs = self.src(ip, 2);
            let dst = self.dst(ip, 3);
            self[dst] = lhs + rhs;
            Some(ip + 4)
        }
        | 2 => {
            let lhs = self.src(ip, 1);
            let rhs = self.src(ip, 2);
            let dst = self.dst(ip, 3);
            self[dst] = lhs * rhs;
            Some(ip + 4)
        }
        | 3 => {
            let dst = self.dst(ip, 1);
            self[dst] = self.iter.give();
            Some(ip + 2)
        }
        | 4 => {
            let src = self.src(ip, 1);
            println!("{}", src);
            Some(ip + 2)
        }
        | 5 => {
            let brc = self.src(ip, 1);
            let jmp = self.src(ip, 2);
            if brc != 0 {
                Some(jmp)
            } else {
                Some(ip + 3)
            }
        }
        | 6 => {
            let brc = self.src(ip, 1);
            let jmp = self.src(ip, 2);
            if brc == 0 {
                Some(jmp)
            } else {
                Some(ip + 3)
            }
        }
        | 7 => {
            let lhs = self.src(ip, 1);
            let rhs = self.src(ip, 2);
            let dst = self.dst(ip, 3);
            if lhs < rhs {
                self[dst] = 1;
            } else {
                self[dst] = 0;
            }
            Some(ip + 4)
        }
        | 8 => {
            let lhs = self.src(ip, 1);
            let rhs = self.src(ip, 2);
            let dst = self.dst(ip, 3);
            if lhs == rhs {
                self[dst] = 1;
            } else {
                self[dst] = 0;
            }
            Some(ip + 4)
        }
        | 99 => {
            None
        }
        | _ => {
            unimplemented!()
        }
        }
    }

    fn src(&self, ip: i32, parameter: i32) -> i32 {
        match self.mode(ip, parameter) {
        | Mode::Pos => self[self[ip + parameter]],
        | Mode::Val => self[ip + parameter],
        }
    }

    fn dst(&self, ip: i32, parameter: i32) -> i32 {
        self[ip + parameter]
    }

    fn mode(&self, ip: i32, parameter: i32) -> Mode {
        let mut n = self[ip] / 10;
        for _ in 0..parameter {
            n /= 10;
        }
        match n % 10 {
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
            data,
            iter: Box::new(iter::empty()),
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
