use std::collections::HashMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct DockingData(Vec<Op>);

#[derive(Clone, Debug)]
enum Op {
    Mask(String),
    Write {
        address: u64,
        value: u64,
    },
}

impl Fro for DockingData {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let mut iter = line.trim().split(" = ");
                let op = iter.give();
                if op == "mask" {
                    Op::Mask(iter.give().to_owned())
                } else {
                    let address = op
                        .trim_start_matches("mem[")
                        .trim_end_matches("]")
                        .to::<u64>();
                    let value = iter
                        .give()
                        .trim()
                        .to::<u64>();
                    Op::Write { address, value }
                }

            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for DockingData {
    fn one(self) -> i64 {
        let mut zeros = 0;
        let mut ones = 0;
        let mut memory = HashMap::new();

        for op in &self.0 {
            match op {
            | Op::Mask(mask) => {
                zeros = 0;
                ones = 0;
                let mut set = 0b1000_0000_0000_0000_0000_0000_0000_0000_0000;

                for bit in mask.chars() {
                    match bit {
                    | '0' => zeros |= set,
                    | '1' => ones |= set,
                    | 'X' => (),
                    | _ => unreachable!(),
                    }
                    set >>= 1;
                }
            }
            | Op::Write { address, value } => {
                memory.insert(address, (value | ones) & !zeros);
            }
            }
        }

        memory
            .values()
            .filter(|value| **value > 0)
            .sum::<u64>()
            as i64
    }

    fn two(self) -> i64 {
        let mut ones = 0;
        let mut float = Vec::new();
        let mut memory = HashMap::new();

        for op in &self.0 {
            match op {
            | Op::Mask(mask) => {
                ones = 0;
                float.clear();
                let mut set = 0b1000_0000_0000_0000_0000_0000_0000_0000_0000;

                for bit in mask.chars() {
                    match bit {
                    | '0' => (),
                    | '1' => ones |= set,
                    | 'X' => float.push(set),
                    | _ => unreachable!(),
                    }
                    set >>= 1;
                }
            }
            | Op::Write { address, value } => {
                for select in 0..1 << float.len() {
                    let mut floating = *address;

                    for bit in 0..float.len() {
                        if (1 << bit) & select == 0 {
                            floating &= !float[bit];
                        } else {
                            floating |= float[bit];
                        }
                    }

                    memory.insert(floating | ones, *value);
                }
            }
            }
        }

        memory
            .values()
            .filter(|value| **value > 0)
            .sum::<u64>()
            as i64
    }
}
