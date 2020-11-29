#[macro_use]
extern crate nom;

use std::collections::HashMap as Map;
use std::io::Write;

use nom::types::CompleteStr;

named!(parse_int<CompleteStr, i64>,
    switch!(take_s!(1),
        CompleteStr("-") => map!(nom::digit, |s| -s.parse::<i64>().unwrap()) |
        CompleteStr(" ") => map!(nom::digit, |s| s.parse::<i64>().unwrap())
    )
);

named!(parse_vector<CompleteStr, (i64, i64)>,
    do_parse!(
           tag_s!("<")  >>
        x: parse_int    >>
           tag_s!(", ") >>
        y: parse_int    >>
           tag_s!(">")  >>
        (x, y)
    )
);

named!(parse_points<CompleteStr, Vec<((i64, i64), (i64, i64))>>,
    separated_list!(
        tag_s!("\n"),
        do_parse!(
                    tag_s!("position=") >>
            position: parse_vector         >>
                    tag_s!(" velocity=") >>
            velocity: parse_vector         >>
            (position, velocity)
        )
    )
);

const INPUT: &'static str = include_str!("input.txt");
const ROWS: i64 = 12;
const COLS: i64 = 80;

fn main() -> Result<(), std::io::Error> {

    let sleep = std::time::Duration::from_millis(500);
    let (_, points) = parse_points(CompleteStr(INPUT)).unwrap();

    let (mut position, velocity): (Map<_, _>, Map<_, _>) = points
        .into_iter()
        .enumerate()
        .map(|(i, (p, v))| ((i, p), (i, v)))
        .unzip();

    let mut grid = vec![vec![false; COLS as usize]; ROWS as usize];
    let stdout = std::io::stdout();
    let mut out = stdout.lock();
    let mut round = 0;

    loop {
        let (mut min_x, mut min_y) = (i64::max_value(), i64::max_value());
        let (mut max_x, mut max_y) = (0, 0);

        for (x, y) in position.values() {
            min_x = i64::min(*x, min_x);
            max_x = i64::max(*x, max_x);
            min_y = i64::min(*y, min_y);
            max_y = i64::max(*y, max_y);
        }

        if max_x - min_x < 200 && max_y - min_y < 200 {
            write!(out, "\nRound {}\n", round)?;
            for row in &mut grid { row.iter_mut().for_each(|b| *b = false) }
            for (x, y) in position.values() {
                let x = (((*x as f32 - min_x as f32) / (max_x as f32 - min_x as f32)) * (COLS as f32 - 1.0)).floor() as usize;
                let y = (((*y as f32 - min_y as f32) / (max_y as f32 - min_y as f32)) * (ROWS as f32 - 1.0)).floor() as usize;
                grid[y][x] = true;
            }
            for row in &grid {
                for c in row {
                    write!(out, "{}", if *c { "#" } else { "." })?;
                }
                write!(out, "\n")?;
            }
            std::thread::sleep(sleep);
        }

        round += 1;
        for (&i, (dx, dy)) in &velocity {
            position.entry(i).and_modify(|(x, y)| { *x += dx; *y += dy; });
        }
    }
}
