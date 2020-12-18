use std::iter;
use std::vec;

use aoc::*;

#[derive(Clone, Debug)]
pub struct OperationOrder(Vec<Vec<Exp>>);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Exp {
    Add,
    Mul,
    LParen,
    RParen,
    Int(i64),
}

impl Fro for Exp {
    fn fro(input: &str) -> Self {
        match input {
            "+" => Exp::Add,
            "*" => Exp::Mul,
            "(" => Exp::LParen,
            ")" => Exp::RParen,
            value => Exp::Int(i64::fro(value)),
        }
    }
}

impl Fro for OperationOrder {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                line.replace("(", "( ")
                    .replace(")", " )")
                    .trim()
                    .split_whitespace()
                    .map(Exp::fro)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for OperationOrder {
    fn one(self) -> i64 {
        self.0
            .into_iter()
            .map(|equation| recurse_one(&mut equation.into_iter().peekable()))
            .sum::<i64>()
    }

    fn two(self) -> i64 {
        self.0
            .into_iter()
            .map(|equation| recurse_two(&mut equation.into_iter().peekable()))
            .sum::<i64>()
    }
}

fn recurse_one(exp: &mut iter::Peekable<vec::IntoIter<Exp>>) -> i64 {
    let mut value = match exp.next() {
        None => return 0,
        Some(Exp::Int(int)) => int,
        Some(Exp::LParen) => {
            let subtree = recurse_one(exp);
            assert_eq!(exp.next(), Some(Exp::RParen));
            subtree
        }
        _ => unreachable!(),
    };

    loop {
        match exp.peek().copied() {
            Some(Exp::Add) => {
                exp.next();
                    match exp.next() {
                    Some(Exp::Int(int)) => value += int,
                    Some(Exp::LParen) => {
                        value += recurse_one(exp);
                        assert_eq!(exp.next(), Some(Exp::RParen));
                    }
                    _ => unreachable!(),
                }
            }
            Some(Exp::Mul) => {
                exp.next();
                match exp.next() {
                    Some(Exp::Int(int)) => value *= int,
                    Some(Exp::LParen) => {
                        value *= recurse_one(exp);
                        assert_eq!(exp.next(), Some(Exp::RParen));
                    }
                    _ => unreachable!(),
                }
            }
            Some(Exp::RParen) | None => return value,
            _ => unreachable!(),
        }
    }
}

fn recurse_two(exp: &mut iter::Peekable<vec::IntoIter<Exp>>) -> i64 {
    let mut value = match exp.next() {
        None => return 0,
        Some(Exp::Int(int)) => int,
        Some(Exp::LParen) => {
            let subtree = recurse_two(exp);
            assert_eq!(exp.next(), Some(Exp::RParen));
            subtree
        }
        _ => unreachable!(),
    };

    loop {
        match exp.peek().copied() {
            Some(Exp::Add) => {
                exp.next();
                match exp.next() {
                    Some(Exp::Int(int)) => value += int,
                    Some(Exp::LParen) => {
                        value += recurse_two(exp);
                        assert_eq!(exp.next(), Some(Exp::RParen));
                    }
                    _ => unreachable!(),
                }
            }
            Some(Exp::Mul) => {
                exp.next();
                value *= recurse_two(exp);
            }
            Some(Exp::RParen) | None => return value,
            _ => unreachable!(),
        }
    }
}
