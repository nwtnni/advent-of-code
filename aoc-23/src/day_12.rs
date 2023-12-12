use std::ops::Range;

use aoc::*;

#[derive(Clone, Debug)]
pub struct HotSprings(Vec<(Vec<Option<bool>>, Vec<i64>)>);

impl Fro for HotSprings {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let (a, b) = line.trim().split_once(' ').unwrap();
                let a = a
                    .chars()
                    .map(|char| match char {
                        '#' => Some(true),
                        '?' => None,
                        _ => Some(false),
                    })
                    .collect();
                let b = b.split(',').map(i64::fro).collect();
                (a, b)
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for HotSprings {
    fn one(self) -> i64 {
        self.0
            .iter()
            .inspect(|(springs, target)| {
                debug2(springs);
                debug3(target)
            })
            .map(|(springs, targets)| solve2(springs, targets, 0, 0))
            // .map(|(springs, targets)| solve(springs, targets))
            .sum()
    }

    fn two(self) -> i64 {
        self.0
            .into_iter()
            .map(|(springs, targets)| {
                let springs = springs
                    .iter()
                    .copied()
                    .chain(std::iter::once(None))
                    .cycle()
                    .take(springs.len() * 5 + 4)
                    .collect::<Vec<_>>();

                let targets = targets
                    .iter()
                    .copied()
                    .cycle()
                    .take(targets.len() * 5)
                    .collect::<Vec<_>>();

                solve(&springs, &targets)
            })
            .sum()
    }
}

fn solve2(springs: &[Option<bool>], targets: &[i64], index: usize, start: usize) -> i64 {
    if index >= targets.len() {
        return 1;
    }

    let mut ways = 0;

    for start in start..springs.len() {
        if fits(springs, start..start + targets[index] as usize) {
            ways += solve2(
                springs,
                targets,
                index + 1,
                start + targets[index] as usize + 1,
            );
        }

        if matches!(springs[start], Some(true)) {
            break;
        }
    }

    ways
}

fn solve(springs: &[Option<bool>], targets: &[i64]) -> i64 {
    let mut table = vec![vec![0i64; springs.len()]; targets.len()];

    for j in 0..springs.len() {
        if fits(springs, j..j + targets[0] as usize) {
            table[0][j] = 1;
        }

        if matches!(springs[j], Some(true)) {
            break;
        }
    }

    for (i, target) in targets.iter().enumerate().skip(1) {
        for (start, ways) in table[i - 1]
            .iter()
            .enumerate()
            .filter(|(_, ways)| **ways != 0)
            .map(|(j, ways)| (j + targets[i - 1] as usize + 1, *ways))
            .collect::<Vec<_>>()
        {
            for j in start..springs.len() {
                if fits(springs, j..j + *target as usize) {
                    table[i][j] += ways;
                }

                if matches!(springs[j], Some(true)) {
                    break;
                }
            }
        }
    }

    debug(&table);
    table.last().unwrap().iter().sum()
}

fn debug(table: &[Vec<i64>]) {
    for line in table {
        for entry in line {
            print!("{} ", entry);
        }
        println!();
    }
    println!();
}

fn debug2(springs: &[Option<bool>]) {
    for s in springs {
        match s {
            None => print!("?"),
            Some(true) => print!("#"),
            Some(false) => print!("."),
        }
        print!(" ");
    }
    println!();
}

fn debug3(targets: &[i64]) {
    for t in targets {
        print!("{} ", t);
    }
    println!();
}

fn fits(springs: &[Option<bool>], range: Range<usize>) -> bool {
    // Check after the end
    if range.end > springs.len() || matches!(springs.get(range.end), Some(Some(true))) {
        return false;
    }

    // Check before the start
    if range
        .start
        .checked_sub(1)
        .map_or(false, |start| matches!(springs[start], Some(true)))
    {
        return false;
    }

    // Check within the range
    if springs[range]
        .iter()
        .any(|spring| matches!(spring, Some(false)))
    {
        return false;
    }

    true
}
