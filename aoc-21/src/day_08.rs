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

       static ALPHABET: &str = "abcdefg";

       fn i(set: AsciiSet) -> impl Iterator<Item = char> {
           ALPHABET.chars().filter(move |c| set.contains(*c))
       }

       let mut total = 0;

       for display in &self.0 {
            let mut mapping = HashMap::new();
            for c in ALPHABET.chars() {
                mapping.insert(c, AsciiSet::from(ALPHABET));
            }

            let mut cf = None;

            while mapping.values().any(|set| set.len() > 1) {
                for input in display.inputs {
                    match input.len() {
                        // 1
                        2 => {
                            for c in i(input) {
                                let new = mapping[&c].and("cf".chars().collect::<AsciiSet>());
                                mapping.insert(c, new);
                            }
                            cf = Some(input);
                        }
                        // 7
                        3 => {
                            for c in i(input) {
                                let new = mapping[&c].and("acf".chars().collect::<AsciiSet>());
                                mapping.insert(c, new);
                            }
                        }
                        // 4
                        4 => {
                            for c in i(input) {
                                let new = mapping[&c].and("bcdf".chars().collect::<AsciiSet>());
                                mapping.insert(c, new);
                            }
                        }
                        // 8
                        7 => (),
                        // 0, 6, 9
                        6 => {
                            let missing = "abcdefg".chars().find(|c| !input.contains(*c)).unwrap();
                            if let Some(cf) = cf {
                                // must be 6
                                if cf.contains(missing) {
                                    mapping.insert(missing, AsciiSet::from('c'));
                                    for c in i(cf) {
                                        if c != missing {
                                            mapping.insert(c, AsciiSet::from('f'));
                                        }
                                    }
                                }
                            } else {
                                let new = mapping[&missing].and("cde".chars().collect::<AsciiSet>());
                                mapping.insert(missing, new);
                            }
                        }
                        // 2, 3, 5
                        5 => {
                            let missing = "abcdefg".chars().filter(|c| !input.contains(*c));

                            for c in missing {
                                let new = mapping[&c].and("bcef".chars().collect::<AsciiSet>());
                                mapping.insert(c, new);
                            }

                        }
                        _ => unreachable!(),
                    }
                }

                let known = mapping
                    .values()
                    .filter(|set| set.len() == 1)
                    .map(|set| i(*set).give())
                    .collect::<AsciiSet>();

                let unknown = ALPHABET
                    .chars()
                    .filter(|c| !known.contains(*c))
                    .collect::<AsciiSet>();

                for v in mapping.values_mut() {
                    if v.len() > 1 {
                        *v = v.and(unknown);
                    }
                }
            }

            let sets = [
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
                let mapped = i(output)
                    .map(|c| i(mapping[&c]).give())
                    .collect::<AsciiSet>();

                for (i, set) in sets.into_iter().enumerate() {
                    if mapped == set {
                        total += 10i64.pow(place as u32) * i as i64;
                    }
                }
            }
        }

        total
    }
}
