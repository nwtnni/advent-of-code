use aoc::*;

#[derive(Clone, Debug)]
pub struct GiantSquid {
    order: Vec<i64>,
    boards: Vec<Board>,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Board([[Tile; 5]; 5]);

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Tile {
    number: i64,
    marked: bool,
}

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
            let mut board = [[Tile { number: 0, marked: false }; 5]; 5];

            for (i, row) in section.trim().split('\n').enumerate() {
                for (j, col) in row.trim().split_whitespace().map(i64::fro).enumerate() {
                    board[i][j].number = col;
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
            self.boards
                .iter_mut()
                .flat_map(|board| board.0.iter_mut())
                .flat_map(|row| row.iter_mut())
                .filter(|tile| tile.number == *called)
                .for_each(|tile| tile.marked = true);

            if let Some(board) = self.boards.iter().find(|board| won(board)) {
                return called * unmarked(board);
            }
        }

        unreachable!()
    }

    fn two(mut self) -> i64 {
        for called in &self.order {
            self.boards
                .iter_mut()
                .flat_map(|board| board.0.iter_mut())
                .flat_map(|row| row.iter_mut())
                .filter(|tile| tile.number == *called)
                .for_each(|tile| tile.marked = true);

            if self.boards.len() > 1 {
                self.boards.retain(|board| !won(board));
                continue;
            }

            if won(&self.boards[0]) {
                return called * unmarked(&self.boards[0]);
            }
        }

        unreachable!()
    }
}

fn won(board: &Board) -> bool {
    let row = board.0.iter().any(|row| row.iter().all(|tile| tile.marked));
    let col = (0..5).any(|col| board.0.iter().all(|row| row[col].marked));
    row || col
}

fn unmarked(board: &Board) -> i64 {
    board
        .0
        .iter()
        .flat_map(|row| row.iter())
        .filter(|tile| !tile.marked)
        .map(|tile| tile.number)
        .sum::<i64>()
}
