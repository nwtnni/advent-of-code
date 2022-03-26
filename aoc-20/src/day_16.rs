use std::collections::HashMap;
use std::collections::HashSet;
use std::iter;
use std::mem;

use aoc::*;

#[derive(Clone, Debug)]
pub struct TicketTranslation {
    fields: Vec<Field>,
    yours: Vec<i64>,
    nearby: Vec<Vec<i64>>,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Field {
    name: &'static str,
    lo: (i64, i64),
    hi: (i64, i64),
}

impl Field {
    fn validate(&self, n: i64) -> bool {
        (n >= self.lo.0 && n <= self.lo.1) || (n >= self.hi.0 && n <= self.hi.1)
    }
}

impl Fro for TicketTranslation {
    fn fro(input: &str) -> Self {
        let mut iter = input.trim().split("\n\n");

        let fields = iter
            .give()
            .trim()
            .split('\n')
            .map(|line| {
                let mut iter = line.split(": ");
                let name = iter.give().leak();

                let mut iter = iter.give().trim().split(" or ");

                let mut lo = iter.give().trim().split('-').map(i64::fro);
                let mut hi = iter.give().trim().split('-').map(i64::fro);

                Field {
                    name,
                    lo: (lo.give(), lo.give()),
                    hi: (hi.give(), hi.give()),
                }
            })
            .collect::<Vec<_>>();

        let yours = iter
            .give()
            .trim()
            .split('\n')
            .nth(1)
            .unwrap()
            .split(',')
            .map(i64::fro)
            .collect::<Vec<_>>();

        let nearby = iter
            .give()
            .trim()
            .split('\n')
            .skip(1)
            .map(|line| line.split(',').map(i64::fro).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        Self {
            fields,
            yours,
            nearby,
        }
    }
}

impl Solution for TicketTranslation {
    fn one(self) -> i64 {
        self.nearby
            .iter()
            .flatten()
            .filter(|value| self.fields.iter().all(|field| !field.validate(**value)))
            .sum()
    }

    fn two(mut self) -> i64 {
        let mut destroy = mem::take(&mut self.nearby);
        destroy.retain(|ticket| {
            ticket
                .iter()
                .all(|value| self.fields.iter().any(|field| field.validate(*value)))
        });
        self.nearby = destroy;

        let mut solved = HashSet::<Field>::new();
        let mut assign: HashMap<usize, HashSet<Field>> = (0..self.fields.len())
            .map(|i| (i, self.fields.iter().copied().collect()))
            .collect();

        'outer: loop {
            for ticket in self.nearby.iter().chain(iter::once(&self.yours)) {
                for (i, value) in ticket.iter().enumerate() {
                    if assign[&i].len() == 1 {
                        continue;
                    }

                    assign
                        .get_mut(&i)
                        .unwrap()
                        .retain(|field| !solved.contains(&field) && field.validate(*value));

                    if assign[&i].len() == 1 {
                        solved.insert(*assign[&i].iter().give());
                    }

                    if solved.len() == self.fields.len() {
                        break 'outer;
                    }
                }
            }
        }

        assign
            .iter()
            .filter(|(_, set)| set.iter().give().name.starts_with("departure"))
            .map(|(i, _)| self.yours[*i])
            .product()
    }
}
