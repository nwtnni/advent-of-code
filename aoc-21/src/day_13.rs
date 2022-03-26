use std::cmp;
use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct TransparentOrigami {
    dots: Vec<(i64, i64)>,
    folds: Vec<Fold>,
}

#[derive(Copy, Clone, Debug)]
enum Fold {
    X(i64),
    Y(i64),
}

impl Fro for TransparentOrigami {
    fn fro(input: &str) -> Self {
        let (dots, folds) = input.trim().split_once("\n\n").unwrap();

        let dots = dots
            .trim()
            .split('\n')
            .map(|dot| {
                let (x, y) = dot.split_once(',').unwrap();
                (i64::fro(x), i64::fro(y))
            })
            .collect::<Vec<_>>();

        let folds = folds
            .trim()
            .split('\n')
            .map(|dot| {
                let (axis, line) = dot
                    .trim()
                    .trim_start_matches("fold along ")
                    .split_once('=')
                    .unwrap();

                match axis {
                    "x" => Fold::X(i64::fro(line)),
                    "y" => Fold::Y(i64::fro(line)),
                    _ => unreachable!(),
                }
            })
            .collect::<Vec<_>>();

        Self { dots, folds }
    }
}

impl Solution for TransparentOrigami {
    fn one(mut self) -> i64 {
        self.folds.resize(1, Fold::X(0));
        self.fold();
        self.dots.into_iter().collect::<HashSet<_>>().len() as i64
    }

    fn two(mut self) -> i64 {
        self.fold();

        let dots = self.dots.into_iter().collect::<HashSet<_>>();

        let (min_x, max_x, min_y, max_y) = dots.iter().fold(
            (i64::MAX, i64::MIN, i64::MAX, i64::MIN),
            |(min_x, max_x, min_y, max_y), (x, y)| {
                (
                    cmp::min(min_x, *x),
                    cmp::max(max_x, *x),
                    cmp::min(min_y, *y),
                    cmp::max(max_y, *y),
                )
            },
        );

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if dots.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }

        panic!("See `stdout` for solution");
    }
}

impl TransparentOrigami {
    fn fold(&mut self) {
        for fold in &self.folds {
            match *fold {
                Fold::X(line) => self
                    .dots
                    .iter_mut()
                    .filter(|(x, _)| *x > line)
                    .for_each(|(x, _)| *x = 2 * line - *x),
                Fold::Y(line) => self
                    .dots
                    .iter_mut()
                    .filter(|(_, y)| *y > line)
                    .for_each(|(_, y)| *y = 2 * line - *y),
            }
        }
    }
}
