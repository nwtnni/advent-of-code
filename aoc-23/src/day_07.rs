use std::collections::HashMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct CamelCards(Vec<(Hand, i64)>);

#[derive(Clone, Debug, PartialEq, Eq)]
struct Hand(Vec<Card>);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Card {
    J,
    C2,
    C3,
    C4,
    C5,
    C6,
    C7,
    C8,
    C9,
    CT,
    CJ,
    CQ,
    CK,
    CA,
}

const SUBSTITUE: &[Card] = &[
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
];

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
    fn kind(&self, cache: &mut HashMap<Card, u8>) -> Kind {
        Self::kind_inner(cache, self.0.iter().copied())
    }

    fn kind_joker(&self, cache: &mut HashMap<Card, u8>) -> Kind {
        SUBSTITUE
            .iter()
            .map(|card| {
                Self::kind_inner(
                    cache,
                    self.0.iter().map(|original| match original {
                        Card::CJ => *card,
                        other => *other,
                    }),
                )
            })
            .max()
            .unwrap()
    }

    fn kind_inner(map: &mut HashMap<Card, u8>, hand: impl Iterator<Item = Card>) -> Kind {
        map.clear();
        for card in hand {
            *map.entry(card).or_insert(0) += 1;
        }

        match (map.len(), map.values().max().unwrap()) {
            (1, 5) => Kind::Five,
            (2, 4) => Kind::Four,
            (2, 3) => Kind::Full,
            (3, 3) => Kind::Three,
            (3, 2) => Kind::Two,
            (4, 2) => Kind::One,
            (5, 1) => Kind::High,
            _ => unreachable!(),
        }
    }
}

impl Fro for CamelCards {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let (hand, bid) = line.split_once(' ').unwrap();
                let hand = hand
                    .chars()
                    .map(|card| {
                        use Card::*;
                        match card {
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

                (Hand(hand), bid.to::<i64>())
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for CamelCards {
    fn one(mut self) -> i64 {
        let mut cache = HashMap::new();
        self.0
            .sort_by_cached_key(|(hand, _)| (hand.kind(&mut cache), hand.0.clone()));
        self.0
            .iter()
            .enumerate()
            .map(|(rank, (_, bid))| (rank as i64 + 1) * *bid)
            .sum()
    }

    fn two(mut self) -> i64 {
        let mut cache = HashMap::new();
        self.0.sort_by_cached_key(|(hand, _)| {
            (
                hand.kind_joker(&mut cache),
                hand.0
                    .iter()
                    .map(|card| if let Card::CJ = card { Card::J } else { *card })
                    .collect::<Vec<_>>(),
            )
        });
        self.0
            .iter()
            .enumerate()
            .map(|(rank, (_, bid))| (rank as i64 + 1) * *bid)
            .sum()
    }
}
