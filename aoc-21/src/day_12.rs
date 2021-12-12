use std::collections::HashMap;
use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct PassagePathing(HashMap<&'static str, Vec<&'static str>>);

impl Fro for PassagePathing {
    fn fro(input: &str) -> Self {
        let mut graph = HashMap::<_, Vec<_>>::new();

        input
            .trim()
            .split('\n')
            .flat_map(|line| {
                let (a, b) = line.split_once('-').unwrap();
                let (a, b) = (a.leak(), b.leak());
                [(a, b), (b, a)]
            })
            .for_each(|(a, b)| {
                graph.entry(a).or_default().push(b);
            });

        Self(graph)
    }
}

impl Solution for PassagePathing {
    fn one(self) -> i64 {
        fn recurse(
            graph: &HashMap<&'static str, Vec<&'static str>>,
            here: &'static str,
            visited: &mut HashSet<&'static str>,
        ) -> i64 {
            let mut count = 0;
            visited.insert(here);

            for next in graph.get(here).into_iter().flatten() {
                if *next == "end" {
                    count += 1;
                    continue;
                }

                if next.chars().all(|char| char.is_ascii_lowercase()) && visited.contains(next) {
                    continue;
                }

                count += recurse(graph, next, visited);
            }

            visited.remove(here);
            count
        }

        recurse(&self.0, "start", &mut HashSet::new())
    }

    fn two(self) -> i64 {
        fn recurse(
            graph: &HashMap<&'static str, Vec<&'static str>>,
            here: &'static str,
            visited: &mut HashSet<&'static str>,
            twice: bool,
        ) -> i64 {
            let mut count = 0;

            for next in graph
                .get(here)
                .into_iter()
                .flatten()
                .filter(|next| **next != "start")
            {
                if *next == "end" {
                    count += 1;
                    continue;
                }

                if next.chars().all(|char| char.is_ascii_uppercase()) {
                    count += recurse(graph, next, visited, twice);
                    continue;
                }

                match (visited.contains(next), twice) {
                    (true, true) => (),
                    (true, false) => count += recurse(graph, next, visited, true),
                    (false, _) => {
                        visited.insert(next);
                        count += recurse(graph, next, visited, twice);
                        visited.remove(next);
                    }
                }
            }

            count
        }

        recurse(&self.0, "start", &mut HashSet::new(), false)
    }
}
