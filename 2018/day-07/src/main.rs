#[macro_use]
extern crate nom;

use petgraph::prelude::*;
use tap::*;
use nom::types::CompleteStr;
use nom::alpha;

const INPUT: &'static str = include_str!("input.txt");

pub struct Step {
    before: &'static str,
    after: &'static str,
}

named!(parse_step<CompleteStr<'static>, Step>,
    do_parse!(
                tag_s!("Step ") >>
        before: alpha >>
                tag_s!(" must be finished before step ") >>
        after:  alpha >>
                tag_s!(" can begin.") >>
        (Step {
            before: before.0,
            after: after.0
        })
    )
);

named!(parse_steps<CompleteStr<'static>, Vec<Step>>,
    separated_list!(tag_s!("\n"), parse_step)
);

type Node = &'static str;

fn ready<T>(node: Node, graph: &DiGraphMap<Node, T>) -> bool {
    graph.neighbors_directed(node, Direction::Incoming).count() == 0
}

fn sort(graph: &mut DiGraphMap<Node, ()>) -> Vec<Node> {
    let mut sorted = Vec::new();
    let mut queue = graph.nodes()
        .filter(|n| ready(n, graph))
        .collect::<Vec<_>>()
        .tap(|v| v.sort());

    while !queue.is_empty() {
        let node = queue.remove(0);
        sorted.push(node);
        graph.remove_node(node);

        let update = graph.nodes()
            .filter(|n| ready(n, graph))
            .filter(|n| !queue.contains(n))
            .collect::<Vec<_>>();

        queue.extend(update);
        queue.sort();
    }

    sorted
}

fn time(node: &str) -> usize {
    match node {
    | "A" => 1,
    | "B" => 2,
    | "C" => 3,
    | "D" => 4,
    | "E" => 5,
    | "F" => 6,
    | "G" => 7,
    | "H" => 8,
    | "I" => 9,
    | "J" => 10,
    | "K" => 11,
    | "L" => 12,
    | "M" => 13,
    | "N" => 14,
    | "O" => 15,
    | "P" => 16,
    | "Q" => 17,
    | "R" => 18,
    | "S" => 19,
    | "T" => 20,
    | "U" => 21,
    | "V" => 22,
    | "W" => 23,
    | "X" => 24,
    | "Y" => 25,
    | "Z" => 26,
    | _ => unreachable!(),
    }
}

fn main() {
    let mut graph: DiGraphMap<Node, ()> = GraphMap::new();
    let (_, steps) = parse_steps(CompleteStr(INPUT))
        .unwrap();

    for step in steps {
        graph.add_edge(step.before, step.after, ());
    }

    let sorted = sort(&mut graph);
    for c in &sorted { print!("{}", c); }
    print!("\n");
}
