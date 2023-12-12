use std::collections::HashMap;
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
    fn one(mut self) -> i64 {
        self.0
            .iter_mut()
            .inspect(|(springs, target)| {
                debug2(springs);
                debug3(target);
            })
            .map(|(springs, targets)| {
                let mut cache = Vec::new();
                let a = solve(springs, targets);
                let b = ways(springs, &mut cache, targets, 0);
                if dbg!(a) != dbg!(b) {
                    panic!();
                }
                b
            })
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

// Dynamic programming implementation
fn solve(springs: &[Option<bool>], targets: &[i64]) -> i64 {
    let mut table = vec![vec![0i64; springs.len()]; targets.len()];

    for j in 0..springs.len() {
        if fits(springs, j..j + targets[0] as usize, targets.len() == 1) {
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
                if fits(springs, j..j + *target as usize, i == targets.len() - 1) {
                    table[i][j] += ways;
                }

                if matches!(springs[j], Some(true)) {
                    break;
                }
            }
        }
    }

    table.last().unwrap().iter().sum()
}

// Recursive non-memoized solution, has right time complexity
// but fails to discount solutions that leave extra springs at the end.
#[allow(dead_code)]
fn solve2(springs: &[Option<bool>], targets: &[i64], index: usize, start: usize) -> i64 {
    if index >= targets.len() {
        return 1;
    }

    let mut ways = 0;

    for start in start..springs.len() {
        if fits(
            springs,
            start..start + targets[index] as usize,
            index == targets.len() - 1,
        ) {
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

fn fits(springs: &[Option<bool>], range: Range<usize>, last: bool) -> bool {
    // Check after the end
    if range.end > springs.len() || matches!(springs.get(range.end), Some(Some(true))) {
        return false;
    }

    // Disallow trailing springs
    if last
        && springs[range.end..]
            .iter()
            .any(|spring| matches!(spring, Some(true)))
    {
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

// Naive solution, works but too slow
#[allow(dead_code)]
fn ways(
    input: &mut Vec<Option<bool>>,
    cache: &mut Vec<i64>,
    target: &[i64],
    mut index: usize,
) -> i64 {
    while index < input.len() && input[index].is_some() {
        index += 1;
    }

    if index == input.len() {
        cache.clear();
        let mut i = 0;
        while i < input.len() {
            let mut j = i + 1;
            while j <= input.len() {
                if input[i..j].iter().all(|a| matches!(a, Some(true))) {
                    j += 1;
                } else {
                    break;
                }
            }

            if j - 1 - i > 0 {
                cache.push((j - 1 - i) as i64);
            }
            i = j;
        }

        if cache == target {
            return 1;
        } else {
            return 0;
        }
    }

    input[index] = Some(true);
    let a = ways(input, cache, target, index);

    input[index] = Some(false);
    let b = ways(input, cache, target, index);

    input[index] = None;
    a + b
}

// Equivalent to naive solution, but tried to track
// runs as we recurse instead of clearing every time.
#[allow(dead_code)]
fn ways2(
    input: &mut Vec<Option<bool>>,
    // inv: cache contains all runs up until index
    // problem: question mark may extend thruogh several calls to ways2
    // if index contains some(false), can count previous run
    // what if several runs are already set
    cache: &mut Vec<i64>,
    target: &[i64],
    mut run: Option<i64>,
    start: usize,
) -> i64 {
    // pop current run
    if start == input.len() {
        if let Some(run) = run {
            cache.push(run);
            let out = (cache == target) as i64;
            cache.pop();
            return out;
        } else {
            return (cache == target) as i64;
        }
    }

    match input[start] {
        Some(true) => {
            let r = run.get_or_insert(0);
            *r += 1;

            // more than target OR current run exceeds next target run
            if cache.len() + 1 > target.len() || target[cache.len()] < *r {
                return 0;
            }

            ways2(input, cache, target, run, start + 1)
        }
        Some(false) => {
            if let Some(run) = run {
                cache.push(run);
            }

            if cache.len() > target.len() || cache != &target[0..cache.len()] {
                if run.is_some() {
                    cache.pop();
                }
                0
            } else {
                let out = ways2(input, cache, target, None, start + 1);
                if run.is_some() {
                    cache.pop();
                }
                out
            }
        }
        None => {
            input[start] = Some(true);
            let a = ways2(input, cache, target, run, start);

            input[start] = Some(false);
            let b = ways2(input, cache, target, run, start);

            input[start] = None;
            a + b
        }
    }
}

// Recursive memoized solution, has right time complexity
// but fails to discount solutions that leave extra springs at the end.
#[allow(dead_code)]
fn ways3(
    input: &mut Vec<Option<bool>>,
    target: &[i64],
    index: usize,
    start: usize,
    memo: &mut HashMap<(usize, usize), i64>,
) -> i64 {
    if let Some(ways) = memo.get(&(index, start)) {
        return *ways;
    }

    // debug(input);
    if index >= target.len() {
        debug2(input);
        return 1;
    }

    let next = target[index];

    let mut ways = 0;
    for (j, i) in input
        .iter()
        .enumerate()
        .skip(start)
        .skip_while(|(_, a)| matches!(a, Some(false)))
        .enumerate()
        .filter(|(j, (i, _))| {
            if *i + next as usize > input.len() {
                return false;
            }

            if input[i - j..*i].iter().any(|a| matches!(a, Some(true))) {
                return false;
            }

            input[*i..*i + next as usize]
                .iter()
                .all(|b| matches!(b, None | Some(true)))
                && match input.get(*i + next as usize) {
                    None | Some(Some(false)) | Some(None) => true,
                    Some(Some(true)) => false,
                }
        })
        .map(|(j, (i, _))| (j, i))
        .collect::<Vec<_>>()
    {
        let save_b = input[i - j..i].to_vec();
        input[i - j..i].iter_mut().for_each(|b| *b = Some(false));

        let save = input[i..i + next as usize].to_vec();
        input[i..i + next as usize]
            .iter_mut()
            .for_each(|b| *b = Some(true));

        let save_a = input.get(i + next as usize).copied();
        if let Some(a) = input.get_mut(i + next as usize) {
            *a = Some(false);
        }

        ways += ways3(input, target, index + 1, i + next as usize + 1, memo);
        input[i - j..i].copy_from_slice(&save_b);
        input[i..i + next as usize].copy_from_slice(&save);
        if let Some(a) = input.get_mut(i + next as usize) {
            *a = save_a.unwrap();
        }
    }

    debug2(input);
    memo.insert((index, start), ways);
    ways
}

#[allow(dead_code)]
fn debug(table: &[Vec<i64>]) {
    for line in table {
        for entry in line {
            print!("{} ", entry);
        }
        println!();
    }
    println!();
}

#[allow(dead_code)]
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

#[allow(dead_code)]
fn debug3(targets: &[i64]) {
    for t in targets {
        print!("{} ", t);
    }
    println!();
}
