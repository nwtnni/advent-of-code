use std::iter;
use std::thread;
use std::time;

use aoc::*;

#[derive(Clone, Debug)]
pub struct SpringdroidAdventure(intcode::Program);

impl Fro for SpringdroidAdventure {
    fn fro(input: &str) -> Self {
        SpringdroidAdventure(intcode::Program::fro(input))
    }
}

impl SpringdroidAdventure {
    fn run<I>(&mut self, input: I) -> i64
    where
        I: IntoIterator<Item = i64>,
    {
        let mut input = input.into_iter();
        let mut output = String::new();
        let mut clear = false;
        loop {
            use intcode::Yield::*;
            match self.0.step() {
                Halt => return 0,
                Step => continue,
                Input(i) => {
                    if let Some(next) = input.next() {
                        self.0[i] = next;
                    }
                }
                Output(10) if clear => {
                    clear = false;
                    println!("\x1B[2J{}", output);
                    output.clear();
                    thread::sleep(time::Duration::from_secs(1));
                }
                Output(o) if o > 255 => {
                    return o;
                }
                Output(o) => {
                    clear = o == 10;
                    output.push(o as u8 as char);
                }
            }
        }
    }
}

impl Solution for SpringdroidAdventure {
    fn one(mut self) -> i64 {
        let input = [
            "NOT A T", "OR T J", "NOT B T", "OR T J", "NOT C T", "OR T J", "AND D J", "WALK",
        ]
        .iter()
        .copied()
        .flat_map(ascii);
        self.run(input)
    }

    fn two(mut self) -> i64 {
        let input = [
            // Hole at A, B, or C
            "NOT A T", "NOT B J", "OR T J", "NOT C T", "OR T J",
            // Future opportunity at E -> F or E -> I
            "NOT I T", "NOT T T", "OR F T", "AND E T",
            // Future opportunity at D -> H
            "OR H T", "AND T J", // Ground at D
            "AND D J", "RUN",
        ]
        .iter()
        .copied()
        .flat_map(ascii);
        self.run(input)
    }
}

fn ascii<'s>(string: &'s str) -> impl Iterator<Item = i64> + 's {
    string
        .bytes()
        .chain(iter::once(b'\n'))
        .map(|byte| byte as i64)
}
