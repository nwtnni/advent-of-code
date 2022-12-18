use std::cmp;
use std::collections::HashMap;

use aoc::*;
use petgraph::graphmap::UnGraphMap;

#[derive(Clone, Debug)]
pub struct ProboscideaVolcanium {
    graph: UnGraphMap<&'static str, i64>,
    rates: HashMap<&'static str, i64>,
}

impl Fro for ProboscideaVolcanium {
    fn fro(input: &str) -> Self {
        let mut rates = HashMap::new();
        let graph = input
            .trim()
            .split('\n')
            .flat_map(|line| {
                let (node, rate) = line.split_once(" has flow rate=").unwrap();
                let (rate, edges) = rate
                    .split_once("; tunnels lead to valves ")
                    .or_else(|| rate.split_once("; tunnel leads to valve "))
                    .unwrap();
                let node = node.trim_start_matches("Valve ").leak();
                let rate = i64::fro(rate);

                rates.insert(node, rate);

                edges
                    .split(", ")
                    .map(str::leak)
                    .map(move |tunnel| (node, tunnel, 1))
            })
            .collect();

        Self { graph, rates }
    }
}

impl Solution for ProboscideaVolcanium {
    fn one(mut self) -> i64 {
        self.debug();
        self.coalesce();
        self.debug();
        todo!()
    }

    fn two(self) -> i64 {
        todo!()
    }
}

impl ProboscideaVolcanium {
    fn coalesce(&mut self) {
        let mut buffer = Vec::new();
        self.rates
            .iter()
            .filter(|(node, _)| **node != "AA")
            .filter(|(_, rate)| **rate == 0)
            .for_each(|(node, _)| {
                buffer.clear();
                buffer.extend(self.graph.neighbors(node));

                for i in 0..buffer.len() {
                    let time_i = self.graph.edge_weight(node, buffer[i]).copied().unwrap();

                    for j in i + 1..buffer.len() {
                        let time_j = self.graph.edge_weight(node, buffer[j]).copied().unwrap();

                        match self.graph.edge_weight_mut(buffer[i], buffer[j]) {
                            Some(weight) => *weight = cmp::min(*weight, time_i + time_j),
                            None => {
                                self.graph.add_edge(buffer[i], buffer[j], time_i + time_j);
                            }
                        }
                    }
                }

                self.graph.remove_node(node);
            });

        self.rates.retain(|node, rate| *node == "AA" || *rate > 0);
    }

    #[allow(unused)]
    fn debug(&self) {
        println!("{}", petgraph::dot::Dot::new(&self.graph));
    }
}
