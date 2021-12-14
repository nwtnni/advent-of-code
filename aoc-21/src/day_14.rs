use std::collections::HashMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct ExtendedPolymerization{
    start: &'static str,
    rules: Vec<Rule> }

#[derive(Copy, Clone, Debug)]
struct Rule {
    from: &'static str,
    to: char,
}

impl Fro for ExtendedPolymerization {
    fn fro(input: &str) -> Self {
        let (a, b) = input.trim().split_once("\n\n").unwrap();

        let start = a.leak();
        let rules = b
            .trim()
            .split('\n')
            .map(Rule::fro)
            .collect::<Vec<_>>();

        Self { start, rules }
    }
}

impl Fro for Rule {
    fn fro(input: &str) -> Self {
        let (a, b) = input.trim().split_once(" -> ").unwrap();
        let (a, b) = (a.leak(), b.chars().give());
        Rule { from: a, to: b }
    }
}

impl Solution for ExtendedPolymerization {
    fn one(self) -> i64 {
        let mut prev = self.start.to_string();
        let rules = self
            .rules
            .iter()
            .map(|rule| (rule.from, rule.to))
            .collect::<HashMap<_, _>>();

        for _ in 0..10 {
            let mut next = String::new();
            next.push_str(&prev[0..1]);
            for i in 0..prev.len() - 1 {
                if let Some(c) = rules.get(&prev[i..i + 2]) {
                    next.push(*c);
                }
                next.push_str(&prev[i + 1..i + 2]);
            }
            prev = next;
        }

        let mut count = HashMap::new();
        for c in prev.chars() {
            *count.entry(c).or_insert(0) += 1;
        }

        let (_, min) = count.iter().min_by_key(|(_, c)| *c).unwrap();
        let (_, max) = count.iter().max_by_key(|(_, c)| *c).unwrap();

        (max - min) as i64
    }

    fn two(self) -> i64 {
        let mut graph = HashMap::new();

        for rule in self.rules {
            let l = [rule.from.as_bytes()[0] - b'A', rule.to as u8 - b'A'];
            let r = [rule.to as u8 - b'A', rule.from.as_bytes()[1] - b'A'];
            let v = [rule.from.as_bytes()[0] - b'A', rule.from.as_bytes()[1] - b'A'];

            graph.insert(v, (l, r));
        }

        let start = self.start.as_bytes();

        fn recurse(
            graph: &HashMap<[u8; 2], ([u8; 2], [u8; 2])>,
            window: [u8; 2],
            height: usize,
            memo: &mut HashMap<([u8; 2], usize), [i64; 26]>,
        ) -> [i64; 26] {
            if let Some(array) = memo.get(&(window, height)) {
                return *array;
            }

            if height == 0 {
                let mut array = [0; 26];
                array[window[0] as usize] += 1;
                array[window[1] as usize] += 1;
                return array;
            }

            match graph.get(&window) {
                None => {
                    let mut array = [0; 26];
                    array[window[0] as usize] += 1;
                    array[window[1] as usize] += 1;
                    array
                }
                Some((l, r)) => {
                    let left = recurse(graph, *l, height - 1, memo);
                    let right = recurse(graph, *r, height - 1, memo);

                    let mut array = [0; 26];
                    for i in 0..26 {
                        array[i] = left[i] + right[i];
                    }

                    let overlap = l[1];
                    array[overlap as usize] -= 1;
                    memo.insert((window, height), array);
                    array
                }
            }
        }

        let mut total = [0; 26];
        let mut memo = HashMap::new();
        for window in start.windows(2) {
            let w = [window[0] - b'A', window[1] - b'A'];
            recurse(&graph, w, 40, &mut memo)
                .into_iter()
                .zip(&mut total)
                .for_each(|(a, b)| *b += a);
        }

        for i in 1..start.len() - 1 {
            total[(start[i] - b'A') as usize] -= 1;
        }

        let min = total.into_iter().filter(|x| *x > 0).min().unwrap();
        let max = total.into_iter().max().unwrap();

        (max - min) as i64
    }
}
