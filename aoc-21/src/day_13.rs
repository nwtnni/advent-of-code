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
        let (dots, folds) = input
            .trim()
            .split_once("\n\n")
            .unwrap();

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

        for fold in self.folds.into_iter().take(1) {
            for (x, y) in &mut self.dots {
                match fold {
                    Fold::X(line) => {
                        if *x > line {
                            let delta = *x - line;
                            *x = line - delta;
                        }
                    },
                    Fold::Y(line) => {
                        if *y > line {
                            let delta = *y - line;
                            *y = line - delta;
                        }
                    }
                }
            }
        }

        self.dots.into_iter().collect::<HashSet<_>>().len() as i64
    }

    fn two(mut self) -> i64 {
        for fold in self.folds {
            for (x, y) in &mut self.dots {
                match fold {
                    Fold::X(line) => {
                        if *x > line {
                            let delta = *x - line;
                            *x = line - delta;
                        }
                    },
                    Fold::Y(line) => {
                        if *y > line {
                            let delta = *y - line;
                            *y = line - delta;
                        }
                    }
                }
            }
        }

        let min_x = *self.dots.iter().map(|(x, _)| x).min().unwrap();
        let max_x = *self.dots.iter().map(|(x, _)| x).max().unwrap();
        let min_y = *self.dots.iter().map(|(_, y)| y).min().unwrap();
        let max_y = *self.dots.iter().map(|(_, y)| y).max().unwrap();
        let dots = self.dots.into_iter().collect::<HashSet<_>>();

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
