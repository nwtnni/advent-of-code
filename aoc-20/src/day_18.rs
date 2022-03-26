use std::iter;
use std::ops;
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

fn recurse_one(equation: &mut iter::Peekable<vec::IntoIter<Exp>>) -> i64 {
    let mut value = match equation.next() {
        Some(Exp::Int(value)) => value,
        Some(Exp::LParen) => {
            let value = recurse_one(equation);
            assert_eq!(equation.next(), Some(Exp::RParen));
            value
        }
        _ => unreachable!(),
    };

    loop {
        let apply: fn(i64, i64) -> i64 = match equation.peek().copied() {
            Some(Exp::Add) => ops::Add::add,
            Some(Exp::Mul) => ops::Mul::mul,
            _ => return value,
        };

        equation.next();

        match equation.next() {
            Some(Exp::Int(int)) => value = apply(value, int),
            Some(Exp::LParen) => {
                value = apply(value, recurse_one(equation));
                assert_eq!(equation.next(), Some(Exp::RParen));
            }
            _ => unreachable!(),
        }
    }
}

fn recurse_two(equation: &mut iter::Peekable<vec::IntoIter<Exp>>) -> i64 {
    let mut value = match equation.next() {
        Some(Exp::Int(value)) => value,
        Some(Exp::LParen) => {
            let value = recurse_two(equation);
            assert_eq!(equation.next(), Some(Exp::RParen));
            value
        }
        _ => unreachable!(),
    };

    loop {
        match equation.peek().copied() {
            Some(Exp::Add) => {
                equation.next();
                match equation.next() {
                    Some(Exp::Int(int)) => value += int,
                    Some(Exp::LParen) => {
                        value += recurse_two(equation);
                        assert_eq!(equation.next(), Some(Exp::RParen));
                    }
                    _ => unreachable!(),
                }
            }
            Some(Exp::Mul) => {
                equation.next();
                value *= recurse_two(equation);
            }
            _ => return value,
        }
    }
}
