use aoc::*;

#[derive(Clone, Debug)]
pub struct LetItSnow {
    row: usize,
    col: usize,
}

impl Fro for LetItSnow {
    fn fro(input: &str) -> Self {
        let (row, col) = input
            .trim()
            .trim_start_matches(
                "To continue, please consult the code grid in the manual.  Enter the code at row ",
            )
            .trim_end_matches('.')
            .split_once(", column ")
            .unwrap();

        Self {
            row: usize::fro(row),
            col: usize::fro(col),
        }
    }
}

impl Solution for LetItSnow {
    fn one(self) -> i64 {
        let mut code = 20151125;

        for sum in 3.. {
            for col in 1..sum {
                let row = sum - col;

                code = (code * 252533) % 33554393;

                if row == self.row && col == self.col {
                    return code;
                }
            }
        }

        unreachable!()
    }

    fn two(self) -> i64 {
        unreachable!()
    }
}
