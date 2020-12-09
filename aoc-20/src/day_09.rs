use aoc::*;

#[derive(Clone)]
pub struct EncodingError(Vec<i64>);

impl Fro for EncodingError {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(i64::fro)
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for EncodingError {
    fn one(self) -> i64 {
        'outer: for i in 25..self.0.len() {
            let sum = self.0[i];
            for j in i - 25..i {
                for k in j + 1..i {
                    if self.0[j] + self.0[k] == sum {
                        continue 'outer;
                    }
                }
            }
            return sum;
        }
        unreachable!()
    }

    fn two(self) -> i64 {
        let sum = self.clone().one();
        for i in 0..self.0.len() {
            for j in i + 2..self.0.len() {
                if self.0[i..j].iter().sum::<i64>() == sum {
                    let min = self.0[i..j].iter().min().unwrap();
                    let max = self.0[i..j].iter().max().unwrap();
                    return min + max;
                }
            }
        }
        unreachable!()
    }
}
