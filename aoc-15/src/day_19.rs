use std::collections::HashMap;
use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct MedicineForRudolph {
    replacements: HashMap<String, Vec<String>>,
    molecule: String,
}

impl Fro for MedicineForRudolph {
    fn fro(input: &str) -> Self {
        let mut iter = input.trim().split("\n\n");

        let mut replacements = HashMap::new();

        for (from, to) in iter
            .give()
            .trim()
            .split('\n')
            .map(|replacement| replacement.split_once(" => ").unwrap())
        {
            replacements
                .entry(String::from(from))
                .or_insert_with(Vec::new)
                .push(String::from(to));
        }

        let molecule = String::from(iter.give());

        Self {
            replacements,
            molecule,
        }
    }
}

impl Solution for MedicineForRudolph {
    fn one(self) -> i64 {
        let mut distinct = HashSet::new();

        for (from, tos) in &self.replacements {
            for to in tos {
                for (index, _) in self.molecule.match_indices(&*from) {
                    let mut buffer =
                        String::with_capacity(self.molecule.len() - from.len() + to.len());
                    buffer.push_str(&self.molecule[..index]);
                    buffer.push_str(to);
                    buffer.push_str(&self.molecule[index + from.len()..]);
                    distinct.insert(buffer);
                }
            }
        }

        distinct.len() as i64
    }

    fn two(self) -> i64 {
        // Had to look up solutions :(
        // https://www.reddit.com/r/adventofcode/comments/3xflz8/day_19_solutions/
        let steps = self
            .molecule
            .chars()
            .filter(|char| char.is_ascii_uppercase())
            .count()
            - self.molecule.matches("Rn").count()
            - self.molecule.matches("Ar").count()
            - 2 * self.molecule.matches('Y').count()
            - 1;
        steps as i64
    }
}
