use aoc::*;

#[derive(Clone, Debug)]
pub struct Scratchcards(Vec<Card>);

#[derive(Clone, Debug)]
struct Card {
    winning: Vec<i64>,
    owned: Vec<i64>,
}

impl Card {
    fn matches(&self) -> usize {
        self.winning
            .iter()
            .map(|winner| self.owned.iter().filter(|value| *value == winner).count())
            .sum::<usize>()
    }

    fn score(&self) -> i64 {
        match self.matches() {
            0 => 0,
            n => 2i64.pow(n as u32 - 1),
        }
    }
}

impl Fro for Scratchcards {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let (_, card) = line.trim().split_once(": ").unwrap();
                let (winning, owned) = card.split_once(" | ").unwrap();
                Card {
                    winning: winning.split_whitespace().map(i64::fro).collect(),
                    owned: owned.split_whitespace().map(i64::fro).collect(),
                }
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for Scratchcards {
    fn one(self) -> i64 {
        self.0.iter().map(Card::score).sum()
    }

    fn two(self) -> i64 {
        self.0
            .iter()
            .enumerate()
            .scan(vec![1; self.0.len()], |copies, (i, card)| {
                let matches = card.matches();

                for j in i + 1..i + 1 + matches {
                    copies[j] += copies[i];
                }

                Some(copies[i])
            })
            .sum()
    }
}
