use aoc::*;

#[derive(Clone, Debug)]
pub struct TractorBeam(intcode::Program);

impl Fro for TractorBeam {
    fn fro(input: &str) -> Self {
        TractorBeam(intcode::Program::fro(input))
    }
}

impl TractorBeam {
    fn get(&mut self, x: i64, y: i64) -> bool {
        self.0.reset();
        self.0.input(x);
        self.0.input(y);
        self.0.output().unwrap() == 1
    }

    #[allow(unused)]
    fn plot(&mut self) {
        for y in 0..50 {
            for x in 0..50 {
                if self.get(x, y) {
                    print!("█");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

impl Solution for TractorBeam {
    fn one(mut self) -> i64 {
        let mut sum = 0;
        for y in 0..50 {
            for x in 0..50 {
                sum += self.get(x, y) as i64;
            }
        }
        sum
    }

    fn two(mut self) -> i64 {
        let mut x = 3;
        let mut y = 4;

        loop {
            if y >= 99 && self.get(x + 99, y - 99) {
                return x * 10000 + y - 99;
            }
            x += 1;
            while self.get(x, y + 1) {
                y += 1;
                if y >= 99 && self.get(x + 99, y - 99) {
                    return x * 10000 + y - 99;
                }
            }
        }
    }
}
