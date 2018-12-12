use std::collections::VecDeque;

use regex::Regex;

const INPUT: &'static str = include_str!("input.txt");

struct Circle {
    p: usize,
    m: usize,
    marbles: VecDeque<usize>,
    players: Vec<usize>,
}

impl Circle {
    fn new(players: usize, m: usize) -> Self {
        let mut marbles = VecDeque::with_capacity(m);
        marbles.push_back(0);
        Circle {
            p: 0,
            m,
            marbles,
            players: vec![0; players],
        }
    }

    fn rotate_cw(&mut self, steps: usize) {
        for _ in 0..steps {
            let m = self.marbles.pop_front().unwrap();
            self.marbles.push_back(m);
        }
    }

    fn rotate_ccw(&mut self, steps: usize) {
        for _ in 0..steps {
            let m = self.marbles.pop_back().unwrap();
            self.marbles.push_front(m);
        }
    }

    fn run(mut self) -> Vec<usize> {
        for marble in 1..=self.m {
            if marble % 23 == 0 {
                self.rotate_ccw(7);
                self.players[self.p] += self.marbles.pop_front().unwrap();
                self.players[self.p] += marble;
            } else {
                self.rotate_cw(2);
                self.marbles.push_front(marble);
            }
            self.p += 1;
            self.p %= self.players.len();
        }
        self.players
    }
}

fn main() {
    let re = Regex::new(r"(\d+) players; last marble is worth (\d+) points").unwrap();
    let captured = re.captures(INPUT).unwrap();
    let players: usize = captured[1].parse().unwrap();
    let last: usize = captured[2].parse().unwrap();

    let scores = Circle::new(players, last * 100).run();
    let max = scores.iter()
        .max()
        .unwrap();

    println!("{}", max);
}
