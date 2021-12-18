use std::cmp;
use std::fmt;
use std::iter;
use std::str;

use std::cell::Cell;
use std::cell::RefCell;
use std::rc;
use std::rc::Rc;
use std::rc::Weak;

use aoc::*;

#[derive(Clone, Debug)]
pub struct Snailfish(Vec<Rc<RefCell<Tree>>>);

#[derive(Clone)]
enum Tree {
    Leaf(Cell<i64>),
    Node {
        children: [Rc<RefCell<Tree>>; 2],
        parent: Option<(rc::Weak<RefCell<Tree>>, Child)>,
    },
}

impl fmt::Debug for Tree {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tree::Leaf(value) => write!(fmt, "{}", value.get()),
            Tree::Node { children, .. } => {
                write!(
                    fmt,
                    "[{:?},{:?}]",
                    children[0].borrow(),
                    children[1].borrow()
                )
            }
        }
    }
}

#[repr(usize)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Child {
    Left = 0,
    Right = 1,
}

impl Fro for Snailfish {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| parse(&mut line.trim().chars().peekable(), None))
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

fn parse(
    iter: &mut iter::Peekable<str::Chars>,
    parent: Option<(Weak<RefCell<Tree>>, Child)>,
) -> Rc<RefCell<Tree>> {
    match iter.next() {
        Some('[') => {
            let node = Rc::new(RefCell::new(Tree::Node {
                children: [
                    Rc::new(RefCell::new(Tree::Leaf(Cell::new(i64::MIN)))),
                    Rc::new(RefCell::new(Tree::Leaf(Cell::new(i64::MIN)))),
                ],
                parent,
            }));

            let left = parse(iter, Some((Rc::downgrade(&node), Child::Left)));
            match &mut *node.borrow_mut() {
                Tree::Node { children, .. } => children[0] = left,
                _ => unreachable!(),
            }

            assert_eq!(iter.next(), Some(','));

            let right = parse(iter, Some((Rc::downgrade(&node), Child::Right)));
            match &mut *node.borrow_mut() {
                Tree::Node { children, .. } => children[1] = right,
                _ => unreachable!(),
            }

            assert_eq!(iter.next(), Some(']'));

            node
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

            Rc::new(RefCell::new(Tree::Leaf(Cell::new(i64::fro(&literal)))))
        }
        _ => unreachable!(),
    }
}

enum Explode {
    Replace,
    Replaced,
}

enum Split {
    Replace(i64, i64),
    Replaced,
}

impl Tree {
    fn explode(&self, depth: usize) -> Option<Explode> {
        match self {
            Tree::Leaf(_) => None,
            Tree::Node { children, .. } if depth < 4 => {
                let left = children[0].borrow().explode(depth + 1);
                match left {
                    None => (),
                    Some(Explode::Replaced) => return Some(Explode::Replaced),
                    Some(Explode::Replace) => {
                        *children[0].borrow_mut() = Tree::Leaf(Cell::new(0));
                        return Some(Explode::Replaced);
                    }
                }

                let right = children[1].borrow().explode(depth + 1);
                match right {
                    None => (),
                    Some(Explode::Replaced) => return Some(Explode::Replaced),
                    Some(Explode::Replace) => {
                        *children[1].borrow_mut() = Tree::Leaf(Cell::new(0));
                        return Some(Explode::Replaced);
                    }
                }

                None
            }
            Tree::Node { children, parent } if depth == 4 => {
                let (left, right) = match (
                    children.get(0).map(|child| child.borrow()).as_deref(),
                    children.get(1).map(|child| child.borrow()).as_deref(),
                ) {
                    (Some(Tree::Leaf(left)), Some(Tree::Leaf(right))) => (left.get(), right.get()),
                    _ => return None,
                };

                let (parent, child) = parent.clone().unwrap();
                let parent = parent.upgrade().unwrap();

                match child {
                    Child::Left => {
                        let parent = parent.borrow();
                        let mut above = match &*parent {
                            Tree::Leaf(_) => unreachable!(),
                            Tree::Node { children, parent } => {
                                children[1].borrow().add(right, 0);
                                parent
                                    .as_ref()
                                    .map(|(parent, child)| (parent.upgrade().unwrap(), *child))
                                    .tap(Option::Some)
                            }
                        };

                        while let Some((parent, child)) = above.take().flatten() {
                            match child {
                                Child::Left => {
                                    let parent = parent.borrow();
                                    above = match &*parent {
                                        Tree::Leaf(_) => unreachable!(),
                                        Tree::Node { parent, .. } => parent
                                            .as_ref()
                                            .map(|(parent, child)| {
                                                (parent.upgrade().unwrap(), *child)
                                            })
                                            .tap(Option::Some),
                                    };
                                }
                                Child::Right => match &*parent.borrow() {
                                    Tree::Leaf(_) => unreachable!(),
                                    Tree::Node { children, .. } => {
                                        children[0].borrow().add(left, 1);
                                    }
                                },
                            }
                        }
                    }
                    Child::Right => {
                        let parent = parent.borrow();
                        let mut above = match &*parent {
                            Tree::Leaf(_) => unreachable!(),
                            Tree::Node { children, parent } => {
                                children[0].borrow().add(left, 1);
                                parent
                                    .as_ref()
                                    .map(|(parent, child)| (parent.upgrade().unwrap(), *child))
                                    .tap(Option::Some)
                            }
                        };

                        while let Some((parent, child)) = above.take().flatten() {
                            match child {
                                Child::Left => match &*parent.borrow() {
                                    Tree::Leaf(_) => unreachable!(),
                                    Tree::Node { children, .. } => {
                                        children[1].borrow().add(right, 0);
                                    }
                                },
                                Child::Right => {
                                    let parent = parent.borrow();
                                    above = match &*parent {
                                        Tree::Leaf(_) => unreachable!(),
                                        Tree::Node { parent, .. } => parent
                                            .as_ref()
                                            .map(|(parent, child)| {
                                                (parent.upgrade().unwrap(), *child)
                                            })
                                            .tap(Option::Some),
                                    };
                                }
                            }
                        }
                    }
                }

                Some(Explode::Replace)
            }
            Tree::Node { .. } => unreachable!(),
        }
    }

    fn add(&self, add: i64, direction: usize) {
        match self {
            Tree::Leaf(value) => {
                let old = value.get();
                let new = old + add;
                value.set(new);
            }
            Tree::Node { children, .. } => {
                children[direction].borrow().add(add, direction);
            }
        }
    }

    fn magnitude(&self) -> i64 {
        match self {
            Tree::Leaf(value) => value.get(),
            Tree::Node { children, .. } => {
                3 * children[0].borrow().magnitude() + 2 * children[1].borrow().magnitude()
            }
        }
    }
}

fn split(tree: &Rc<RefCell<Tree>>) -> Option<Split> {
    match &*tree.borrow() {
        Tree::Leaf(value) if value.get() < 10 => None,
        Tree::Leaf(value) => {
            let half = value.get() as f64 / 2.0;
            let left = half.floor() as i64;
            let right = half.ceil() as i64;
            Some(Split::Replace(left, right))
        }
        Tree::Node { children, .. } => {
            for (i, child) in children.iter().enumerate() {
                match split(child) {
                    None => (),
                    Some(Split::Replaced) => return Some(Split::Replaced),
                    Some(Split::Replace(left, right)) => {
                        *child.borrow_mut() = Tree::Node {
                            children: [
                                Rc::new(RefCell::new(Tree::Leaf(Cell::new(left)))),
                                Rc::new(RefCell::new(Tree::Leaf(Cell::new(right)))),
                            ],
                            parent: Some((
                                Rc::downgrade(tree),
                                if i == 0 { Child::Left } else { Child::Right },
                            )),
                        };
                        return Some(Split::Replaced);
                    }
                }
            }
            None
        }
    }
}

fn deep_clone(tree: &Tree, parent: Option<(Weak<RefCell<Tree>>, Child)>) -> Rc<RefCell<Tree>> {
    match tree {
        Tree::Leaf(value) => Rc::new(RefCell::new(Tree::Leaf(Cell::new(value.get())))),
        Tree::Node { children, .. } => {
            let tree = Rc::new(RefCell::new(Tree::Leaf(Cell::new(0))));

            let left = deep_clone(
                &*children[0].borrow(),
                Some((Rc::downgrade(&tree), Child::Left)),
            );
            let right = deep_clone(
                &*children[1].borrow(),
                Some((Rc::downgrade(&tree), Child::Right)),
            );

            *tree.borrow_mut() = Tree::Node {
                children: [left, right],
                parent,
            };

            tree
        }
    }
}

fn reduce(tree: &Rc<RefCell<Tree>>) {
    let mut dirty = true;
    while dirty {
        dirty = false;

        match tree.borrow().explode(0) {
            None => (),
            Some(Explode::Replace) => unreachable!(),
            Some(Explode::Replaced) => {
                dirty = true;
                continue;
            }
        }

        match split(tree) {
            None => (),
            Some(Split::Replace(_, _)) => unreachable!(),
            Some(Split::Replaced) => {
                dirty = true;
                continue;
            }
        }
    }
}

impl Solution for Snailfish {
    fn one(mut self) -> i64 {
        self.0.reverse();

        let mut sum = Some(self.0.pop().unwrap());

        reduce(sum.as_ref().unwrap());

        while let Some(next) = self.0.pop() {
            let left = sum.take().unwrap();
            let right = next;

            let tree = Rc::new(RefCell::new(Tree::Leaf(Cell::new(0))));

            match &mut *left.borrow_mut() {
                Tree::Leaf(_) => (),
                Tree::Node { parent, .. } => *parent = Some((Rc::downgrade(&tree), Child::Left)),
            }

            match &mut *right.borrow_mut() {
                Tree::Leaf(_) => (),
                Tree::Node { parent, .. } => *parent = Some((Rc::downgrade(&tree), Child::Right)),
            }

            *tree.borrow_mut() = Tree::Node {
                children: [left, right],
                parent: None,
            };

            sum = Some(tree);

            reduce(sum.as_ref().unwrap());
        }

        sum.unwrap().borrow().magnitude()
    }

    fn two(self) -> i64 {
        let mut max = i64::MIN;
        for i in 0..self.0.len() {
            for j in 0..self.0.len() {
                if i == j {
                    continue;
                }

                let sum = Rc::new(RefCell::new(Tree::Leaf(Cell::new(0))));
                let left = deep_clone(
                    &*self.0[i].borrow(),
                    Some((Rc::downgrade(&sum), Child::Left)),
                );
                let right = deep_clone(
                    &*self.0[j].borrow(),
                    Some((Rc::downgrade(&sum), Child::Right)),
                );
                *sum.borrow_mut() = Tree::Node {
                    children: [left, right],
                    parent: None,
                };

                dbg!(&sum);
                reduce(&sum);
                dbg!(&sum, sum.borrow().magnitude());

                max = cmp::max(max, sum.borrow().magnitude());
            }
        }
        max
    }
}
