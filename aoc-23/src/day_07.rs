use std::collections::HashMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct CamelCards(Vec<(Hand, i64)>);

#[derive(Clone, Debug, PartialEq, Eq)]
struct Hand(Vec<Card>);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    CJ,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    CT,
    CQ,
    CK,
    CA,
}

impl Card {
    fn all() -> &'static [Card] {
        &[
            Card::C2,
            Card::C3,
            Card::C4,
            Card::C5,
            Card::C6,
            Card::C7,
            Card::C8,
            Card::C9,
            Card::CT,
            Card::CQ,
            Card::CK,
            Card::CA,
        ]
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Kind {
    High,
    One,
    Two,
    Three,
    Full,
    Four,
    Five,
}

impl Hand {
    fn kind(&self) -> Kind {
        let mut m = HashMap::new();
        self.0.iter().for_each(|c| *m.entry(c).or_insert(0) += 1);
        if m.len() == 1 {
            Kind::Five
        } else if m.len() == 2 {
            match m.values().max().unwrap() {
                3 => Kind::Full,
                4 => Kind::Four,
                _ => unreachable!(),
            }
        } else if m.len() == 3 {
            match m.values().max().unwrap() {
                3 => Kind::Three,
                2 => Kind::Two,
                _ => unreachable!(),
            }
        } else if m.len() == 4 {
            Kind::One
        } else {
            Kind::High
        }
    }

    fn kind_joker(&self) -> Kind {
        let mut m = HashMap::new();
        let mut max = Kind::High;

        for sub in Card::all() {
            m.clear();
            self.0
                .iter()
                .for_each(|c| *m.entry(if let Card::CJ = c { sub } else { c }).or_insert(0) += 1);
            let kind = if m.len() == 1 {
                Kind::Five
            } else if m.len() == 2 {
                match m.values().max().unwrap() {
                    3 => Kind::Full,
                    4 => Kind::Four,
                    _ => unreachable!(),
                }
            } else if m.len() == 3 {
                match m.values().max().unwrap() {
                    3 => Kind::Three,
                    2 => Kind::Two,
                    _ => unreachable!(),
                }
            } else if m.len() == 4 {
                Kind::One
            } else {
                Kind::High
            };
            max = std::cmp::max(max, kind);
        }

        max
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.kind()
            .cmp(&other.kind())
            .then_with(|| self.0.cmp(&other.0))
    }
}

impl Fro for CamelCards {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let (a, b) = line.split_once(' ').unwrap();
                let a = a
                    .chars()
                    .map(|c| {
                        use Card::*;
                        match c {
                            '2' => C2,
                            '3' => C3,
                            '4' => C4,
                            '5' => C5,
                            '6' => C6,
                            '7' => C7,
                            '8' => C8,
                            '9' => C9,
                            'T' => CT,
                            'J' => CJ,
                            'Q' => CQ,
                            'K' => CK,
                            'A' => CA,
                            _ => unreachable!(),
                        }
                    })
                    .collect();

                (Hand(a), b.to::<i64>())
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for CamelCards {
    fn one(mut self) -> i64 {
        self.0.sort();
        self.0
            .iter()
            .enumerate()
            .map(|(i, (_, j))| (i as i64 + 1) * *j)
            .sum()
    }

    fn two(mut self) -> i64 {
        self.0
            .sort_by_cached_key(|(h, _)| (h.kind_joker(), h.0.clone()));

        self.0
            .iter()
            .enumerate()
            .map(|(i, (_, j))| (i as i64 + 1) * *j)
            .sum()
    }
}
