use aoc::*;

#[derive(Clone, Debug)]
pub struct Trebuchet(Vec<String>);

impl Fro for Trebuchet {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(String::from)
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for Trebuchet {
    fn one(self) -> i64 {
        self.0
            .iter()
            .map(|line| {
                let a = line.chars().find_map(|a| a.to_digit(10)).unwrap();
                let b = line.chars().rev().find_map(|a| a.to_digit(10)).unwrap();
                a * 10 + b
            })
            .sum::<u32>() as i64
    }

    fn two(self) -> i64 {
        self.0
            .iter()
            .map(|line| {
                let mut a = None;
                let mut b = None;

                for i in 0..line.len() {
                    for j in i + 1..line.len() + 1 {
                        let digit = match &line[i..j] {
                            "0" => 0,
                            "1" | "one" => 1,
                            "2" | "two" => 2,
                            "3" | "three" => 3,
                            "4" | "four" => 4,
                            "5" | "five" => 5,
                            "6" | "six" => 6,
                            "7" | "seven" => 7,
                            "8" | "eight" => 8,
                            "9" | "nine" => 9,
                            _ => continue,
                        };

                        a.get_or_insert(digit);
                        b = Some(digit);
                    }
                }

                a.unwrap() * 10 + b.unwrap()
            })
            .sum()
    }
}
