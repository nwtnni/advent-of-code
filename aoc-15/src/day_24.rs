use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct ItHangsInTheBalance(Vec<i64>);

impl Fro for ItHangsInTheBalance {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(i64::fro)
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for ItHangsInTheBalance {
    fn one(self) -> i64 {
        let total = self.0.iter().sum::<i64>();
        let third = total / 3;

        let mut groups = Vec::new();

        recurse(&self.0, 0, third, 0, &mut groups);

        let set = groups.iter().copied().collect::<HashSet<_>>();
        let all = 2u64.pow(self.0.len() as u32) - 1;

        let mut firsts = Vec::new();

        'outer: for i in 0..groups.len() {
            for j in 0..groups.len() {
                let a = groups[i];
                let b = groups[j];

                if a & b > 0 {
                    continue;
                }

                if set.contains(&(all & !a & !b)) {
                    firsts.push(a);
                    continue 'outer;
                }
            }
        }

        self.minimum(&firsts)
    }

    fn two(self) -> i64 {
        let total = self.0.iter().sum::<i64>();
        let fourth = total >> 2;

        let mut groups = Vec::new();

        recurse(&self.0, 0, fourth, 0, &mut groups);

        let set = groups.iter().copied().collect::<HashSet<_>>();
        let all = 2u64.pow(self.0.len() as u32) - 1;

        let mut firsts = Vec::new();

        'outer: for i in 0..groups.len() {
            for j in 0..groups.len() {
                for k in 0..groups.len() {
                    let a = groups[i];
                    let b = groups[j];
                    let c = groups[k];

                    if a & b > 0 || b & c > 0 || a & c > 0 {
                        continue;
                    }

                    if set.contains(&(all & !a & !b & !c)) {
                        firsts.push(a);
                        continue 'outer;
                    }
                }
            }
        }

        self.minimum(&firsts)
    }
}

impl ItHangsInTheBalance {
    fn minimum(&self, firsts: &[u64]) -> i64 {
        firsts
            .iter()
            .map(|first| {
                let count = first.count_ones();
                let mut entanglement = 1;

                for index in 0..64 {
                    if first & (1 << index) > 0 {
                        entanglement *= self.0[index];
                    }
                }

                (count, entanglement)
            })
            .min()
            .unwrap()
            .1
    }
}

/// Push all groups of packages within `packages[index..]` that add up to `quota` onto `groups`.
///
/// Groups are bit sets, where each bit corresponds to the index of the package within `packages`.
fn recurse(packages: &[i64], index: usize, quota: i64, group: u64, groups: &mut Vec<u64>) {
    match packages.get(index) {
        None => (),
        // Early return requires that packages are in sorted order
        Some(package) if *package > quota => (),
        Some(package) => {
            // Recurse without this package
            recurse(packages, index + 1, quota, group, groups);

            // Recurse with this package
            if *package == quota {
                groups.push(group | 1 << index);
            } else {
                recurse(
                    packages,
                    index + 1,
                    quota - package,
                    group | 1 << index,
                    groups,
                );
            }
        }
    }
}
