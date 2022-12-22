use std::collections::HashMap;
use std::fmt;

use aoc::*;

#[derive(Clone, Debug)]
pub struct MonkeyMath(HashMap<&'static str, Monkey>);

#[derive(Copy, Clone, Debug)]
enum Monkey {
    Op(Op, &'static str, &'static str),
    Value(i64),
}

#[derive(Clone, Debug)]
enum Tree {
    Node(Op, Box<Tree>, Box<Tree>),
    Leaf(i64),
    Input,
}

impl fmt::Display for Tree {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tree::Node(op, left, right) => write!(fmt, "({} {} {})", left, op, right),
            Tree::Leaf(value) => write!(fmt, "{}", value),
            Tree::Input => write!(fmt, "input"),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

impl fmt::Display for Op {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let op = match self {
            Op::Add => '+',
            Op::Sub => '-',
            Op::Mul => '*',
            Op::Div => '/',
        };
        write!(fmt, "{}", op)
    }
}

impl Op {
    fn evaluate(&self, left: i64, right: i64) -> i64 {
        match self {
            Op::Add => left + right,
            Op::Sub => left - right,
            Op::Mul => left * right,
            Op::Div => left / right,
        }
    }

    fn invert(&self) -> Self {
        match self {
            Op::Add => Op::Sub,
            Op::Sub => Op::Add,
            Op::Mul => Op::Div,
            Op::Div => Op::Mul,
        }
    }
}

impl Fro for MonkeyMath {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let (a, b) = line.split_once(": ").unwrap();

                let monkey = if let Some((b, c)) = b.split_once(" + ") {
                    Monkey::Op(Op::Add, b.leak(), c.leak())
                } else if let Some((b, c)) = b.split_once(" - ") {
                    Monkey::Op(Op::Sub, b.leak(), c.leak())
                } else if let Some((b, c)) = b.split_once(" * ") {
                    Monkey::Op(Op::Mul, b.leak(), c.leak())
                } else if let Some((b, c)) = b.split_once(" / ") {
                    Monkey::Op(Op::Div, b.leak(), c.leak())
                } else {
                    Monkey::Value(i64::fro(b))
                };

                (a.leak(), monkey)
            })
            .collect::<HashMap<_, _>>()
            .tap(Self)
    }
}

impl Solution for MonkeyMath {
    fn one(self) -> i64 {
        self.evaluate("root")
    }

    fn two(self) -> i64 {
        let (mut human, mut monkey) = match self.0["root"] {
            Monkey::Value(_) => unreachable!(),
            Monkey::Op(_, left, right) if self.contains(left, "humn") => {
                (self.convert(left), self.evaluate(right))
            }
            Monkey::Op(_, left, right) => (self.convert(right), self.evaluate(left)),
        };

        loop {
            match human {
                Tree::Node(op, left, right) => match (left.evaluate(), right.evaluate()) {
                    (None, None) => unreachable!(),
                    (Some(_), Some(_)) => unreachable!(),
                    (None, Some(right)) => {
                        human = *left;
                        monkey = op.invert().evaluate(monkey, right);
                    }
                    (Some(left), None) => {
                        human = *right;
                        match op {
                            Op::Add | Op::Mul => monkey = op.invert().evaluate(monkey, left),
                            Op::Sub => monkey = -monkey + left,
                            Op::Div => unreachable!(),
                        }
                    }
                },
                Tree::Leaf(_) => unreachable!(),
                Tree::Input => break monkey,
            }
        }
    }
}

impl MonkeyMath {
    fn evaluate(&self, monkey: &'static str) -> i64 {
        match self.0[monkey] {
            Monkey::Value(value) => value,
            Monkey::Op(op, left, right) => {
                let left = self.evaluate(left);
                let right = self.evaluate(right);
                match op {
                    Op::Add => left + right,
                    Op::Sub => left - right,
                    Op::Mul => left * right,
                    Op::Div => left / right,
                }
            }
        }
    }

    fn contains(&self, root: &'static str, monkey: &'static str) -> bool {
        root == monkey
            || match self.0[root] {
                Monkey::Op(_, left, right) => {
                    self.contains(left, monkey) || self.contains(right, monkey)
                }
                Monkey::Value(_) => false,
            }
    }

    fn convert(&self, monkey: &'static str) -> Tree {
        if monkey == "humn" {
            return Tree::Input;
        }

        match &self.0[monkey] {
            Monkey::Op(op, left, right) => Tree::Node(
                *op,
                Box::new(self.convert(left)),
                Box::new(self.convert(right)),
            ),
            Monkey::Value(value) => Tree::Leaf(*value),
        }
    }
}

impl Tree {
    fn evaluate(&self) -> Option<i64> {
        match self {
            Tree::Node(op, left, right) => {
                let left = left.evaluate()?;
                let right = right.evaluate()?;
                Some(op.evaluate(left, right))
            }
            Tree::Leaf(value) => Some(*value),
            Tree::Input => None,
        }
    }
}
