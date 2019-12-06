use petgraph::algo;
use petgraph::prelude::*;

use aoc::*;

pub struct Placeholder {
    di: DiGraphMap<&'static str, ()>,
    un: UnGraphMap<&'static str, ()>,
}

impl Fro for Placeholder {
    fn fro(input: &str) -> Self {
        let mut di = DiGraphMap::new();
        let mut un = UnGraphMap::new();
        for line in input.split_whitespace() {
            let mut iter = line.split(')');
            let inner: &'static str = Box::leak(iter.give().to_owned().into_boxed_str());
            let outer: &'static str = Box::leak(iter.give().to_owned().into_boxed_str());
            di.add_edge(outer, inner, ());
            un.add_edge(outer, inner, ());
        }
        Placeholder { di, un }
    }
}

impl Solution for Placeholder {
    fn one(self) -> i32 {
        let mut direct = 0;
        let mut indirect = 0;
        for planet in self.di.nodes() {
            for neighbor in self.di.neighbors(planet) {
                direct += 1;
                let mut dfa = Dfs::new(&self.di, neighbor);
                dfa.next(&self.di);
                while let Some(_) = dfa.next(&self.di) {
                    indirect += 1;
                }
            }
        }
        direct + indirect
    }

    fn two(self) -> i32 {
        algo::dijkstra(&self.un, "YOU", Some("SAN"), |_| 1)["SAN"] - 2
    }
}
