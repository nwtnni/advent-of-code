use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct GiantSquid {
    order: Vec<i64>,
    boards: Vec<Board>,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Board([[(i64, bool); 5]; 5]);

impl Fro for GiantSquid {
    fn fro(input: &str) -> Self {
        let mut sections = input
            .trim()
            .split("\n\n");

        let numbers = sections.give();
        let order = numbers
            .split(',')
            .map(i64::fro)
            .collect::<Vec<_>>();

        let mut boards = Vec::new();
        for section in sections {
            let mut board = [[(0, false); 5]; 5];

            for (i, row) in section.trim().split('\n').enumerate() {
                for (j, col) in row.trim().split_whitespace().map(i64::fro).enumerate() {
                    board[i][j] = (col, false);
                }
            }
            boards.push(Board(board));
        }

        Self {
            order,
            boards,
        }
    }
}

impl Solution for GiantSquid {
    fn one(mut self) -> i64 {
        for called in &self.order {
            for board in &mut self.boards {
                for row in &mut board.0 {
                    for (number, marked) in row {
                        if number == called {
                            *marked = true
                        }
                    }
                }

                if let Some(won) = won(&board) {
                    return won * called;
                }
            }
        }

        unreachable!()
    }

    fn two(mut self) -> i64 {
        let mut remove = HashSet::new();

        for called in &self.order {
            let len = self.boards.len();
            for board in &mut self.boards {
                for row in &mut board.0 {
                    for (number, marked) in row {
                        if number == called {
                            *marked = true
                        }
                    }
                }

                match (won(&board), len == 1) {
                    (Some(won), true) => return won * called,
                    (None, _) => continue,
                    (Some(_), false) => { remove.insert(*board); },
                }
            }

            self.boards.retain(|board| !remove.contains(board));
        }

        unreachable!()
    }
}

fn won(board: &Board) -> Option<i64> {
    if board.0.iter().any(|row| row.iter().all(|(_, marked)| *marked)) {
        return board
            .0
            .iter()
            .map(|row| row.iter().filter_map(|(number, marked)| if *marked { None } else { Some(number) }).sum::<i64>())
            .sum::<i64>()
            .tap(Some);
    }


    if (0..5).any(|col| board.0.iter().all(|row| row[col].1)) {
        return board
            .0
            .iter()
            .map(|row| row.iter().filter_map(|(number, marked)| if *marked { None } else { Some(number) }).sum::<i64>())
            .sum::<i64>()
            .tap(Some);
    }

    None
}
