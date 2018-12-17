use std::collections::{HashMap as Map, HashSet as Set};

use tap::TapOps;

const INPUT: &'static str = include_str!("input.txt");

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Dir { U, D, L, R, }

impl Dir {
    fn left(&self) -> Self {
        match self {
        | Dir::U => Dir::L,
        | Dir::D => Dir::R,
        | Dir::L => Dir::D,
        | Dir::R => Dir::U,
        }
    }

    fn right(&self) -> Self {
        match self {
        | Dir::U => Dir::R,
        | Dir::D => Dir::L,
        | Dir::L => Dir::U,
        | Dir::R => Dir::D,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Pos {
    y: usize,
    x: usize,
}

impl Pos {
    fn step(&self, dir: Dir) -> Self {
        match dir {
        | Dir::U => Pos { x: self.x, y: self.y - 1 },
        | Dir::D => Pos { x: self.x, y: self.y + 1 },
        | Dir::L => Pos { x: self.x - 1, y: self.y },
        | Dir::R => Pos { x: self.x + 1, y: self.y },
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Cart {
    turns: usize,
    dir: Dir,
}

impl Cart {
    fn go_straight(&self, cross: bool) -> Self {
        let dt = if cross { 1 } else { 0 };
        Cart { turns: self.turns + dt, dir: self.dir }
    }

    fn turn_left(&self, cross: bool) -> Self {
        let dt = if cross { 1 } else { 0 };
        Cart { turns: self.turns + dt, dir: self.dir.left() }
    }

    fn turn_right(&self, cross: bool) -> Self {
        let dt = if cross { 1 } else { 0 };
        Cart { turns: self.turns + dt, dir: self.dir.right() }
    }

    fn step(&self, pos: Pos, track: char) -> (Pos, Self) {
        let cart = match (track, self.dir) {
        | ('-',  _)
        | ('|',  _) => self.go_straight(false),
        | ('/',  Dir::U)
        | ('/',  Dir::D)
        | ('\\', Dir::L)
        | ('\\', Dir::R) => self.turn_right(false),
        | ('/',  Dir::L)
        | ('/',  Dir::R)
        | ('\\', Dir::U)
        | ('\\', Dir::D) => self.turn_left(false),
        | ('+', _) if self.turns % 3 == 0 => self.turn_left(true),
        | ('+', _) if self.turns % 3 == 1 => self.go_straight(true),
        | ('+', _) => self.turn_right(true),
        | _ => unreachable!(),
        };
        (pos.step(cart.dir), cart)
    }

}

fn main() {

    let mut tracks: Map<Pos, char> = Map::default();
    let mut carts: Map<Pos, Cart> = Map::default();

    for (y, line) in INPUT.trim().lines().enumerate() {
        for (x, track) in line.chars().enumerate() {
            let pos = Pos { x, y };
            match track {
            | '<' => {
                carts.insert(pos, Cart { turns: 0, dir: Dir::L });
                tracks.insert(pos, '-');
            }
            | '>' => {
                carts.insert(pos, Cart { turns: 0, dir: Dir::R });
                tracks.insert(pos, '-');
            }
            | '^' => {
                carts.insert(pos, Cart { turns: 0, dir: Dir::U });
                tracks.insert(pos, '|');
            }
            | 'v' => {
                carts.insert(pos, Cart { turns: 0, dir: Dir::D });
                tracks.insert(pos, '|');
            }
            | '-' | '|' | '/' | '\\' | '+' => {
                tracks.insert(pos, track);
            }
            | _ => (),
            };
        }
    }

    let mut dead: Set<Pos> = Set::default();

    loop {
        if carts.len() == 1 {
            println!("{:?}", carts);
            return;
        }

        let sorted = carts.iter()
            .map(|(pos, cart)| (pos.clone(), cart.clone()))
            .collect::<Vec<_>>()
            .tap(|v| v.sort());

        let mut moved: Map<Pos, Cart> = Map::default();

        for (pos, cart) in sorted {

            // Collided with cart that moved earlier this tick
            if dead.contains(&pos) { continue }

            // Register this cart as having moved
            carts.remove(&pos);

            // Step cart
            let (next_pos, next_cart) = cart.step(pos, tracks[&pos]);

            // Save collision as dead zone
            if carts.contains_key(&next_pos)
            || moved.insert(next_pos, next_cart).is_some() {
                dead.insert(next_pos);
                moved.remove(&next_pos);
            }
        }

        carts = moved;
        dead.clear();
    }
}
