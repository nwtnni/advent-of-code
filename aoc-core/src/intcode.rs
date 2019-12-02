use std::ops;
use std::str;

#[derive(Clone, Debug)]
pub struct Program(Vec<i32>);

impl Program {
    pub fn run(&mut self, noun: i32, verb: i32) -> i32 {
        self[1] = noun;
        self[2] = verb;
        let mut ip = 0;
        while let Some(next) = self.step(ip) {
            ip = next;
        }
        self[0]
    }

    // Execute current instruction and return next instruction pointer
    fn step(&mut self, ip: i32) -> Option<i32> {
        match self[ip] {
        | 1 => {
            let lhs = self[ip + 1];
            let rhs = self[ip + 2];
            let dst = self[ip + 3];
            self[dst] = self[lhs] + self[rhs];
            Some(ip + 4)
        }
        | 2 => {
            let lhs = self[ip + 1];
            let rhs = self[ip + 2];
            let dst = self[ip + 3];
            self[dst] = self[lhs] * self[rhs];
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
}

impl str::FromStr for Program {
    type Err = crate::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        input.trim()
            .split(',')
            .map(|line| line.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()
            .map(Program)
            .map_err(crate::Error::InvalidInt)
    }
}

impl ops::Index<i32> for Program {
    type Output = i32;
    fn index(&self, i: i32) -> &Self::Output {
        &self.0[i as usize]
    }
}

impl ops::IndexMut<i32> for Program {
    fn index_mut(&mut self, i: i32) -> &mut Self::Output {
        &mut self.0[i as usize]
    }
}
