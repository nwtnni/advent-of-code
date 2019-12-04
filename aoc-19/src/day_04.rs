use std::collections::HashSet;
use std::str;

#[derive(Clone, Debug)]
pub struct Placeholder(i32, i32);

impl str::FromStr for Placeholder {
    type Err = aoc::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut iter = input.trim().split('-');
        let l = iter
            .next()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let r = iter
            .next()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        Ok(Placeholder(l, r))
    }
}

impl aoc::Solution for Placeholder {
    fn one(self) -> i32 {
        let mut count = 0;
        'outer: for i in self.0..self.1 {
            let s = i.to_string();
            let mut last = 0;
            let mut double = false;
            for j in 0..s.len() {
                let x = s[j..j+1].parse::<i32>().unwrap();
                if x < last { continue 'outer; }
                double |= last == x;
                last = x;
            }
            if double {
            count += 1;
            }
        }
        count
    }

    fn two(self) -> i32 {
        let mut count = 0;
        'outer: for i in self.0..self.1 {
            let s = i.to_string();
            let mut last = None;
            let mut last_2 = None;
            let mut double = HashSet::new();
            let mut triple = HashSet::new();
            for j in 0..s.len() {
                let x = s[j..j+1].parse::<i32>().unwrap();
                if x < last.unwrap_or(-1) { continue 'outer; }
                if last == Some(x) {
                    double.insert(x);
                }
                if Some(x) == last && Some(x) == last_2 {
                    triple.insert(x);
                }
                last_2 = last;
                last = Some(x);
            }
            if double.difference(&triple).count() > 0 {
                count += 1;
            }
        }
        count
    }
}
