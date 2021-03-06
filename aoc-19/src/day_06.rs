use petgraph::algo;
use petgraph::prelude::*;
use petgraph::visit::Walker;

use aoc::*;

pub struct UniversalOrbitMap {
    di: DiGraphMap<&'static str, ()>,
    un: UnGraphMap<&'static str, ()>,
}

impl Fro for UniversalOrbitMap {
    fn fro(input: &str) -> Self {
        let mut di = DiGraphMap::new();
        let mut un = UnGraphMap::new();
        for line in input.split_whitespace() {
            let mut iter = line.split(')');
            let inner = iter.give().leak();
            let outer = iter.give().leak();
            di.add_edge(outer, inner, ());
            un.add_edge(outer, inner, ());
        }
        UniversalOrbitMap { di, un }
    }
}

impl Solution for UniversalOrbitMap {
    fn one(self) -> i64 {
        self.di
            .nodes()
            .map(|planet| Dfs::new(&self.di, planet).iter(&self.di))
            .map(|dfs| dfs.count() as i64 - 1)
            .sum()
    }

    fn two(self) -> i64 {
        algo::dijkstra(&self.un, "YOU", Some("SAN"), |_| 1)["SAN"] - 2
    }
}
