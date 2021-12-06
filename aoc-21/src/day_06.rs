use aoc::*;

#[derive(Clone, Debug)]
pub struct Lanternfish(Vec<i64>);

impl Fro for Lanternfish {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split(',')
            .map(i64::fro)
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for Lanternfish {
    fn one(mut self) -> i64 {
        for _ in 0..80 {
            let mut new = 0;
            for age in &mut self.0 {
                if *age == 0 {
                    *age = 6;
                    new += 1;
                } else {
                    *age -= 1;
                }
            }
            for _ in 0..new {
                self.0.push(8);
            }
        }
        self.0.len() as i64
    }

    fn two(self) -> i64 {
        let mut fish = [0; 9];

        for age in self.0 {
            fish[age as usize] += 1;
        }

        for _ in 0..256 {
            fish.rotate_left(1);
            fish[6] += fish[8];
        }

        fish.iter().sum()
    }
}
