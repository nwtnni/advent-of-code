use std::iter;
use std::mem;

use aoc::*;

#[derive(Clone, Debug)]
pub struct FlawedFrequencyTransmission {
    prev: Vec<Digit>,
    next: Vec<Digit>,
}

impl Fro for FlawedFrequencyTransmission {
    fn fro(input: &str) -> Self {
        let prev = input
            .trim()
            .chars()
            .map(Digit::from_char_unchecked)
            .collect::<Vec<_>>();

        FlawedFrequencyTransmission {
            next: vec![Digit::D0; prev.len()],
            prev,
        }
    }
}

fn pattern(pos: usize) -> impl Iterator<Item = i64> {
    let count = pos + 1;
    iter::repeat(0)
        .take(count)
        .chain(iter::repeat(1).take(count))
        .chain(iter::repeat(0).take(count))
        .chain(iter::repeat(-1).take(count))
        .cycle()
        .skip(1)
}

impl Solution for FlawedFrequencyTransmission {
    fn one(mut self) -> i64 {
        for _ in 0..100 {
            for pos in 0..self.next.len() {
                self.next[pos] = Digit::from_int_unchecked(
                    self.prev
                        .iter()
                        .zip(pattern(pos))
                        .map(|(l, r)| *l as i64 * r)
                        .sum::<i64>()
                        .abs()
                        % 10,
                );
            }
            mem::swap(&mut self.prev, &mut self.next);
        }
        self.prev[0..8]
            .iter()
            .copied()
            .map(char::from)
            .collect::<String>()
            .to::<i64>()
    }

    fn two(mut self) -> i64 {
        // Upper half of the full array
        let mut half = Vec::with_capacity(self.prev.len() * 5000);
        for _ in 0..5000 {
            for d in self.prev.iter().copied() {
                half.push(d);
            }
        }

        let offset = half[0..7]
            .iter()
            .copied()
            .map(char::from)
            .collect::<String>()
            .to::<usize>()
            - half.len();

        self.prev = half;
        self.next = vec![Digit::D0; self.prev.len()];

        for _ in 0..100 {
            let mut sum = 0;
            for pos in (1..self.next.len()).rev() {
                sum += self.prev[pos] as i64;
                self.next[pos] = Digit::from_int_unchecked(sum.abs() % 10);
            }
            sum -= self.prev[self.prev.len() - 1] as i64;
            self.next[0] = Digit::from_int_unchecked(sum.abs() % 10);
            mem::swap(&mut self.prev, &mut self.next);
        }

        self.prev[offset..][0..8]
            .iter()
            .copied()
            .map(char::from)
            .collect::<String>()
            .to::<i64>()
    }
}
