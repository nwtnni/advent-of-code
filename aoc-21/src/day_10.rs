use std::iter;
use std::vec;

use aoc::*;

#[derive(Clone, Debug)]
pub struct SyntaxScoring(Vec<String>);

enum Syntax {
    Corrupt(char),
    Complete(iter::Rev<vec::IntoIter<char>>),
}

impl Fro for SyntaxScoring {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(String::from)
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for SyntaxScoring {
    fn one(self) -> i64 {
        self.0
            .iter()
            .filter_map(|line| match check(line) {
                Syntax::Corrupt(char) => Some(char),
                Syntax::Complete(_) => None,
            })
            .map(|char| match char {
                ')' => 3,
                ']' => 57,
                '}' => 1197,
                '>' => 25137,
                _ => unreachable!(),
            })
            .sum()
    }

    fn two(self) -> i64 {
        let scores = self
            .0
            .iter()
            .filter_map(|line| match check(line) {
                Syntax::Corrupt(_) => None,
                Syntax::Complete(completion) => Some(completion),
            })
            .map(|completion| {
                completion.fold(0, |score, char| {
                    score * 5
                        + match char {
                            '(' => 1,
                            '[' => 2,
                            '{' => 3,
                            '<' => 4,
                            _ => unreachable!(),
                        }
                })
            })
            .collect::<Vec<_>>()
            .tap_mut(|scores| scores.sort());

        scores[scores.len() / 2]
    }
}

fn check(line: &str) -> Syntax {
    let mut stack = Vec::new();

    for char in line.chars() {
        match char {
            '[' => stack.push('['),
            ']' if stack.last().copied() == Some('[') => drop(stack.pop()),
            ']' => return Syntax::Corrupt(']'),

            '(' => stack.push('('),
            ')' if stack.last().copied() == Some('(') => drop(stack.pop()),
            ')' => return Syntax::Corrupt(')'),

            '<' => stack.push('<'),
            '>' if stack.last().copied() == Some('<') => drop(stack.pop()),
            '>' => return Syntax::Corrupt('>'),

            '{' => stack.push('{'),
            '}' if stack.last().copied() == Some('{') => drop(stack.pop()),
            '}' => return Syntax::Corrupt('}'),

            _ => unreachable!(),
        }
    }

    stack.into_iter().rev().tap(Syntax::Complete)
}
