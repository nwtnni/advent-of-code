use std::collections::HashMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct SomeAssemblyRequired(Vec<Instruction>);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Instruction {
    src: Gate,
    dst: String,
}

impl Instruction {
    fn evaluate(&self, assignment: &mut HashMap<String, i64>) -> bool {
        match self.src.evaluate(assignment) {
            None => return false,
            Some(value) => {
                assignment.insert(self.dst.clone(), value);
                true
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Gate {
    Unary(Unary, Operand),
    Binary(Operand, Binary, Operand),
}

impl Gate {
    fn evaluate(&self, assignment: &mut HashMap<String, i64>) -> Option<i64> {
        match self {
            Gate::Unary(unary, arg) => unary.evaluate(assignment, arg),
            Gate::Binary(lhs, binary, rhs) => binary.evaluate(assignment, lhs, rhs),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Unary {
    Identity,
    Not,
}

impl Unary {
    fn evaluate(&self, assignment: &mut HashMap<String, i64>, arg: &Operand) -> Option<i64> {
        let arg = arg.evaluate(assignment)?;
        match self {
            Unary::Identity => Some(arg),
            Unary::Not => Some(!arg),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Binary {
    And,
    Or,
    LShift,
    RShift,
}

impl Binary {
    fn evaluate(
        &self,
        assignment: &mut HashMap<String, i64>,
        lhs: &Operand,
        rhs: &Operand,
    ) -> Option<i64> {
        let lhs = lhs.evaluate(assignment)?;
        let rhs = rhs.evaluate(assignment)?;
        match self {
            Binary::And => Some(lhs & rhs),
            Binary::Or => Some(lhs | rhs),
            Binary::LShift => Some(lhs << rhs),
            Binary::RShift => Some(lhs >> rhs),
        }
    }
}

impl Fro for Instruction {
    fn fro(input: &str) -> Self {
        let mut iter = input.split(" -> ");

        let src = iter.give();
        let dst = iter.give().tap(String::fro);

        let gate = if let Some((_, reg)) = src.split_once("NOT ") {
            Gate::Unary(Unary::Not, Operand::fro(reg))
        } else if let Some((lhs, rhs)) = src.split_once(" AND ") {
            Gate::Binary(Operand::fro(lhs), Binary::And, Operand::fro(rhs))
        } else if let Some((lhs, rhs)) = src.split_once(" OR ") {
            Gate::Binary(Operand::fro(lhs), Binary::Or, Operand::fro(rhs))
        } else if let Some((lhs, rhs)) = src.split_once(" LSHIFT ") {
            Gate::Binary(Operand::fro(lhs), Binary::LShift, Operand::fro(rhs))
        } else if let Some((lhs, rhs)) = src.split_once(" RSHIFT ") {
            Gate::Binary(Operand::fro(lhs), Binary::RShift, Operand::fro(rhs))
        } else {
            Gate::Unary(Unary::Identity, Operand::fro(src))
        };

        Instruction { src: gate, dst }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Operand {
    Reg(String),
    Const(i64),
}

impl Operand {
    fn evaluate(&self, assignment: &mut HashMap<String, i64>) -> Option<i64> {
        match self {
            Operand::Reg(reg) => assignment.get(reg).copied(),
            Operand::Const(value) => Some(*value),
        }
    }
}

impl Fro for Operand {
    fn fro(input: &str) -> Self {
        if input.chars().all(|char| char.is_ascii_alphabetic()) {
            Operand::Reg(String::fro(input))
        } else {
            Operand::Const(i64::fro(input))
        }
    }
}

impl Fro for SomeAssemblyRequired {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(Instruction::fro)
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for SomeAssemblyRequired {
    fn one(mut self) -> i64 {
        let mut assignment = HashMap::new();
        while !self.0.is_empty() {
            self.0
                .retain(|instruction| !instruction.evaluate(&mut assignment));
        }
        assignment["a"]
    }

    fn two(mut self) -> i64 {
        let a = self.clone().one();

        self.0.iter_mut().for_each(|instruction| {
            if instruction.dst.as_str() == "b" {
                instruction.src = Gate::Unary(Unary::Identity, Operand::Const(a))
            }
        });

        self.one()
    }
}
