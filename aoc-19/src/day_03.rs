use std::str;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Placeholder(Vec<Wire>, Vec<Wire>);

#[derive(Copy, Clone, Debug)]
struct Wire {
  dir: aoc::Dir,
  len: i32,
}

impl str::FromStr for Wire {
  type Err = aoc::Error;
  fn from_str(input: &str) -> Result<Self, Self::Err> {
    let mut iter = input.chars();
    let dir = match iter.next() {
    | Some('R') => aoc::Dir::E,
    | Some('D') => aoc::Dir::S,
    | Some('U') => aoc::Dir::N,
    | Some('L') => aoc::Dir::W,
    | _ => unreachable!(),
    };
    let len = iter.as_str()
      .parse::<i32>()
      .unwrap();
    Ok(Wire {
      dir,
      len,
    })
  }
}

impl str::FromStr for Placeholder {
    type Err = aoc::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
      let mut wires = input.split_whitespace();
      let l = wires.next()
        .unwrap()
        .split(',')
        .map(|wire| wire.parse::<Wire>().unwrap())
        .collect();
      let r = wires.next()
        .unwrap()
        .split(',')
        .map(|wire| wire.parse::<Wire>().unwrap())
        .collect();
      Ok(Placeholder(l, r))
    }
}

impl aoc::Solution for Placeholder {
    fn one(self) -> i32 {
      let mut l = HashSet::new();
      let mut pos = aoc::Pos::default();
      for wire in &self.0 {
        for _ in 0..wire.len {
          pos = pos.shift(wire.dir);
          l.insert(pos);
        }
      }

      let mut r = HashSet::new();
      let mut pos = aoc::Pos::default();
      for wire in &self.1 {
        for _ in 0..wire.len {
          pos = pos.shift(wire.dir);
          r.insert(pos);
        }
      }

      let p = l.intersection(&r)
        .min_by_key(|p| p.x.abs() + p.y.abs())
        .unwrap();

      p.x.abs() + p.y.abs()
    }

    fn two(self) -> i32 {
      let mut ls = HashSet::new();
      let mut lm = HashMap::new();
      let mut pos = aoc::Pos::default();
      let mut steps = 0;
      for wire in &self.0 {
        for _ in 0..wire.len {
          pos = pos.shift(wire.dir);
          steps += 1;
          ls.insert(pos);
          lm.entry(pos).or_insert(steps);
        }
      }

      let mut rs = HashSet::new();
      let mut rm = HashMap::new();
      let mut pos = aoc::Pos::default();
      let mut steps = 0;
      for wire in &self.1 {
        for _ in 0..wire.len {
          pos = pos.shift(wire.dir);
          steps += 1;
          rs.insert(pos);
          rm.entry(pos).or_insert(steps);
        }
      }

      let p = ls.intersection(&rs)
        .min_by_key(|p| lm.get(p).unwrap() + rm.get(p).unwrap())
        .unwrap();

      lm.get(p).unwrap() + rm.get(p).unwrap()
    }
}
