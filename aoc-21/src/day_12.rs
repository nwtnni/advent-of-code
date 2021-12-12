use std::collections::HashSet;

use aoc::*;
use petgraph::graphmap::UnGraphMap;

#[derive(Clone, Debug)]
pub struct PassagePathing(UnGraphMap<&'static str, ()>);

impl Fro for PassagePathing {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let (a, b) = line.split_once('-').unwrap();
                (a.leak(), b.leak(), ())
            })
            .collect::<UnGraphMap<_, _>>()
            .tap(Self)
    }
}

impl Solution for PassagePathing {
    fn one(mut self) -> i64 {
        let mut total = 0;
        recurse(&mut self.0, "start", &mut total);
        total as i64
    }

    fn two(mut self) -> i64 {
        let mut all = HashSet::new();
        recurse2(&mut self.0, "start", false, &mut vec![], &mut all);
        all.len() as i64
    }
}

fn recurse(graph: &mut UnGraphMap<&'static str, ()>, here: &'static str, total: &mut usize) {
    if here == "end" {
        *total += 1;
        return;
    }

    if !petgraph::algo::dijkstra(&*graph, here, Some("end"), |_| 1).contains_key("end") {
        return;
    }

    let neighbors = graph.neighbors(here).collect::<Vec<_>>();

    if here.chars().all(|char| char.is_ascii_lowercase()) {
        graph.remove_node(here);
    }

    for next in &neighbors {
        recurse(graph, next, total);
    }

    if here.chars().all(|char| char.is_ascii_lowercase()) {
        graph.extend(neighbors.into_iter().map(|neighbor| (here, neighbor, ())));
    }
}

fn recurse2(graph: &mut UnGraphMap<&'static str, ()>, here: &'static str, twice: bool, path: &mut Vec<&'static str>, all: &mut HashSet<Vec<&'static str>>) {
    path.push(here);

    if here == "end" {
        all.insert(path.clone());
        path.pop();
        return;
    }

    if !petgraph::algo::dijkstra(&*graph, here, Some("end"), |_| 1).contains_key("end") {
        path.pop();
        return;
    }

    if here.chars().all(|char| char.is_ascii_lowercase()) {
        if !twice && here != "start" && here != "end" {
            for next in graph.neighbors(here).collect::<Vec<_>>() {
                recurse2(graph, next, true, path, all);
            }
        }

        let neighbors = graph.neighbors(here).collect::<Vec<_>>();
        graph.remove_node(here);

        for next in &neighbors {
            recurse2(graph, next, twice, path, all);
        }

        graph.extend(neighbors.into_iter().map(|neighbor| (here, neighbor, ())));
    }

    if here.chars().all(|char| char.is_ascii_uppercase()) {
        for next in graph.neighbors(here).collect::<Vec<_>>() {
            recurse2(graph, next, twice, path, all);
        }
    }

    path.pop();
}
