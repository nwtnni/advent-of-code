use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;

use aoc::*;
use petgraph::graphmap::DiGraphMap;

#[derive(Clone, Debug)]
pub struct BeaconScanner(Vec<Scanner>);

#[derive(Clone, Debug)]
struct Scanner(Vec<(i64, i64, i64)>);

impl Fro for BeaconScanner {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split("\n\n")
            .map(Scanner::fro)
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Fro for Scanner {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .skip(1)
            .map(|line| {
                let mut iter = line.split(',').map(i64::fro);
                (iter.give(), iter.give(), iter.give())
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for BeaconScanner {
    fn one(self) -> i64 {
        let mut relative = HashSet::new();
        let mut oriented = HashMap::new();

        for (i, scanner) in self.0.iter().enumerate() {
            for (j, (ox, oy, oz)) in scanner.0.iter().enumerate() {
                relative.clear();

                scanner
                    .0
                    .iter()
                    .map(|(x, y, z)| (x - ox, y - oy, z - oz))
                    .tap(|iter| relative.extend(iter));

                for (k, candidate) in self.0.iter().enumerate() {
                    if i == k {
                        continue;
                    }

                    // Already found overlap between scanners
                    if oriented
                        .get(&k)
                        .into_iter()
                        .flatten()
                        .any(|(x, _, _, _): &(usize, _, _, _)| *x == i)
                    {
                        continue;
                    }

                    for origin in 0..candidate.0.len() {
                        if let Some(orientation) = overlap(&relative, origin, candidate) {
                            oriented.entry(k).or_insert_with(Vec::new).push((
                                i,
                                j,
                                orientation,
                                origin,
                            ));
                        }
                    }
                }
            }
        }

        // Collapse everything into a single coordinate system
        let mut graph = DiGraphMap::new();

        for (relative_index, entries) in &oriented {
            for (absolute_index, absolute_origin, orientation, relative_origin) in entries {
                graph.add_edge(
                    relative_index,
                    absolute_index,
                    (relative_origin, absolute_origin, orientation),
                );
            }
        }

        let mut consistent = vec![self.0[0].clone()];

        for (i, scanner) in self.0.iter().enumerate().skip(1) {
            let mut updated = scanner.clone();

            for window in petgraph::algo::all_simple_paths::<Vec<_>, _>(&graph, &i, &0, 0, None)
                .take(1)
                .next()
                .unwrap()
                .windows(2)
            {
                let (relative_origin, absolute_origin, orientation) =
                    graph.edge_weight(window[0], window[1]).unwrap();

                let orient = orient(**orientation);
                let (x2, y2, z2) = orient(self.0[*window[0]].0[**relative_origin]);
                let (x1, y1, z1) = self.0[*window[1]].0[**absolute_origin];
                let (dx, dy, dz) = (x2 - x1, y2 - y1, z2 - z1);

                for point in &mut updated.0 {
                    *point = orient(*point);
                    point.0 -= dx;
                    point.1 -= dy;
                    point.2 -= dz;
                }
            }

            consistent.push(updated);
        }

        consistent
            .iter()
            .flat_map(|Scanner(scanner)| scanner)
            .collect::<HashSet<_>>()
            .len() as i64
    }

    fn two(self) -> i64 {
        let mut relative = HashSet::new();
        let mut oriented = HashMap::new();

        for (i, scanner) in self.0.iter().enumerate() {
            for (j, (ox, oy, oz)) in scanner.0.iter().enumerate() {
                relative.clear();

                scanner
                    .0
                    .iter()
                    .map(|(x, y, z)| (x - ox, y - oy, z - oz))
                    .tap(|iter| relative.extend(iter));

                for (k, candidate) in self.0.iter().enumerate() {
                    if i == k {
                        continue;
                    }

                    // Already found overlap between scanners
                    if oriented
                        .get(&k)
                        .into_iter()
                        .flatten()
                        .any(|(x, _, _, _): &(usize, _, _, _)| *x == i)
                    {
                        continue;
                    }

                    for origin in 0..candidate.0.len() {
                        if let Some(orientation) = overlap(&relative, origin, candidate) {
                            oriented.entry(k).or_insert_with(Vec::new).push((
                                i,
                                j,
                                orientation,
                                origin,
                            ));
                        }
                    }
                }
            }
        }

        // Collapse everything into a single coordinate system
        let mut graph = DiGraphMap::new();

        for (relative_index, entries) in &oriented {
            for (absolute_index, absolute_origin, orientation, relative_origin) in entries {
                graph.add_edge(
                    relative_index,
                    absolute_index,
                    (relative_origin, absolute_origin, orientation),
                );
            }
        }

        let mut positions = vec![(0, 0, 0)];

        for (i, scanner) in self.0.iter().enumerate().skip(1) {
            let mut updated = scanner.clone();

            for window in petgraph::algo::all_simple_paths::<Vec<_>, _>(&graph, &i, &0, 0, None)
                .take(1)
                .next()
                .unwrap()
                .windows(2)
            {
                let (relative_origin, absolute_origin, orientation) =
                    graph.edge_weight(window[0], window[1]).unwrap();

                let orient = orient(**orientation);
                let (x2, y2, z2) = orient(self.0[*window[0]].0[**relative_origin]);
                let (x1, y1, z1) = self.0[*window[1]].0[**absolute_origin];
                let (dx, dy, dz) = (x2 - x1, y2 - y1, z2 - z1);

                for point in &mut updated.0 {
                    *point = orient(*point);
                    point.0 -= dx;
                    point.1 -= dy;
                    point.2 -= dz;
                }
            }

            let (x0, y0, z0) = updated.0[0];
            let (x1, y1, z1) = updated.0[1];

            let (dx, dy, dz) = (x1 - x0, y1 - y0, z1 - z0);

            for orientation in 0..48 {
                let orient = orient(orientation);

                let (x2, y2, z2) = orient(scanner.0[0]);
                let (x3, y3, z3) = orient(scanner.0[1]);

                if dx == x3 - x2 && dy == y3 - y2 && dz == z3 - z2 {
                    positions.push((x0 - x2, y0 - y2, z0 - z2));
                    break;
                }
            }
        }

        let mut max = i64::MIN;

        for i in 0..positions.len() {
            for j in 0..positions.len() {
                if i == j {
                    continue;
                }

                let (x1, y1, z1) = positions[i];
                let (x2, y2, z2) = positions[j];
                max = cmp::max(max, (x2 - x1).abs() + (y2 - y1).abs() + (z2 - z1).abs());
            }
        }

        max
    }
}

fn overlap(relative: &HashSet<(i64, i64, i64)>, origin: usize, scanner: &Scanner) -> Option<usize> {
    for orientation in 0..48 {
        let orient = orient(orientation);
        let (ox, oy, oz) = orient(scanner.0[origin]);

        let overlaps = scanner
            .0
            .iter()
            .copied()
            .map(orient)
            .map(|(x, y, z)| (x - ox, y - oy, z - oz))
            .filter(|point| relative.contains(point))
            .count();

        if overlaps >= 12 {
            return Some(orientation);
        }
    }

    None
}

fn orient(i: usize) -> impl Fn((i64, i64, i64)) -> (i64, i64, i64) {
    move |(x, y, z)| match i {
        0 => (x, y, z),
        1 => (x, y, -z),
        2 => (x, -y, z),
        3 => (x, -y, -z),
        4 => (-x, y, z),
        5 => (-x, y, -z),
        6 => (-x, -y, z),
        7 => (-x, -y, -z),
        8 => (x, z, y),
        9 => (x, z, -y),
        10 => (x, -z, y),
        11 => (x, -z, -y),
        12 => (-x, z, y),
        13 => (-x, z, -y),
        14 => (-x, -z, y),
        15 => (-x, -z, -y),
        16 => (y, x, z),
        17 => (y, x, -z),
        18 => (y, -x, z),
        19 => (y, -x, -z),
        20 => (-y, x, z),
        21 => (-y, x, -z),
        22 => (-y, -x, z),
        23 => (-y, -x, -z),
        24 => (y, z, x),
        25 => (y, z, -x),
        26 => (y, -z, x),
        27 => (y, -z, -x),
        28 => (-y, z, x),
        29 => (-y, z, -x),
        30 => (-y, -z, x),
        31 => (-y, -z, -x),
        32 => (z, y, x),
        33 => (z, y, -x),
        34 => (z, -y, x),
        35 => (z, -y, -x),
        36 => (-z, y, x),
        37 => (-z, y, -x),
        38 => (-z, -y, x),
        39 => (-z, -y, -x),
        40 => (z, x, y),
        41 => (z, x, -y),
        42 => (z, -x, y),
        43 => (z, -x, -y),
        44 => (-z, x, y),
        45 => (-z, x, -y),
        46 => (-z, -x, y),
        47 => (-z, -x, -y),
        _ => unreachable!(),
    }
}
