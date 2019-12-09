use std::cmp;

use aoc::*;

pub struct AmplificationCircuit(intcode::Program);

impl Fro for AmplificationCircuit {
    fn fro(input: &str) -> Self {
        AmplificationCircuit(intcode::Program::fro(input))
    }
}

impl Solution for AmplificationCircuit {
    fn one(self) -> i64 {
        let mut max = 0;
        let mut settings = [0, 1, 2, 3, 4];
        let mut programs = vec![self.0; 5];

        for (l, r) in permute(5) {

            settings.swap(l, r);

            for (program, setting) in programs.iter_mut().zip(&settings) {
                program.reset();
                program.input(*setting);
            }

            let out = programs
                .iter_mut()
                .try_fold(0, |input, program| program.pipe(input))
                .unwrap();

            max = cmp::max(out, max);
        }
        max
    }

    fn two(self) -> i64 {
        let mut max = 0;
        let mut settings = [5, 6, 7, 8, 9];
        let mut programs = vec![self.0; 5];

        for (l, r) in permute(5) {

            settings.swap(l, r);

            for (program, setting) in programs.iter_mut().zip(&settings) {
                program.reset();
                program.input(*setting);
            }

            let mut input = 0;
            'outer: loop {
                for program in &mut programs {
                    match program.pipe(input) {
                    | Some(output) => input = output,
                    | None => break 'outer,
                    }
                }
            }

            max = cmp::max(input, max);
        }
        max
    }
}
