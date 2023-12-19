use std::collections::HashMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct Aplenty {
    rules: HashMap<State, Vec<Rule>>,
    items: Vec<Item>,
}

type Item = [i64; 4];
type State = String;

#[derive(Clone, Debug)]
enum Rule {
    Lt {
        field: usize,
        value: i64,
        next: State,
    },
    Gt {
        field: usize,
        value: i64,
        next: State,
    },
    Go(String),
}

impl Fro for Aplenty {
    fn fro(input: &str) -> Self {
        let mut iter = input.trim().split("\n\n");

        let rules = iter
            .give()
            .lines()
            .map(|line| {
                let (a, b) = line.split_once('{').unwrap();
                let bs = b
                    .trim_end_matches('}')
                    .split(',')
                    .map(|rule| match rule.split_once(':') {
                        None => Rule::Go(rule.to_owned()),
                        Some((l, r)) => match l.split_once('<') {
                            Some((f, v)) => Rule::Lt {
                                field: match f {
                                    "x" => 0,
                                    "m" => 1,
                                    "a" => 2,
                                    "s" => 3,
                                    _ => unreachable!(),
                                },
                                value: v.to::<i64>(),
                                next: r.to_owned(),
                            },
                            None => {
                                let (f, v) = l.split_once('>').unwrap();
                                Rule::Gt {
                                    field: match f {
                                        "x" => 0,
                                        "m" => 1,
                                        "a" => 2,
                                        "s" => 3,
                                        _ => unreachable!(),
                                    },
                                    value: v.to::<i64>(),
                                    next: r.to_owned(),
                                }
                            }
                        },
                    })
                    .collect::<Vec<_>>();
                (a.to_owned(), bs)
            })
            .collect::<HashMap<_, _>>();

        let items = iter
            .give()
            .lines()
            .map(|line| {
                line.trim_start_matches('{')
                    .trim_end_matches('}')
                    .split(',')
                    .map(|pair| pair.split_once('=').unwrap().1.to::<i64>())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>();

        Self { rules, items }
    }
}

impl Solution for Aplenty {
    fn one(self) -> i64 {
        self.items
            .iter()
            .filter(|item| {
                let mut here = "in";

                while let Some(rules) = self.rules.get(here) {
                    here = rules
                        .iter()
                        .find_map(|rule| match rule {
                            Rule::Lt { field, value, next } if item[*field] < *value => Some(next),
                            Rule::Gt { field, value, next } if item[*field] > *value => Some(next),
                            Rule::Go(next) => Some(next),
                            _ => None,
                        })
                        .unwrap()
                }

                here == "A"
            })
            .flatten()
            .sum()
    }

    fn two(self) -> i64 {
        #[derive(Copy, Clone, Debug)]
        struct Bound {
            min: i64,
            max: i64,
        }

        let mut paths = vec![];
        let mut work = vec![("in", [Bound { min: 1, max: 4000 }; 4])];

        while let Some((here, mut bounds)) = work.pop() {
            if here == "A" {
                paths.push(bounds);
                continue;
            } else if here == "R" {
                continue;
            }

            for rule in &self.rules[here] {
                match rule {
                    Rule::Lt { field, value, next } => {
                        let mut split = bounds;
                        split[*field].max = bounds[*field].max.min(value - 1);
                        work.push((next, split));

                        bounds[*field].min = *value;
                    }
                    Rule::Gt { field, value, next } => {
                        let mut split = bounds;
                        split[*field].min = bounds[*field].min.max(value + 1);
                        work.push((next, split));

                        bounds[*field].max = *value;
                    }
                    Rule::Go(next) => {
                        work.push((next, bounds));
                    }
                }
            }
        }

        paths
            .iter()
            .map(|bounds| {
                bounds
                    .iter()
                    .filter(|Bound { min, max }| max >= min)
                    .map(|Bound { min, max }| max - min + 1)
                    .product::<i64>()
            })
            .sum()
    }
}
