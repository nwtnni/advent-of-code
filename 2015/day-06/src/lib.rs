use std::usize;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn to(self, other: Pos) -> impl Iterator<Item = Pos> {
        let lx = usize::min(self.x, other.x);
        let hx = usize::max(self.x, other.x);
        let ly = usize::min(self.y, other.y);
        let hy = usize::max(self.y, other.y);
        (ly..=hy).flat_map(move |y| {
            (lx..=hx).map(move |x| {
                Pos { x, y }
            })
        })
    }

    pub fn parse(s: &str) -> Self {
        let mut iter = s.split(',')
            .map(usize::from_str)
            .filter_map(Result::ok);
        let x = iter.next().unwrap();
        let y = iter.next().unwrap();
        Pos { x, y }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mode {
    On,
    Toggle,
    Off,
}

impl Mode {
    pub fn parse<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Self {
        match iter.next() {
        | Some("turn") => {
            match iter.next() {
            | Some("on") => Mode::On,
            | Some("off") => Mode::Off,
            | _ => unreachable!(),
            }
        }
        | Some("toggle") => Mode::Toggle,
        | _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Instruction {
    pub start: Pos,
    pub end: Pos,
    pub mode: Mode,
}

impl Instruction {
    pub fn parse(s: &str) -> Self {
        let mut iter = s.split_whitespace();
        let mode = Mode::parse(&mut iter);
        let start = iter.next()
            .map(Pos::parse)
            .unwrap();
        let _ = iter.next();
        let end = iter.next()
            .map(Pos::parse)
            .unwrap();
        Instruction { start, end, mode }
    }
}
