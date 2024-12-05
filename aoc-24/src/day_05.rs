use aoc::*;
use petgraph::prelude::DiGraphMap;

#[derive(Clone, Debug)]
pub struct PrintQueue {
    rules: DiGraphMap<i64, ()>,
    updates: Vec<Vec<i64>>,
}

impl Fro for PrintQueue {
    fn fro(input: &str) -> Self {
        let (rules, updates) = input.trim().split_once("\n\n").unwrap();
        let rules = rules
            .split('\n')
            .map(|line| {
                let (l, r) = line.split_once('|').unwrap();
                (l.to::<i64>(), r.to::<i64>(), ())
            })
            .collect();

        let updates = updates
            .split('\n')
            .map(|line| line.split(',').map(i64::fro).collect())
            .collect();

        Self { rules, updates }
    }
}

impl Solution for PrintQueue {
    fn one(self) -> i64 {
        self.updates
            .iter()
            .filter(|update| self.correct(update))
            .map(|update| update[update.len() / 2])
            .sum()
    }

    fn two(self) -> i64 {
        self.updates
            .iter()
            .filter(|update| !self.correct(update))
            .map(|update| self.sorted(update))
            .map(|update| update[update.len() / 2])
            .sum()
    }
}

impl PrintQueue {
    fn correct(&self, update: &[i64]) -> bool {
        update
            .windows(2)
            .all(|window| self.rules.contains_edge(window[0], window[1]))
    }

    fn sorted(&self, update: &[i64]) -> Vec<i64> {
        let subgraph = update
            .iter()
            .flat_map(|page| self.rules.edges(*page))
            .filter(|(_, next, ())| update.contains(next))
            .collect::<DiGraphMap<i64, ()>>();

        petgraph::algo::toposort(&subgraph, None).unwrap()
    }
}
