use aoc::*;

#[derive(Clone, Debug)]
pub struct CathodeRayTube(Vec<Ins>);

#[derive(Copy, Clone, Debug)]
enum Ins {
    Noop,
    Add(i64),
}

impl Fro for CathodeRayTube {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| match line {
                "noop" => Ins::Noop,
                _ => Ins::Add(i64::fro(line.trim_start_matches("addx "))),
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for CathodeRayTube {
    fn one(self) -> i64 {
        let mut cycle = 1;
        let mut x = 1;
        let mut pause = false;
        let mut sum = 0;

        let mut iter = self.0.into_iter().peekable();
        while let Some(ins) = iter.peek() {
            if cycle > 220 {
                break;
            }

            if cycle == 20 || cycle > 20 && (cycle - 20) % 40 == 0 {
                sum += x * cycle;
            }

            match ins {
                Ins::Noop => {
                    iter.next();
                    cycle += 1;
                }
                Ins::Add(dx) if pause => {
                    pause = false;
                    x += dx;
                    cycle += 1;
                    iter.next();
                }
                Ins::Add(_) => {
                    pause = true;
                    cycle += 1;
                }
            }
        }
        sum
    }

    fn two(self) -> i64 {
        let mut cycle = 1;
        let mut x = 1i64;
        let mut pause = false;

        let mut iter = self.0.into_iter().peekable();
        let mut screen = vec![false; 40 * 6];

        while let Some(ins) = iter.peek() {
            let sx = (cycle - 1) % 40;
            if (x - sx).abs() <= 1 {
                screen[cycle as usize - 1] = true;
            }

            match ins {
                Ins::Noop => {
                    iter.next();
                    cycle += 1;
                }
                Ins::Add(dx) if pause => {
                    pause = false;
                    x += *dx;
                    cycle += 1;
                    iter.next();
                }
                Ins::Add(_) => {
                    pause = true;
                    cycle += 1;
                }
            }
        }
        for i in 0..6 {
            for j in 0..40 {
                print!("{}", if screen[i * 40 + j] { '#' } else { '.' })
            }
            println!();
        }

        panic!();
    }
}
