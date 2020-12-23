use std::cell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::rc;

use aoc::*;

#[derive(Clone, Debug)]
pub struct CrabCups(VecDeque<i64>);

impl Fro for CrabCups {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .chars()
            .map(|char| i64::fro(&char.to_string()))
            .collect::<VecDeque<_>>()
            .tap(Self)
    }
}

impl Solution for CrabCups {
    fn one(mut self) -> i64 {

        let min = self
            .0
            .iter()
            .copied()
            .min()
            .unwrap();

        let max = self
            .0
            .iter()
            .copied()
            .max()
            .unwrap();

        for _ in 0..100 {

            let current = self.0.pop_front().unwrap();
            let three = [
                self.0.pop_front().unwrap(),
                self.0.pop_front().unwrap(),
                self.0.pop_front().unwrap(),
            ];

            let mut target = current - 1;

            let index = loop {
                if let Some(index) = self
                    .0
                    .iter()
                    .copied()
                    .position(|label| label == target)
                {
                    break index;
                } else {
                    target -= 1;
                    if target < min {
                        target = max;
                    }
                }
            };

            self.0.insert(index + 1, three[2]);
            self.0.insert(index + 1, three[1]);
            self.0.insert(index + 1, three[0]);
            self.0.push_back(current);
        }

        let count = self.0.len();

        self.0
            .iter()
            .cycle()
            .skip_while(|label| **label != 1)
            .skip(1)
            .take(count - 1)
            .map(|label| label.to_string())
            .collect::<String>()
            .to::<i64>()
    }

    fn two(mut self) -> i64 {

        let max = self
            .0
            .iter()
            .copied()
            .max()
            .unwrap();

        self.0.extend(max + 1..=1_000_000);

        let nodes = self
            .0
            .iter()
            .copied()
            .map(|label| (label, rc::Rc::new(Cup::new(label))))
            .collect::<HashMap<_, _>>();

        for i in 0..self.0.len() {
            let curr = i;
            let prev = (i + self.0.len() - 1).rem_euclid(self.0.len());
            let next = (i + self.0.len() + 1).rem_euclid(self.0.len());
            *nodes[&self.0[curr]].prev.borrow_mut() = Some(rc::Rc::clone(&nodes[&self.0[prev]]));
            *nodes[&self.0[curr]].next.borrow_mut() = Some(rc::Rc::clone(&nodes[&self.0[next]]));
        }

        let mut current_cup = rc::Rc::clone(&nodes[&self.0[0]]);
        let mut current_label = current_cup.label;

        for i in 0..10_000_000 {
            if i % 1000 == 0 {
                println!("{}", i);
            }

            let first_cup = rc::Rc::clone(current_cup.next.borrow().as_ref().unwrap());
            let first_label = first_cup.label;

            let second_cup = rc::Rc::clone(first_cup.next.borrow().as_ref().unwrap());
            let second_label = second_cup.label;

            let third_cup = rc::Rc::clone(second_cup.next.borrow().as_ref().unwrap());
            let third_label = third_cup.label;

            let after_third_cup = rc::Rc::clone(third_cup.next.borrow().as_ref().unwrap());

            let mut target_label = match (current_label - 1).rem_euclid(self.0.len() as i64) {
                0 => self.0.len() as i64,
                l => l,
            };
            while [first_label, second_label, third_label].contains(&target_label) {
                target_label = match (target_label - 1).rem_euclid(self.0.len() as i64) {
                    0 => self.0.len() as i64,
                    l => l,
                };
            }

            let target_cup = rc::Rc::clone(&nodes[&target_label]);
            let after_target_cup = rc::Rc::clone(target_cup.next.borrow().as_ref().unwrap());

            // Splice
            *current_cup.next.borrow_mut() = Some(rc::Rc::clone(&after_third_cup));
            *after_third_cup.prev.borrow_mut() = Some(rc::Rc::clone(&current_cup));

            *target_cup.next.borrow_mut() = Some(rc::Rc::clone(&first_cup));
            *first_cup.prev.borrow_mut() = Some(rc::Rc::clone(&target_cup));

            *third_cup.next.borrow_mut() = Some(rc::Rc::clone(&after_target_cup));
            *after_target_cup.prev.borrow_mut() = Some(rc::Rc::clone(&third_cup));

            let temp_cup = rc::Rc::clone(current_cup.next.borrow().as_ref().unwrap());
            current_cup = temp_cup;
            current_label = current_cup.label;
        }

        while current_label != 1 {
            let temp_cup = rc::Rc::clone(current_cup.next.borrow().as_ref().unwrap());
            current_cup = temp_cup;
            current_label = current_cup.label;
        }

        let first_cup = rc::Rc::clone(current_cup.next.borrow().as_ref().unwrap());
        let second_cup = rc::Rc::clone(first_cup.next.borrow().as_ref().unwrap());

        first_cup.label * second_cup.label
    }
}

#[derive(Clone, Debug)]
struct Cup {
    label: i64,
    next: cell::RefCell<Option<rc::Rc<Cup>>>,
    prev: cell::RefCell<Option<rc::Rc<Cup>>>,
}

impl Cup {
    pub fn new(label: i64) -> Self {
        Cup {
            label,
            next: Default::default(),
            prev: Default::default(),
        }
    }
}
