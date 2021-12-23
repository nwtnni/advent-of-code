use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::iter;

use aoc::*;
use priority_queue::PriorityQueue;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Amphipod {
    halls: [Option<Type>; 11],
    rooms: [[Option<Type>; 2]; 4],
    last: Option<Tile>,
}

impl Fro for Amphipod {
    fn fro(input: &str) -> Self {
        let mut iter = input.trim().split('\n').skip(2);

        let halls = [None; 11];
        let mut rooms = [[None; 2]; 4];

        for (i, above) in iter
            .next()
            .into_iter()
            .flat_map(|line| line.chars())
            .filter_map(Type::from_char)
            .enumerate()
        {
            rooms[i][0] = Some(above);
        }

        for (i, below) in iter
            .next()
            .into_iter()
            .flat_map(|line| line.chars())
            .filter_map(Type::from_char)
            .enumerate()
        {
            rooms[i][1] = Some(below);
        }

        Self {
            halls,
            rooms,
            last: None,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Type {
    A,
    B,
    C,
    D,
}

impl Type {
    fn from_char(char: char) -> Option<Self> {
        match char {
            'A' => Some(Type::A),
            'B' => Some(Type::B),
            'C' => Some(Type::C),
            'D' => Some(Type::D),
            _ => None,
        }
    }

    fn energy(&self) -> i64 {
        match self {
            Type::A => 1,
            Type::B => 10,
            Type::C => 100,
            Type::D => 1000,
        }
    }
}

impl Amphipod {
    fn step(&self, buffer: &mut Vec<(Self, i64)>) {
        for from in (0..11).map(Tile::Hall).chain([
            Tile::Room(0, 0),
            Tile::Room(0, 1),
            Tile::Room(1, 0),
            Tile::Room(1, 1),
            Tile::Room(2, 0),
            Tile::Room(2, 1),
            Tile::Room(3, 0),
            Tile::Room(3, 1),
        ]) {
            if let Some(r#type) = self.get(from) {
                match (r#type, from) {
                    (Type::A, Tile::Room(0, 1)) => continue,
                    (Type::A, Tile::Room(0, 0)) if self.get(Tile::Room(0, 1)) == Some(Type::A) => {
                        continue
                    }

                    (Type::B, Tile::Room(1, 1)) => continue,
                    (Type::B, Tile::Room(1, 0)) if self.get(Tile::Room(1, 1)) == Some(Type::B) => {
                        continue
                    }

                    (Type::C, Tile::Room(2, 1)) => continue,
                    (Type::C, Tile::Room(2, 0)) if self.get(Tile::Room(2, 1)) == Some(Type::C) => {
                        continue
                    }

                    (Type::D, Tile::Room(3, 1)) => continue,
                    (Type::D, Tile::Room(3, 0)) if self.get(Tile::Room(3, 1)) == Some(Type::D) => {
                        continue
                    }
                    _ => (),
                }

                for (to, cost) in self.movable(from, r#type) {
                    let mut next = self.clone();
                    *next.get_mut(from) = None;
                    *next.get_mut(to) = Some(r#type);
                    next.last = Some(to);
                    buffer.push((next, cost));
                }
            }
        }
    }

    fn movable(&self, from: Tile, r#type: Type) -> impl Iterator<Item = (Tile, i64)> + '_ {
        let mut all = HashMap::new();

        self.reachable(from, &mut all, 0);

        all.into_iter()
            .filter(move |(tile, _)| {
                // Constraint: cannot move around in hallway unless last to move
                match (from, tile) {
                    (_, Tile::Room(_, _)) => true,
                    (Tile::Hall(_), _) => self.last.map_or(true, |last| last == from),
                    (_, _) => true,
                }
            })
            .filter(|(tile, _)| {
                // Constraint: cannot stop outside room
                match tile {
                    Tile::Hall(2) => false,
                    Tile::Hall(4) => false,
                    Tile::Hall(6) => false,
                    Tile::Hall(8) => false,
                    _ => true,
                }
            })
            .filter(move |(tile, _)| {
                // Constraint: cannot move into non-destination
                match (tile, r#type) {
                    (Tile::Room(0, _), Type::A) => true,
                    (Tile::Room(0, _), _) => false,

                    (Tile::Room(1, _), Type::B) => true,
                    (Tile::Room(1, _), _) => false,

                    (Tile::Room(2, _), Type::C) => true,
                    (Tile::Room(2, _), _) => false,

                    (Tile::Room(3, _), Type::D) => true,
                    (Tile::Room(3, _), _) => false,

                    _ => true,
                }
            })
            .filter(move |(tile, _)| {
                // Constraint: cannot move into room with other type
                match tile {
                    Tile::Hall(_) => true,
                    // Optimization: don't move into 0th room if 1st is available
                    Tile::Room(i, 0) => match self.get(Tile::Room(*i, 1)) {
                        None => false,
                        Some(other) => other == r#type,
                    },
                    Tile::Room(_, 1) => true,
                    _ => unreachable!(),
                }
            })
            .map(move |(tile, distance)| (tile, distance * r#type.energy()))
    }

    fn reachable(&self, from: Tile, all: &mut HashMap<Tile, i64>, distance: i64) {
        all.insert(from, distance);

        for next in self.around(from) {
            // Already visited
            if all.contains_key(&next) {
                continue;
            }

            // Constraint: cannot pass through others
            if self.get(next).is_some() {
                continue;
            }

            self.reachable(next, all, distance + 1);
        }
    }

    fn around(&self, tile: Tile) -> impl Iterator<Item = Tile> {
        match tile {
            Tile::Hall(0) => Or::L(iter::once(Tile::Hall(1))),

            Tile::Hall(1) => Or::R(Or::L([Tile::Hall(0), Tile::Hall(2)].into_iter())),
            Tile::Hall(2) => Or::R(Or::R(
                [Tile::Hall(1), Tile::Room(0, 0), Tile::Hall(3)].into_iter(),
            )),

            Tile::Hall(3) => Or::R(Or::L([Tile::Hall(2), Tile::Hall(4)].into_iter())),
            Tile::Hall(4) => Or::R(Or::R(
                [Tile::Hall(3), Tile::Room(1, 0), Tile::Hall(5)].into_iter(),
            )),

            Tile::Hall(5) => Or::R(Or::L([Tile::Hall(4), Tile::Hall(6)].into_iter())),
            Tile::Hall(6) => Or::R(Or::R(
                [Tile::Hall(5), Tile::Room(2, 0), Tile::Hall(7)].into_iter(),
            )),

            Tile::Hall(7) => Or::R(Or::L([Tile::Hall(6), Tile::Hall(8)].into_iter())),
            Tile::Hall(8) => Or::R(Or::R(
                [Tile::Hall(7), Tile::Room(3, 0), Tile::Hall(9)].into_iter(),
            )),

            Tile::Hall(9) => Or::R(Or::L([Tile::Hall(8), Tile::Hall(10)].into_iter())),
            Tile::Hall(10) => Or::L(iter::once(Tile::Hall(9))),

            Tile::Room(i, 0) => Or::R(Or::L([Tile::Hall(2 * i + 2), Tile::Room(i, 1)].into_iter())),
            Tile::Room(i, 1) => Or::L(iter::once(Tile::Room(i, 0))),

            _ => unreachable!(),
        }
    }

    fn get(&self, tile: Tile) -> Option<Type> {
        match tile {
            Tile::Hall(i) => self.halls[i],
            Tile::Room(i, j) => self.rooms[i][j],
        }
    }

    fn get_mut(&mut self, tile: Tile) -> &mut Option<Type> {
        match tile {
            Tile::Hall(i) => &mut self.halls[i],
            Tile::Room(i, j) => &mut self.rooms[i][j],
        }
    }

    fn is_finished(&self) -> bool {
        self.rooms[0][0] == Some(Type::A)
            && self.rooms[0][1] == Some(Type::A)
            && self.rooms[1][0] == Some(Type::B)
            && self.rooms[1][1] == Some(Type::B)
            && self.rooms[2][0] == Some(Type::C)
            && self.rooms[2][1] == Some(Type::C)
            && self.rooms[3][0] == Some(Type::D)
            && self.rooms[3][1] == Some(Type::D)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Tile {
    Hall(usize),
    Room(usize, usize),
}

impl Solution for Amphipod {
    fn one(self) -> i64 {
        let mut seen = HashSet::new();
        let mut queue = PriorityQueue::<Self, cmp::Reverse<i64>>::new();
        let mut buffer = Vec::new();

        seen.insert(self.clone());
        queue.push(self, cmp::Reverse(0));

        while let Some((prev, cmp::Reverse(cost))) = queue.pop() {
            if prev.is_finished() {
                return cost;
            }

            prev.step(&mut buffer);
            seen.insert(prev);

            for (next, delta) in buffer.drain(..) {
                if seen.contains(&next) {
                    continue;
                }

                queue.push_increase(next, cmp::Reverse(cost + delta));
            }
        }

        unreachable!()
    }

    fn two(self) -> i64 {
        todo!()
    }
}

impl fmt::Debug for Amphipod {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "#############")?;

        macro_rules! opt {
            ($amphipod:expr) => {
                match $amphipod {
                    None => write!(fmt, "."),
                    Some(r#type) => write!(fmt, "{:?}", r#type),
                }
            };
        }

        write!(fmt, "#")?;
        for hall in self.halls {
            opt!(hall)?;
        }
        writeln!(fmt, "#")?;

        write!(fmt, "##")?;
        for room in 0..4 {
            write!(fmt, "#")?;
            opt!(self.rooms[room][0])?;
        }
        writeln!(fmt, "###")?;

        write!(fmt, "  ")?;
        for room in 0..4 {
            write!(fmt, "#")?;
            opt!(self.rooms[room][1])?;
        }
        writeln!(fmt, "#  ")?;

        writeln!(fmt, "  #########")
    }
}
