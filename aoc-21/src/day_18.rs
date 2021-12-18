use std::cmp;
use std::fmt;
use std::iter;
use std::str;

use aoc::*;

#[derive(Clone, Debug)]
pub struct Snailfish(Vec<Tree>);

#[derive(Clone)]
enum Tree {
    Leaf(i64),
    Node(Box<Tree>, Box<Tree>),
}

impl fmt::Debug for Tree {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tree::Leaf(value) => write!(fmt, "{}", value),
            Tree::Node(left, right) => {
                write!(fmt, "[{:?},{:?}]", left, right,)
            }
        }
    }
}

#[repr(usize)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    Left = 0,
    Right = 1,
}

impl Fro for Snailfish {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| parse(&mut line.trim().chars().peekable()))
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

fn parse(iter: &mut iter::Peekable<str::Chars>) -> Tree {
    match iter.next() {
        Some('[') => {
            let left = parse(iter);
            assert_eq!(iter.next(), Some(','));
            let right = parse(iter);
            assert_eq!(iter.next(), Some(']'));
            Tree::Node(Box::new(left), Box::new(right))
        }
        Some(next @ '0'..='9') => {
            let mut literal = next.to_string();

            while let Some(peek) = iter.peek().copied() {
                if peek.is_numeric() {
                    iter.next();
                    literal.push(peek);
                } else {
                    break;
                }
            }

            Tree::Leaf(i64::fro(&literal))
        }
        _ => unreachable!(),
    }
}

impl Tree {
    fn explode(&mut self, depth: usize) -> Option<(i64, i64)> {
        match self {
            Tree::Leaf(_) => None,
            Tree::Node(left, right) if depth < 4 => {
                match left.explode(depth + 1) {
                    None => (),
                    Some((l, 0)) => return Some((l, 0)),
                    Some((l, r)) => {
                        right.add(r, Direction::Left);
                        return Some((l, 0));
                    }
                }

                match right.explode(depth + 1) {
                    None => (),
                    Some((0, r)) => return Some((0, r)),
                    Some((l, r)) => {
                        left.add(l, Direction::Right);
                        return Some((0, r));
                    }
                }

                None
            }
            Tree::Node(left, right) if depth == 4 => match (&**left, &**right) {
                (Tree::Leaf(l), Tree::Leaf(r)) => {
                    let (l, r) = (*l, *r);
                    *self = Tree::Leaf(0);
                    Some((l, r))
                }
                (_, _) => None,
            },
            Tree::Node(_, _) => unreachable!(),
        }
    }

    fn add(&mut self, add: i64, direction: Direction) {
        match self {
            Tree::Leaf(value) => *value += add,
            Tree::Node(left, right) => match direction {
                Direction::Left => left.add(add, Direction::Left),
                Direction::Right => right.add(add, Direction::Right),
            },
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Tree::Node(left, right) => [left, right].into_iter().any(|child| child.split()),
            Tree::Leaf(value) if *value < 10 => false,
            Tree::Leaf(value) => {
                let m = *value as f64 / 2.0;
                let l = m.floor() as i64;
                let r = m.ceil() as i64;
                *self = Tree::Node(Box::new(Tree::Leaf(l)), Box::new(Tree::Leaf(r)));
                true
            }
        }
    }

    fn reduce(&mut self) {
        let mut dirty = true;
        while dirty {
            dirty = false;

            match self.explode(0) {
                None => (),
                Some(_) => {
                    dirty = true;
                    continue;
                }
            }

            if self.split() {
                dirty = true;
                continue;
            }
        }
    }

    fn magnitude(&self) -> i64 {
        match self {
            Tree::Leaf(value) => *value,
            Tree::Node(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

impl Solution for Snailfish {
    fn one(mut self) -> i64 {
        self.0.reverse();

        let mut sum = Some(self.0.pop().unwrap());
        sum.as_mut().unwrap().reduce();

        while let Some(next) = self.0.pop() {
            let left = sum.take().unwrap();
            let right = next;
            sum = Some(Tree::Node(Box::new(left), Box::new(right)));
            sum.as_mut().unwrap().reduce();
        }

        sum.unwrap().magnitude()
    }

    fn two(self) -> i64 {
        let mut max = i64::MIN;

        for i in 0..self.0.len() {
            for j in 0..self.0.len() {
                if i == j {
                    continue;
                }

                let mut sum = Tree::Node(Box::new(self.0[i].clone()), Box::new(self.0[j].clone()));
                sum.reduce();
                max = cmp::max(max, sum.magnitude());
            }
        }

        max
    }
}
