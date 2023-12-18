use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct LavaductLagoon(Vec<(Dir, i64, Dir, i64)>);

impl Fro for LavaductLagoon {
    fn fro(input: &str) -> Self {
        input
            .lines()
            .map(|line| {
                let mut iter = line.split_whitespace();
                let dir = match iter.give() {
                    "R" => Dir::E,
                    "D" => Dir::S,
                    "L" => Dir::W,
                    "U" => Dir::N,
                    _ => unreachable!(),
                };

                let d = iter.give().to::<i64>();
                let c = iter.give().trim_start_matches("(#").trim_end_matches(')');

                (
                    dir,
                    d,
                    match u8::from_str_radix(&c[5..6], 16).unwrap() {
                        0 => Dir::E,
                        1 => Dir::S,
                        2 => Dir::W,
                        3 => Dir::N,
                        _ => unreachable!(),
                    },
                    i64::from_str_radix(&c[0..5], 16).unwrap(),
                )
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for LavaductLagoon {
    fn one(self) -> i64 {
        let mut trench = HashSet::new();

        let mut here = Pos::from_i_j(0, 0);

        trench.insert(here);

        for (dir, distance, _, _) in &self.0 {
            for d in 1..=*distance {
                trench.insert(here.shiftn(*dir, d));
            }

            here.shiftn_mut(*dir, *distance);
        }

        flood(&mut trench);
        trench.len() as i64
    }

    fn two(self) -> i64 {
        let mut here = Pos::from_i_j(0, 0);
        let mut total = 0;
        for (_, _, dir, distance) in &self.0 {
            let next = here.shiftn(*dir, *distance);
            total += *distance;
            total += (here.y + next.y) * (here.x - next.x);
            here = next;
        }

        total / 2 + 1
    }
}

fn flood(trench: &mut HashSet<Pos>) {
    let mut stack = Vec::new();
    stack.push(Pos::from_i_j(1, 1));
    while let Some(next) = stack.pop() {
        for dir in Dir::all() {
            let next = next.shift(dir);
            match trench.get(&next) {
                None => {
                    trench.insert(next);
                    stack.push(next);
                }
                Some(_) => continue,
            }
        }
    }
}
