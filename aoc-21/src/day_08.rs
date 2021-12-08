use std::collections::HashMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct SevenSegmentSearch(Vec<Display>);

#[derive(Copy, Clone, Debug)]
struct Display {
    inputs: [AsciiSet; 10],
    outputs: [AsciiSet; 4],
}

impl Fro for SevenSegmentSearch {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(Display::fro)
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Fro for Display {
    fn fro(input: &str) -> Self {
        let (left, right) = input.split_once(" | ").unwrap();
        let mut inputs = [AsciiSet::none(); 10];
        let mut outputs = [AsciiSet::none(); 4];

        left
            .trim()
            .split(' ')
            .map(AsciiSet::from)
            .enumerate()
            .for_each(|(i, input)| inputs[i] = input);

        right
            .trim()
            .split(' ')
            .map(AsciiSet::from)
            .enumerate()
            .for_each(|(i, output)| outputs[i] = output);

        Display { inputs, outputs }
    }
}

impl Solution for SevenSegmentSearch {
    fn one(self) -> i64 {
        self.0
            .iter()
            .map(|display| {
                display
                    .outputs
                    .iter()
                    .filter(|output| match output.len() {
                        2 | 3 | 4 | 7 => true,
                        _ => false,
                    }).count()
            })
            .sum::<usize>()
            as i64
    }

    fn two(self) -> i64 {

       static SEGMENTS: &str = "abcdefg";

       let mut total = 0;

       for display in &self.0 {
            let mut mapping = SEGMENTS
                .chars()
                .map(|char| (char, AsciiSet::from(SEGMENTS)))
                .collect::<HashMap<_, _>>();

            let mut narrow = |set: AsciiSet, possible: &str| {
                for char in set {
                    mapping
                        .get_mut(&char)
                        .unwrap()
                        .and_mut(possible);
                }
            };

            for input in display.inputs {
                match input.len() {
                    2 => narrow(input, "cf"), // 1
                    3 => narrow(input, "acf"), // 7
                    4 => narrow(input, "bcdf"), // 4
                    5 => narrow(input.not(SEGMENTS), "bcef"), // 2, 3, 5
                    6 => narrow(input.not(SEGMENTS), "cde"), // 0, 6, 9
                    7 => (),
                    _ => unreachable!(),
                }
            }

            while mapping.values().any(|possible| possible.len() > 1) {
                let unknown = mapping
                    .values()
                    .copied()
                    .filter(|possible| possible.len() == 1)
                    .flatten()
                    .collect::<AsciiSet>()
                    .not(SEGMENTS);

                mapping
                    .values_mut()
                    .filter(|possible| possible.len() > 1)
                    .for_each(|possible| possible.and_mut(unknown));
            }

            let digits = [
                AsciiSet::from("abcefg"),
                AsciiSet::from("cf"),
                AsciiSet::from("acdeg"),
                AsciiSet::from("acdfg"),
                AsciiSet::from("bcdf"),
                AsciiSet::from("abdfg"),
                AsciiSet::from("abdefg"),
                AsciiSet::from("acf"),
                AsciiSet::from("abcdefg"),
                AsciiSet::from("abcdfg"),
            ];

            for (place, output) in display.outputs.into_iter().rev().enumerate() {
                let mapped = output
                    .into_iter()
                    .map(|char| mapping[&char])
                    .flatten()
                    .collect::<AsciiSet>();

                for (i, digit) in digits.into_iter().enumerate() {
                    if mapped == digit {
                        total += 10i64.pow(place as u32) * i as i64;
                        break;
                    }
                }
            }
        }

        total
    }
}
