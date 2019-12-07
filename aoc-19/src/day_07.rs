use std::cmp;

use aoc::*;

pub struct Placholder(intcode::Program);

impl Fro for Placholder {
    fn fro(input: &str) -> Self {
        Placholder(intcode::Program::fro(input))
    }
}

impl Solution for Placholder {
    fn one(self) -> i32 {
        let mut max = 0;
        for a in 0..5 {
            for b in 0..5 {
                for c in 0..5 {
                    for d in 0..5 {
                        for e in 0..5 {
                            if a == b || b == c || c == d || d == e || a == c || a == d || a == e || b == d || b == e ||  c == e { continue }

                            let mut ap = self.0.clone();
                            let mut bp = self.0.clone();
                            let mut cp = self.0.clone();
                            let mut dp = self.0.clone();
                            let mut ep = self.0.clone();

                            ap.run_input(a);
                            ap.run_input(0);
                            let o = ap.run_output().unwrap();

                            bp.run_input(b);
                            bp.run_input(o);
                            let o = bp.run_output().unwrap();

                            cp.run_input(c);
                            cp.run_input(o);
                            let o = cp.run_output().unwrap();

                            dp.run_input(d);
                            dp.run_input(o);
                            let o = dp.run_output().unwrap();

                            ep.run_input(e);
                            ep.run_input(o);
                            let o = ep.run_output().unwrap();

                            max = cmp::max(o, max);
                        }
                    }
                }
            }
        }
        max
    }

    fn two(self) -> i32 {
        let mut max = 0;
        for a in 5..10 {
            for b in 5..10 {
                for c in 5..10 {
                    for d in 5..10 {
                        for e in 5..10 {
                            if a == b || b == c || c == d || d == e || a == c || a == d || a == e || b == d || b == e ||  c == e { continue }

                            let mut ap = self.0.clone();
                            let mut bp = self.0.clone();
                            let mut cp = self.0.clone();
                            let mut dp = self.0.clone();
                            let mut ep = self.0.clone();

                            ap.run_input(a);
                            bp.run_input(b);
                            cp.run_input(c);
                            dp.run_input(d);
                            ep.run_input(e);

                            ap.run_input(0);
                            let mut o = ap.run_output().unwrap();

                            loop {

                            if bp.run_input(o) { break }
                            match bp.run_output() {
                                Some(x) => o = x,
                                None => break,
                            };

                            if cp.run_input(o) { break }
                            match cp.run_output() {
                                Some(x) => o = x,
                                None => break,
                            };

                            if dp.run_input(o) { break }
                            match dp.run_output() {
                                Some(x) => o = x,
                                None => break,
                            };

                            if ep.run_input(o) { break }
                            match ep.run_output() {
                                Some(x) => o = x,
                                None => break,
                            };

                            if ap.run_input(o) { break }
                            match ap.run_output() {
                                Some(x) => o = x,
                                None => break,
                            };

                            };
                            max = cmp::max(o, max);
                        }
                    }
                }
            }
        }
        max
    }
}
