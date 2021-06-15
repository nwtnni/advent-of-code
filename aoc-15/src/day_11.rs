use std::convert::TryFrom as _;
use std::mem;

use aoc::*;

#[derive(Clone, Debug)]
pub struct CorporatePolicy(Password);

type Password = [u8; 8];

impl Fro for CorporatePolicy {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .bytes()
            .map(|byte| byte - b'a')
            .collect::<Vec<_>>()
            .tap(Password::try_from)
            .unwrap()
            .tap(Self)
    }
}

fn valid(password: &Password) -> bool {
    allowed(password) && increasing(password) && overlapping(password)
}

fn allowed(password: &Password) -> bool {
    password
        .iter()
        .any(|byte| ![b'i' - b'a', b'o' - b'a', b'l' - b'a'].contains(byte))
}

fn increasing(password: &Password) -> bool {
    password
        .windows(3)
        .any(|window| window[0] + 1 == window[1] && window[1] + 1 == window[2])
}

fn overlapping(password: &Password) -> bool {
    password
        .windows(2)
        .scan(false, |skip, window| {
            if mem::take(skip) {
                Some(0)
            } else if window[0] == window[1] {
                *skip = true;
                Some(1)
            } else {
                Some(0)
            }
        })
        .sum::<usize>()
        >= 2
}

fn increment(password: &mut Password) {
    password
        .iter_mut()
        .rev()
        .scan(1, |carry, byte| {
            let next = mem::take(carry) + *byte;
            if next < 26 {
                *byte = next;
                None
            } else {
                *carry = 1;
                *byte = next % 26;
                Some(())
            }
        })
        .count();
}

impl Solution for CorporatePolicy {
    fn one(mut self) -> i64 {
        while !valid(&self.0) {
            increment(&mut self.0);
        }

        println!(
            "{}",
            self.0
                .iter()
                .map(|byte| char::from(byte + b'a'))
                .collect::<String>()
        );

        0
    }

    fn two(mut self) -> i64 {
        while !valid(&self.0) {
            increment(&mut self.0);
        }

        increment(&mut self.0);

        while !valid(&self.0) {
            increment(&mut self.0);
        }

        println!(
            "{}",
            self.0
                .iter()
                .map(|byte| char::from(byte + b'a'))
                .collect::<String>()
        );

        0
    }
}
