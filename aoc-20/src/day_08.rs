use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct HandheldHalting(Vec<(Op, i64)>);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Op {
    Nop,
    Acc,
    Jmp,
}

impl Fro for HandheldHalting {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let mut iter = line.trim().split_whitespace();
                let op = match iter.give() {
                    "nop" => Op::Nop,
                    "acc" => Op::Acc,
                    "jmp" => Op::Jmp,
                    _ => unreachable!(),
                };

                let arg = iter.give().trim_start_matches('+').to::<i64>();
                (op, arg)
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for HandheldHalting {
    fn one(self) -> i64 {
        let mut ip = 0;
        let mut acc = 0;
        let mut hit = HashSet::new();

        while hit.insert(ip) {
            match self.0[ip as usize] {
            | (Op::Nop, _) => ip += 1,
            | (Op::Acc, arg) => { acc += arg; ip += 1 },
            | (Op::Jmp, arg) => ip += arg,
            }
        }
        acc
    }

    fn two(self) -> i64 {
        for swap in 0..self.0.len() {
            let mut program = self.0.clone();

            if program[swap].0 == Op::Acc {
                continue;
            } else if program[swap].0 == Op::Nop {
                program[swap].0 = Op::Jmp;
            } else {
                program[swap].0 = Op::Nop;
            }

            let mut ip = 0;
            let mut acc = 0;
            let mut hit = HashSet::new();

            while hit.insert(ip) && ip as usize != program.len() {
                match program[ip as usize] {
                | (Op::Nop, _) => ip += 1,
                | (Op::Acc, arg) => { acc += arg; ip += 1 },
                | (Op::Jmp, arg) => ip += arg,
                }
            }

            if ip as usize == program.len() {
                return acc;
            }
        }
        unreachable!()
    }
}
