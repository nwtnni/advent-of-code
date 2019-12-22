use std::collections::HashSet;

use aoc::*;

pub struct SetAndForget(intcode::Program);

impl Fro for SetAndForget {
    fn fro(input: &str) -> Self {
        SetAndForget(intcode::Program::fro(input))
    }
}

fn parse_dir(output: i64) -> Dir {
    match output as u8 {
    | b'^' => Dir::N,
    | b'v' => Dir::S,
    | b'>' => Dir::E,
    | b'<' => Dir::W,
    | _ => unreachable!(),
    }
}

impl SetAndForget {
    fn scaffold(&mut self) -> (HashSet<Pos>, Pos, Dir) {
        let mut scaffold = HashSet::new();
        let mut pos = Pos::default();
        let mut dir = Dir::N;
        let mut x = 0;
        let mut y = 0;
        while let Some(next) = self.0.output() {
            match next as u8 {
            | b'#'  => { scaffold.insert(Pos { x, y }); x += 1; }
            | b'\n' => { y += 1; x = 0; }
            | b'v' | b'<' | b'>' | b'^' => {
                dir = parse_dir(next);
                pos = Pos { x, y };
                scaffold.insert(pos);
                x += 1;
            }
            | _ => { x += 1; }
            }
        }
        (scaffold, pos, dir)

    }
}

impl Solution for SetAndForget {
    fn one(mut self) -> i64 {
        let (scaffold, _, _) = self.scaffold();
        let mut sum = 0;
        let get = |x, y| scaffold.contains(&Pos { x, y });
        for y in 0..80i64 {
            for x in 0..80i64 {
                if get(x, y)
                && get(x + 1, y) && get(x - 1, y)
                && get(x, y + 1) && get(x, y - 1) {
                    sum += x * y;
                }
            }
        }
        sum
    }

    fn two(mut self) -> i64 {
        let (scaffold, pos, dir) = self.scaffold();
        for y in (0..80).rev() {
            for x in 0..80 {
                if x == pos.x && y == pos.y {
                    match dir {
                    | Dir::N => print!("^"),
                    | Dir::S => print!("v"),
                    | Dir::E => print!(">"),
                    | Dir::W => print!("<"),
                    }
                } else if scaffold.contains(&Pos { x, y }) {
                    print!("█");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
        0
    }
}
