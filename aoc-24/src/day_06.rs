use std::collections::HashMap;
use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct GuardGallivant {
    cols: usize,
    rows: usize,
    grid: HashSet<Pos>,
    pos: Pos,
    dir: Dir,
}

impl Fro for GuardGallivant {
    fn fro(input: &str) -> Self {
        let mut grid = HashSet::new();
        let mut rows = 0;
        let mut cols = 0;
        let mut guard = Pos::default();
        for (y, row) in input.trim().split('\n').enumerate() {
            rows += 1;
            cols = 0;
            for (x, col) in row.trim().chars().enumerate() {
                cols += 1;
                let pos = Pos {
                    x: x as i64,
                    y: y as i64,
                };

                match col {
                    '#' => {
                        grid.insert(pos);
                    }
                    '^' => guard = pos,
                    _ => (),
                }
            }
        }
        Self {
            cols,
            rows,
            grid,
            pos: guard,
            dir: Dir::N,
        }
    }
}

impl Solution for GuardGallivant {
    fn one(self) -> i64 {
        match self.visit(self.pos, self.dir, &HashMap::new()) {
            Outcome::Loop => unreachable!(),
            Outcome::Escape(count) => count,
        }
    }

    fn two(mut self) -> i64 {
        let mut seen = HashMap::<_, Vec<_>>::new();
        let mut obstacles = HashSet::new();

        let mut pos = self.pos;
        let mut dir = self.dir;

        loop {
            let mut next_dir = dir;
            let mut next_pos = pos.shift(next_dir);

            while self.grid.contains(&next_pos) {
                next_dir.rotate_clockwise_mut();
                next_pos = pos.shift(next_dir);
            }

            if !self.contains(next_pos) {
                return obstacles.len() as i64;
            }

            // Try adding obstacle in path
            if !seen.contains_key(&next_pos) {
                self.grid.insert(next_pos);
                if let Outcome::Loop = self.visit(pos, dir, &seen) {
                    obstacles.insert(next_pos);
                }
                self.grid.remove(&next_pos);
            }

            seen.entry(pos).or_default().push(dir);

            dir = next_dir;
            pos = next_pos;
        }
    }
}

enum Outcome {
    Escape(i64),
    Loop,
}

impl GuardGallivant {
    fn visit(&self, mut pos: Pos, mut dir: Dir, old: &HashMap<Pos, Vec<Dir>>) -> Outcome {
        let mut new = HashMap::<_, Vec<_>>::new();

        loop {
            if old
                .get(&pos)
                .map(|dirs| dirs.contains(&dir))
                .unwrap_or(false)
            {
                return Outcome::Loop;
            }

            let dirs = new.entry(pos).or_default();
            if dirs.contains(&dir) {
                return Outcome::Loop;
            } else {
                dirs.push(dir);
            }

            let mut next = pos.shift(dir);
            while self.grid.contains(&next) {
                dir.rotate_clockwise_mut();
                next = pos.shift(dir);
            }

            if !self.contains(next) {
                return Outcome::Escape((old.len() + new.len()) as i64);
            }

            pos = next;
        }
    }

    fn contains(&self, pos: Pos) -> bool {
        (0..self.cols as i64).contains(&pos.x) && (0..self.rows as i64).contains(&pos.y)
    }

    fn debug(&self, pos: Pos, dir: Dir) {
        eprint!("\x1B[2J");
        for y in 0..self.rows as i64 {
            for x in 0..self.cols as i64 {
                if self.grid.contains(&Pos { x, y }) {
                    eprint!("#");
                } else if x == pos.x && y == pos.y {
                    eprint!(
                        "\x1B[31m{}\x1B[0m",
                        match dir {
                            Dir::N => '^',
                            Dir::S => 'v',
                            Dir::E => '>',
                            Dir::W => '<',
                        }
                    )
                } else {
                    eprint!(" ");
                }
            }
            eprintln!();
        }
    }
}
