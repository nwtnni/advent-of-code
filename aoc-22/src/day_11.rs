use aoc::*;

#[derive(Clone, Debug)]
pub struct MonkeyInTheMiddle(Vec<Monkey>);

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    test: Test,
}

#[derive(Copy, Clone, Debug)]
enum Operation {
    Add(i64),
    Mul(i64),
    Square,
}

impl Operation {
    fn apply(&self, item: i64, divisor: i64) -> i64 {
        match self {
            Operation::Add(x) => (item % divisor + x % divisor) % divisor,
            Operation::Mul(x) => (item % divisor * x % divisor) % divisor,
            Operation::Square => (item % divisor).pow(2) % divisor,
        }
    }
}

#[derive(Clone, Debug)]
struct Test {
    divisible: i64,
    r#true: usize,
    r#false: usize,
}

impl Test {
    fn apply(&self, item: i64) -> usize {
        if item % self.divisible == 0 {
            self.r#true
        } else {
            self.r#false
        }
    }
}

impl Fro for MonkeyInTheMiddle {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split("\n\n")
            .map(|line| {
                let mut iter = line.split('\n');
                let _monkey = iter
                    .give()
                    .trim()
                    .trim_start_matches("Monkey ")
                    .trim_end_matches(':')
                    .tap(i64::fro);
                let items = iter
                    .give()
                    .trim()
                    .trim_start_matches("Starting items: ")
                    .split(", ")
                    .map(i64::fro)
                    .collect::<Vec<_>>();

                let operation = iter
                    .give()
                    .trim()
                    .trim_start_matches("Operation: new = old ");

                let operation = if let Some(suffix) = operation.strip_prefix("* ") {
                    if suffix == "old" {
                        Operation::Square
                    } else {
                        Operation::Mul(i64::fro(suffix))
                    }
                } else if let Some(suffix) = operation.strip_prefix("+ ") {
                    if suffix == "old" {
                        Operation::Square
                    } else {
                        Operation::Add(i64::fro(suffix))
                    }
                } else {
                    unreachable!()
                };

                let divisible = iter
                    .give()
                    .trim()
                    .trim_start_matches("Test: divisible by ")
                    .tap(i64::fro);
                let r#true = iter
                    .give()
                    .trim()
                    .trim_start_matches("If true: throw to monkey ")
                    .tap(usize::fro);
                let r#false = iter
                    .give()
                    .trim()
                    .trim_start_matches("If false: throw to monkey ")
                    .tap(usize::fro);

                Monkey {
                    items,
                    operation,
                    test: Test {
                        divisible,
                        r#true,
                        r#false,
                    },
                }
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for MonkeyInTheMiddle {
    fn one(mut self) -> i64 {
        let mut inspects = vec![0; self.0.len()];
        let mut buffer = Vec::new();

        for _ in 0..20 {
            #[allow(clippy::needless_range_loop)]
            for monkey in 0..self.0.len() {
                buffer.append(&mut self.0[monkey].items);

                for item in buffer.drain(..) {
                    inspects[monkey] += 1;
                    let worry = self.0[monkey].operation.apply(item, i64::MAX) / 3;
                    let next = self.0[monkey].test.apply(worry);
                    self.0[next].items.push(worry);
                }
            }
        }

        inspects.sort();
        inspects.reverse();
        inspects[0] * inspects[1]
    }

    fn two(mut self) -> i64 {
        let mut inspects = vec![0; self.0.len()];
        let mut buffer = Vec::new();
        let lcm = self
            .0
            .iter()
            .map(|monkey| monkey.test.divisible)
            .product::<i64>();

        for _ in 0..10000 {
            #[allow(clippy::needless_range_loop)]
            for monkey in 0..self.0.len() {
                buffer.append(&mut self.0[monkey].items);

                for item in buffer.drain(..) {
                    inspects[monkey] += 1;
                    let worry = self.0[monkey].operation.apply(item, lcm);
                    let next = self.0[monkey].test.apply(worry);
                    self.0[next].items.push(worry);
                }
            }
        }

        inspects.sort();
        inspects.reverse();
        inspects[0] * inspects[1]
    }
}
