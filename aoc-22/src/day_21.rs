use std::collections::HashMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct MonkeyMath(HashMap<&'static str, Monkey>);

#[derive(Copy, Clone, Debug)]
enum Monkey {
    Op(Op, &'static str, &'static str),
    Value(i64),
}

#[derive(Copy, Clone, Debug)]
enum Op {
    Add,
    Sub,
    Mul,
    Div,
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
        let mut cache = HashMap::new();
        self.evaluate("root", &mut cache)
    }

    fn two(self) -> i64 {
        todo!()
    }
}

impl MonkeyMath {
    fn evaluate(&self, monkey: &'static str, cache: &mut HashMap<&'static str, i64>) -> i64 {
        if let Some(value) = cache.get(monkey) {
            return *value;
        }

        let value = match self.0[monkey] {
            Monkey::Value(value) => value,
            Monkey::Op(op, left, right) => {
                let left = self.evaluate(left, cache);
                let right = self.evaluate(right, cache);
                match op {
                    Op::Add => left + right,
                    Op::Sub => left - right,
                    Op::Mul => left * right,
                    Op::Div => left / right,
                }
            }
        };

        cache.insert(monkey, value);
        value
    }
}
