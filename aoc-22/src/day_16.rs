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
                let node: &'static str = node.trim_start_matches("Valve ").leak();
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
    fn one(self) -> i64 {
        // let (nodes, distances) = self.floyd_warshall();
        // let mut max = 0;
        // search("AA", &nodes, &self.rates, &distances, &mut max, 0, 0, 0, 0);
        // max
        todo!()
    }

    fn two(self) -> i64 {
        todo!()
    }
}

// fn search(
//     node: usize,
//     nodes: &[&'static str],
//     rates: &HashMap<&'static str, i64>,
//     distances: &HashMap<(&'static str, &'static str), i64>,
//     max: &mut i64,
//     opened: u32,
//     visited: u32,
//     time: i64,
//     score: i64,
// ) {
//     if time == 30 {
//         *max = cmp::max(*max, score);
//         return;
//     }
//
//     for i in 0..nodes.len() {
//         if opened & (1 << i) > 0 {
//             continue;
//         }
//
//         search(
//             nodes[i],
//             nodes,
//             distances,
//             max,
//             opened,
//             time + distances[&(node, nodes[i])] + 1,
//             score + (30 - time - 1) * rates[node],
//         );
//     }
// }

impl ProboscideaVolcanium {
    #[allow(unused)]
    fn floyd_warshall(
        &mut self,
    ) -> (
        Vec<&'static str>,
        HashMap<(&'static str, &'static str), i64>,
    ) {
        let mut nodes = self.graph.nodes().collect::<Vec<_>>();

        let mut distances = self
            .graph
            .all_edges()
            .map(|(from, to, cost)| ((from, to), *cost))
            .chain(nodes.iter().map(|node| ((*node, *node), 0)))
            .collect::<HashMap<_, _>>();

        for k in &nodes {
            for i in &nodes {
                for j in &nodes {
                    let ik = distances.get(&(i, k)).copied().unwrap_or(i64::MAX);
                    let kj = distances.get(&(k, j)).copied().unwrap_or(i64::MAX);

                    if distances.get(&(i, j)).copied().unwrap_or(i64::MAX) > ik.saturating_add(kj) {
                        distances.insert((i, j), ik + kj);
                    }
                }
            }
        }

        nodes.retain(|node| self.rates[node] > 0);
        distances.retain(|(from, to), _| self.rates[from] > 0 && self.rates[to] > 0);
        (nodes, distances)
    }

    #[allow(unused)]
    fn debug(&self) {
        println!("{}", petgraph::dot::Dot::new(&self.graph));
    }
}
