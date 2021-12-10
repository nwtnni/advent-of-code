use aoc::*;

#[derive(Clone, Debug)]
pub struct SyntaxScoring(Vec<Vec<char>>);

impl Fro for SyntaxScoring {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| line.chars().collect())
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for SyntaxScoring {
    fn one(self) -> i64 {
        self.0
            .iter()
            .filter_map(|line| corrupted(line))
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
        let mut scores = self.0
            .iter()
            .filter(|line| corrupted(line).is_none())
            .map(|line| complete(line))
            .collect::<Vec<_>>();

        scores.sort();
        scores[scores.len() / 2]
    }
}

fn corrupted(line: &[char]) -> Option<char> {
    let mut stack = Vec::new();
    for char in line {
        match char {
            '[' => stack.push('['),
            ']' if stack.last().copied() == Some('[') => { stack.pop(); }
            ']' => { return Some(']') }

            '(' => stack.push('('),
            ')' if stack.last().copied() == Some('(') => { stack.pop(); }
            ')' => { return Some(')') }

            '<' => stack.push('<'),
            '>' if stack.last().copied() == Some('<') => { stack.pop(); }
            '>' => { return Some('>') }

            '{' => stack.push('{'),
            '}' if stack.last().copied() == Some('{') => { stack.pop(); }
            '}' => { return Some('}') }

            _ => unreachable!(),
        }
    }
    None
}

fn complete(line: &[char]) -> i64 {
    let mut stack = Vec::new();
    for char in line {
        match char {
            '[' => stack.push('['),
            ']' => { stack.pop(); }

            '(' => stack.push('('),
            ')' => { stack.pop(); }

            '<' => stack.push('<'),
            '>' => { stack.pop(); }

            '{' => stack.push('{'),
            '}' => { stack.pop(); }

            _ => unreachable!(),
        }
    }

    let mut score = 0;
    for value in stack
        .into_iter()
        .rev()
        .map(|c| match c {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => unreachable!(),
        })
    {
        score *= 5;
        score += value;
    }

    score
}
