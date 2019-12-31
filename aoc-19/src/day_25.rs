use std::collections::VecDeque;
use std::iter;

use aoc::*;
use aoc::intcode;

pub struct Cryostasis(intcode::Program);

impl Fro for Cryostasis {
    fn fro(input: &str) -> Self {
        input.to::<intcode::Program>().tap(Cryostasis)
    }
}

impl Solution for Cryostasis {

    fn one(mut self) -> i64 {

        let mut output = String::new();
        let mut input = [
            // Hull Breach
            "south",
            // Stables
            "take cake",
            "south",
            // Kitchen
            "west",
            // Arcade
            "take mutex",
            "east",
            // Kitchen
            "north",
            // Stables
            "north",
            // Hull Breach
            "west",
            // Sick Bay
            "take klein bottle",
            "south",
            // Hot Chocolate Fountain
            "east",
            // Gift Wrapping Center
            "take monolith",
            "south",
            // Crew Quarters
            "take fuel cell",
            "west",
            // Corridor
            "west",
            // Warp Drive Maintenance
            "take astrolabe",
            "east",
            // Corridor
            "east",
            // Crew Quarters
            "north",
            // Gift Wrapping Center
            "west",
            // Hot Chocolate Fountain
            "north",
            // Sick Bay
            "west",
            // Storage
            "north",
            // Engineering
            "take tambourine",
            "south",
            // Storage
            "west",
            // Navigation
            "take dark matter",
            "west",
            // Security Checkpoint
            "inv",
        ].iter()
            .copied()
            .flat_map(asciiln)
            .collect::<VecDeque<_>>();

        let mut inventory = 0b1111_1111u8;
        let items = [
            "cake",
            "mutex",
            "klein bottle",
            "monolith",
            "fuel cell",
            "astrolabe",
            "tambourine",
            "dark matter",
        ];

        let mut buffer = VecDeque::new();
        let mut count = 0;

        loop {
            use intcode::Yield::*;
            match self.0.step() {
            | Halt => {
                return output.split_whitespace()
                    .filter_map(|word| word.parse::<i64>().ok())
                    .give();
            }
            | Step => continue,
            | Input(i) => {
                if let Some(next) = input.pop_front() {
                    self.0[i] = next;
                    continue;
                }

                if buffer.is_empty() {
                    count += 1;

                    for i in 0..8 {
                        let mask = 1 << i;
                        if count & mask > 0 {
                            // Not in inventory but needed
                            if inventory & mask == 0 {
                                inventory |= mask;
                                buffer.extend(ascii("take "));
                                buffer.extend(asciiln(items[i]));
                            }
                        } else {
                            // In inventory but unneeded
                            if inventory & mask > 0 {
                                inventory &= !mask;
                                buffer.extend(ascii("drop "));
                                buffer.extend(asciiln(items[i]));
                            }
                        }
                    }

                    buffer.extend(asciiln("north"));
                }

                if let Some(next) = buffer.pop_front() {
                    print!("{}", next as u8 as char);
                    self.0[i] = next;
                    continue;
                }
            }
            | Output(o) => {
                if output.ends_with("Command?\n") {
                    print!("{}", output);
                    output.clear();
                }
                output.push(o as u8 as char);
            }
            }
        }
    }

    fn two(self) -> i64 {
        todo!()
    }
}

fn ascii<'s>(string: &'s str) -> impl Iterator<Item = i64> + 's {
    string.chars().map(|char| char as i64)
}

fn asciiln<'s>(string: &'s str) -> impl Iterator<Item = i64> + 's {
    ascii(string).chain(iter::once(10))
}
