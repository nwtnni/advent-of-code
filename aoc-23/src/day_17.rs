use std::cmp::Reverse;
use std::collections::HashSet;

use aoc::*;
use priority_queue::PriorityQueue;

#[derive(Clone, Debug)]
pub struct ClumsyCrucible {
    rows: i64,
    cols: i64,
    grid: Vec<i64>,
}

impl Fro for ClumsyCrucible {
    fn fro(input: &str) -> Self {
        let mut grid = Vec::new();
        let mut rows = 0;
        let mut cols = 0;
        for (_, row) in input.trim().split('\n').enumerate() {
            rows += 1;
            cols = 0;
            for (_, col) in row.trim().chars().enumerate() {
                cols += 1;
                grid.push(col.to_digit(10).unwrap() as i64);
            }
        }
        Self { cols, rows, grid }
    }
}

impl Solution for ClumsyCrucible {
    fn one(self) -> i64 {
        self.dijkstra(Self::normal, 0)
    }

    fn two(self) -> i64 {
        self.dijkstra(Self::ultra, 3)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Node {
    p: Pos,
    d: Dir,
    m: i8,
}

impl ClumsyCrucible {
    fn dijkstra(&self, filter: fn(Node, Node) -> bool, minimum: i8) -> i64 {
        let mut visited = HashSet::new();
        let mut queue = PriorityQueue::new();
        queue.push(
            Node {
                p: Pos::from_i_j(0, 0),
                d: Dir::E,
                m: -1,
            },
            Reverse(0),
        );
        queue.push(
            Node {
                p: Pos::from_i_j(0, 0),
                d: Dir::S,
                m: -1,
            },
            Reverse(0),
        );

        let end = Pos::from_i_j(self.rows - 1, self.cols - 1);

        while let Some((here, Reverse(loss))) = queue.pop() {
            if here.p == end && here.m >= minimum {
                return loss;
            }

            visited.insert(here);

            self.reachable(here, filter)
                .filter(|next| !visited.contains(next))
                .for_each(|next| {
                    queue.push_increase(next, Reverse(loss + self.loss(next.p)));
                });
        }

        unreachable!()
    }

    fn reachable(&self, node: Node, filter: fn(Node, Node) -> bool) -> impl Iterator<Item = Node> {
        let cw = node.d.rotate_clockwise();
        let ccw = node.d.rotate_counterclockwise();
        let rows = self.rows;
        let cols = self.cols;

        [
            Node {
                p: node.p.shift(cw),
                d: cw,
                m: 0,
            },
            Node {
                p: node.p.shift(node.d),
                d: node.d,
                m: node.m + 1,
            },
            Node {
                p: node.p.shift(ccw),
                d: ccw,
                m: 0,
            },
        ]
        .into_iter()
        .filter(move |next| {
            next.p.i() >= 0 && next.p.i() < rows && next.p.j() >= 0 && next.p.j() < cols
        })
        .filter(move |next| filter(node, *next))
    }

    fn normal(_: Node, next: Node) -> bool {
        next.m < 3
    }

    fn ultra(node: Node, next: Node) -> bool {
        (node.m >= 3 || next.d == node.d) && (node.m < 9 || next.d != node.d)
    }

    fn loss(&self, pos: Pos) -> i64 {
        self.grid[(pos.i() * self.cols + pos.j()) as usize]
    }
}
