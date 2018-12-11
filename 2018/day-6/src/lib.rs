use std::str::FromStr;

pub const INPUT: &'static str = include_str!("input.txt");

#[derive(Copy, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn parse(s: &str) -> Self {
        let mut iter = s.split(", ");
        let x = iter.next()
            .map(FromStr::from_str)
            .map(Result::unwrap)
            .unwrap();
        let y = iter.next()
            .map(FromStr::from_str)
            .map(Result::unwrap)
            .unwrap();
        Point { x, y }
    }
}

pub fn dist(p1: Point, p2: Point) -> isize {
    let dx = p1.x - p2.x;
    let dy = p1.y - p2.y;
    isize::abs(dx) + isize::abs(dy)
}
