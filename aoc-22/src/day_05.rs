use aoc::*;

#[derive(Clone, Debug)]
pub struct SupplyStacks {
    state: Vec<Vec<char>>,
    moves: Vec<(i64, usize, usize)>,
}

impl Fro for SupplyStacks {
    fn fro(input: &str) -> Self {
        let mut iter = input.split("\n\n");

        let mut state = vec![vec![]; 9];

        for line in iter.next().unwrap().lines() {
            state.iter_mut().enumerate().for_each(|(index, stack)| {
                match line.chars().nth(index * 4 + 1) {
                    Some(letter) if letter.is_alphabetic() => stack.push(letter),
                    Some(_) => (),
                    None => (),
                }
            })
        }

        state.iter_mut().for_each(|stack| stack.reverse());

        let moves = iter
            .give()
            .split('\n')
            .map(|line| {
                let (a, b) = line
                    .trim_start_matches("move ")
                    .split_once(" from ")
                    .unwrap();
                let (b, c) = b.split_once(" to ").unwrap();
                (
                    a.parse().unwrap(),
                    b.parse::<usize>().unwrap() - 1,
                    c.parse::<usize>().unwrap() - 1,
                )
            })
            .collect();

        Self { state, moves }
    }
}

impl Solution for SupplyStacks {
    fn one(mut self) -> i64 {
        for (count, from, to) in &self.moves {
            for _ in 0..*count {
                let letter = self.state[*from].pop().unwrap();
                self.state[*to].push(letter);
            }
        }

        panic!("{}", self.message());
    }

    fn two(mut self) -> i64 {
        let mut buffer = Vec::new();

        for (count, from, to) in &self.moves {
            buffer.extend((0..*count).map(|_| self.state[*from].pop().unwrap()));
            buffer
                .drain(..)
                .rev()
                .for_each(|letter| self.state[*to].push(letter));
        }

        panic!("{}", self.message());
    }
}

impl SupplyStacks {
    fn message(&self) -> String {
        self.state
            .iter()
            .map(|stack| stack.last().copied())
            .map(Option::unwrap)
            .collect()
    }
}
