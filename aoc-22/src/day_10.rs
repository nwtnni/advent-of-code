use aoc::*;

#[derive(Clone, Debug)]
pub struct CathodeRayTube(Vec<Instruction>);

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Noop,
    Addx(i64),
}

impl Fro for CathodeRayTube {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| match line {
                "noop" => Instruction::Noop,
                _ => Instruction::Addx(i64::fro(line.trim_start_matches("addx "))),
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for CathodeRayTube {
    fn one(self) -> i64 {
        let mut sum = 0;

        self.run(|cycle, x| {
            if cycle <= 220 && (cycle == 20 || cycle > 20 && (cycle - 20) % 40 == 0) {
                sum += x * cycle;
            }
        });

        sum
    }

    fn two(self) -> i64 {
        let mut screen = vec![false; 40 * 6];

        self.run(|cycle, x| {
            let sx = (cycle - 1) % 40;
            if (x - sx).abs() <= 1 {
                screen[cycle as usize - 1] = true;
            }
        });

        for i in 0..6 {
            for j in 0..40 {
                print!("{}", if screen[i * 40 + j] { '█' } else { ' ' })
            }
            println!();
        }

        panic!();
    }
}

impl CathodeRayTube {
    fn run<F: FnMut(i64, i64)>(&self, mut apply: F) {
        let mut pause = false;
        let mut cycle = 1;
        let mut x = 1;

        let mut iter = self.0.iter().peekable();

        while let Some(ins) = iter.peek() {
            apply(cycle, x);

            match ins {
                Instruction::Noop => {
                    iter.next();
                    cycle += 1;
                }
                Instruction::Addx(_) if !pause => {
                    pause = true;
                    cycle += 1;
                }
                Instruction::Addx(dx) => {
                    x += dx;
                    pause = false;
                    iter.next();
                    cycle += 1;
                }
            }
        }
    }
}
