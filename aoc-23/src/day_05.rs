use std::cmp::Ordering;
use std::collections::HashMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct IfYouGiveASeedAFertilizer {
    seeds: Vec<i64>,
    maps: HashMap<String, Map>,
}

#[derive(Clone, Debug)]
struct Map {
    source: String,
    target: String,
    ranges: Vec<[i64; 3]>,
}

impl Fro for IfYouGiveASeedAFertilizer {
    fn fro(input: &str) -> Self {
        let mut iter = input.trim().split("\n\n");

        let (_, seeds) = iter.give().split_once(": ").unwrap();
        let seeds = seeds.split_whitespace().map(i64::fro).collect::<Vec<_>>();

        let mut maps = HashMap::new();
        for map in iter {
            let mut iter = map.split('\n');
            let (name, _) = iter.give().split_once(' ').unwrap();
            let (a, b) = name.split_once('-').unwrap();
            let (_, b) = b.split_once('-').unwrap();

            let ranges = iter
                .map(|a| {
                    a.split_whitespace()
                        .map(i64::fro)
                        .collect::<Vec<i64>>()
                        .try_into()
                        .unwrap()
                })
                .collect();

            maps.insert(
                a.to_owned(),
                Map {
                    source: a.to_owned(),
                    target: b.to_owned(),
                    ranges,
                },
            );
        }

        Self { seeds, maps }
    }
}

impl Solution for IfYouGiveASeedAFertilizer {
    fn one(self) -> i64 {
        self.seeds
            .iter()
            .map(|seed| {
                let mut map = &self.maps["seed"];
                let mut i = *seed;
                loop {
                    if map.source == "location" {
                        return i;
                    } else {
                        let mut j = None;
                        for range in &map.ranges {
                            if i >= range[1] && i < range[1] + range[2] {
                                j = Some(range[0] + i - range[1]);
                                break;
                            }
                        }

                        match j {
                            None => (),
                            Some(j) => i = j,
                        }

                        if map.target == "location" {
                            break i;
                        } else {
                            map = &self.maps[&map.target];
                        }
                    }
                }
            })
            .min()
            .unwrap()
    }

    fn two(mut self) -> i64 {
        for map in &mut self.maps {
            map.1.ranges.sort_by_key(|[_, s, _]| *s);
        }

        self.seeds
            .chunks(2)
            .map(|range| {
                let mut ranges = vec![[range[0], range[0] + range[1] - 1]];
                let mut map = &self.maps["seed"];

                loop {
                    let initial = ranges.iter().map(|[s, e]| e + 1 - s).sum::<i64>();
                    let mut out = Vec::new();
                    let mut ls = ranges.iter_mut().peekable();
                    let mut rs = map.ranges.iter().peekable();

                    loop {
                        match (ls.peek_mut(), rs.peek()) {
                            (None, None) => break,
                            (None, Some(_)) => break,
                            (Some(l), None) => {
                                out.push(**l);
                                ls.next();
                            }
                            (Some(l), Some(r)) => {
                                let sl = l[0];
                                let el = l[1];

                                let sr = r[1];
                                let er = r[1] + r[2] - 1;

                                eprintln!(
                                    "{}-{} to {}-{}, maps to {}-{}",
                                    sl,
                                    el,
                                    sr,
                                    er,
                                    r[0],
                                    r[0] + r[2] - 1
                                );

                                match (sl.cmp(&sr), el.cmp(&er)) {
                                    (Ordering::Less, Ordering::Less) => {
                                        if el < sr {
                                            out.push([sl, el]);
                                            ls.next();
                                        } else {
                                            out.push([sl, sr - 1]);
                                            l[0] = sr;
                                        }
                                    }
                                    (Ordering::Greater, Ordering::Greater) => {
                                        if sl > er {
                                            rs.next();
                                        } else {
                                            out.push([sl - sr + r[0], er - sr + r[0]]);
                                            l[0] = er + 1;
                                            rs.next();
                                        }
                                    }
                                    (Ordering::Less, Ordering::Equal) => {
                                        out.push([sl, sr - 1]);
                                        out.push([r[0], er - sr + r[0]]);
                                        ls.next();
                                        rs.next();
                                    }
                                    (Ordering::Less, Ordering::Greater) => {
                                        out.push([sl, sr - 1]);
                                        out.push([r[0], er - sr + r[0]]);
                                        l[0] = er + 1;
                                        rs.next();
                                    }
                                    (Ordering::Equal, Ordering::Less) => {
                                        out.push([r[0], el - sr + r[0]]);
                                        ls.next();
                                    }
                                    (Ordering::Equal, Ordering::Equal) => {
                                        out.push([r[0], er - sr + r[0]]);
                                        ls.next();
                                        rs.next();
                                    }
                                    (Ordering::Equal, Ordering::Greater) => {
                                        out.push([r[0], er - sr + r[0]]);
                                        l[0] = er + 1;
                                    }
                                    (Ordering::Greater, Ordering::Less) => {
                                        out.push([sl - sr + r[0], el - sr + r[0]]);
                                        ls.next();
                                    }
                                    (Ordering::Greater, Ordering::Equal) => {
                                        out.push([sl - sr + r[0], er - sr + r[0]]);
                                        ls.next();
                                        rs.next();
                                    }
                                }
                            }
                        }
                    }

                    assert_eq!(initial, out.iter().map(|[s, e]| e + 1 - s).sum::<i64>(),);
                    ranges = out;
                    ranges.sort_by_key(|[s, _]| *s);
                    if map.target == "location" {
                        return ranges[0][0];
                    } else {
                        map = &self.maps[&map.target];
                    }
                }
            })
            .min()
            .unwrap()
    }
}
