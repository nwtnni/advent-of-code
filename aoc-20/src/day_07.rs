use std::iter;
use std::mem;

use aoc::*;
use petgraph::prelude::*;

pub struct HandyHaversacks(DiGraphMap<&'static str, i64>);

impl Fro for HandyHaversacks {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .flat_map(|line| {
                let mut iter = line.trim().split(" contain ");

                let start = iter
                    .give()
                    .trim_end_matches(" bags")
                    .to_string()
                    .leak();

                let rest = iter.give();
                if rest == "no other bags." {
                    return Or::L(iter::empty());
                }

                rest.trim()
                    .split(",")
                    .map(move |pair| {
                        let mut iter = pair
                            .trim()
                            .trim_end_matches(".")
                            .trim_end_matches("s")
                            .trim_end_matches(" bag")
                            .splitn(2, " ");
                        let count = iter.give().to::<i64>();
                        let end = iter.give().to_string().leak();
                        (start, end, count)
                    })
                    .tap(Or::R)
            })
            .collect::<DiGraphMap<_, _>>()
            .tap(Self)
    }
}

impl Solution for HandyHaversacks {
    fn one(self) -> i64 {
        let graph = self
            .0
            .all_edges()
            .map(|(start, end, count)| (end, start, count))
            .collect::<DiGraphMap<_, i64>>();

        petgraph::algo::dijkstra(&graph, "shiny gold", None, |_| 1usize)
            .len()
            as i64
            - 1
    }

    fn two(self) -> i64 {
        let mut layer = vec![(1, "shiny gold")];
        let mut sum = 0;

        while layer.len() > 0 {
            for (multiplier, start) in mem::take(&mut layer) {
                sum += multiplier;
                for (_, end, edge) in self.0.edges(&start) {
                    layer.push((edge * multiplier, end));
                }
            }
        }

        sum as i64 - 1
    }
}
