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
                let first = line.chars().find(|a| a.is_numeric()).unwrap();
                let last = line.chars().rev().find(|a| a.is_numeric()).unwrap();
                format!("{}{}", first, last).parse::<i64>().unwrap()
            })
            .sum()
    }

    fn two(self) -> i64 {
        self.0
            .iter()
            .map(|line| {
                let find1 = |line: &str| {
                    for i in 0..line.len() {
                        for j in i + 1..line.len() + 1 {
                            return match &line[i..j] {
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
                        }
                    }
                    unreachable!()
                };

                let find2 = |line: &str| {
                    let mut last = None;
                    for i in 0..line.len() {
                        for j in i + 1..line.len() + 1 {
                            last = Some(match &line[i..j] {
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
                            });
                        }
                    }
                    last.unwrap()
                };

                find1(line) * 10 + find2(line)
            })
            .sum()
    }
}
