use std::rc::Rc;
use std::collections::BTreeMap as Map;

const INPUT: &'static str = include_str!("input.txt");

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    pub fn around(&self) -> impl Iterator<Item = Pos> {
        let x = self.x;
        let y = self.y;
        (-1..=1).flat_map(move |dy| {
            (-1..=1).map(move |dx| {
                Pos { x: x + dx, y: y + dy }
            })
        }).filter(move |p| *p != Pos { x, y })
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Acre {
    Open,
    Tree,
    Yard,
}

impl Acre {
    pub fn parse(c: char) -> Self {
        match c {
        | '.' => Acre::Open,
        | '|' => Acre::Tree,
        | '#' => Acre::Yard,
        | _   => unreachable!(),
        }
    }
}

fn main() {

    let mut state: Map<Pos, Acre> = INPUT.trim()
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| {
                    (Pos { x: x as isize, y: y as isize }, Acre::parse(c))
                })
        })
        .collect();

    let mut next_state: Map<Pos, Acre> = Map::default();
    let mut states: Vec<Rc<Map<Pos, Acre>>> = Vec::default();
    let mut seen: Map<Rc<Map<Pos, Acre>>, usize> = Map::default();
    let mut t = 0;

    let rc = Rc::new(state.clone());
    states.push(rc.clone());
    seen.insert(rc, 0);

    let (u, t) = loop {
        for (pos, acre) in &state {
            let next = match acre {
            | Acre::Open => {
                let trees = pos.around()
                    .filter_map(|p| state.get(&p))
                    .filter(|a| **a == Acre::Tree)
                    .count();

                if trees >= 3 { Acre::Tree } else { Acre::Open }
            }
            | Acre::Tree => {
                let yards = pos.around()
                    .filter_map(|p| state.get(&p))
                    .filter(|a| **a == Acre::Yard)
                    .count();

                if yards >= 3 { Acre::Yard } else { Acre::Tree }
            }
            | Acre::Yard => {
                let mut yard = false;
                let mut tree = false;
                for a in pos.around().filter_map(|p| state.get(&p)) {
                    match a {
                    | Acre::Yard => yard = true,
                    | Acre::Tree => tree = true,
                    | _ => (),
                    }
                }

                if yard && tree { Acre::Yard } else { Acre::Open }
            }
            };
            next_state.insert(*pos, next);
        }

        std::mem::swap(&mut state, &mut next_state);

        if let Some(u) = seen.get(&state) {
            break (u, t);
        }

        let rc = Rc::new(state.clone());
        seen.insert(rc.clone(), t);
        states.push(rc);
        next_state.clear();
        t += 1;
    };

    let period = t - u;
    let index = ((1000000000 - u) % period) + u;

    let trees = states[index].values()
        .filter(|a| **a == Acre::Tree)
        .count();

    let yards = states[index].values()
        .filter(|a| **a == Acre::Yard)
        .count();

    println!("{}", trees * yards);
}
