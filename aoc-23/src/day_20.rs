use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::VecDeque;

use aoc::*;

#[derive(Clone, Debug)]
pub struct PulsePropagation(HashMap<String, Module>);

#[derive(Clone, Debug)]
struct Module {
    r#type: Option<Type>,
    output: Vec<String>,
}

#[derive(Copy, Clone, Debug)]
enum Type {
    Conj,
    Flip,
}

impl Fro for PulsePropagation {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let (a, b) = line.split_once(" -> ").unwrap();
                let (r#type, name) = if let Some(a) = a.strip_prefix('%') {
                    (Some(Type::Flip), a)
                } else if let Some(a) = a.strip_prefix('&') {
                    (Some(Type::Conj), a)
                } else {
                    (None, a)
                };

                (
                    name.to_owned(),
                    Module {
                        r#type,
                        output: b.split(", ").map(String::from).collect(),
                    },
                )
            })
            .collect::<HashMap<_, _>>()
            .tap(Self)
    }
}

impl Solution for PulsePropagation {
    fn one(self) -> i64 {
        let mut work = VecDeque::new();

        let mut flips = HashMap::new();
        let mut conjs = HashMap::<_, HashMap<_, _>>::new();

        // Initialize conjuction memory
        for (name, module) in &self.0 {
            for output in &module.output {
                if matches!(
                    self.0.get(output).and_then(|module| module.r#type),
                    Some(Type::Conj)
                ) {
                    conjs
                        .entry(output.as_str())
                        .or_default()
                        .insert(name.as_str(), false);
                }
            }
        }

        let mut low = 0;
        let mut high = 0;

        for _ in 0..1000 {
            work.push_back(("", "broadcaster", false));
            while let Some((prev, next, pulse)) = work.pop_front() {
                dbg!((prev, next, pulse));
                if pulse {
                    high += 1;
                } else {
                    low += 1;
                }

                let Some(module) = self.0.get(next) else {
                    continue;
                };

                match module.r#type {
                    Some(Type::Conj) => {
                        *conjs.get_mut(next).unwrap().get_mut(prev).unwrap() = pulse;

                        let send = !conjs.get(next).unwrap().values().all(|pulse| *pulse);

                        for output in &module.output {
                            work.push_back((next, output, send));
                        }
                    }
                    Some(Type::Flip) => {
                        if pulse {
                            continue;
                        }

                        let entry = flips.entry(next).or_insert(false);
                        *entry ^= true;
                        for output in &module.output {
                            work.push_back((next, output, *entry));
                        }
                    }
                    None => {
                        for output in &module.output {
                            work.push_back((next, output, pulse));
                        }
                        continue;
                    }
                }
            }
        }

        low * high
    }

    fn two(self) -> i64 {
        let mut work = VecDeque::new();

        let mut flips = BTreeMap::new();
        let mut conjs = HashMap::<_, BTreeMap<_, _>>::new();

        // Initialize conjuction memory
        for (name, module) in &self.0 {
            for output in &module.output {
                if matches!(
                    self.0.get(output).and_then(|module| module.r#type),
                    Some(Type::Conj)
                ) {
                    conjs
                        .entry(output.as_str())
                        .or_default()
                        .insert(name.as_str(), false);
                }
            }
        }

        // println!("digraph pulses {{");
        // for (name, module) in &self.0 {
        //     println!(
        //         "{} [label = \"{}{}\"];",
        //         name,
        //         match module.r#type {
        //             Some(Type::Conj) => "&",
        //             Some(Type::Flip) => "%",
        //             None => "",
        //         },
        //         name
        //     );
        //     for output in &module.output {
        //         println!("{} -> {};", name, output);
        //     }
        // }
        // println!("}}");

        // for (st, intermediate) in [("tk", "qs"), ("rt", "xj"), ("cr", "km"), ("fv", "kz")] {
        for (st, intermediate) in [("cr", "km"), ("fv", "kz")] {
            for press in 1.. {
                if press > 1 && conjs[intermediate].values().all(|val| !*val) {
                    println!("{}", press);
                    break;
                }

                if press > 1 {
                    // for s in [
                    //     "cr", "vz", "hf", "tl", "br", "fk", "gk", "xt", "gh", "dj", "fj", "sl",
                    // ] {

                    println!();
                }

                // flips
                //     .iter()
                //     .collect::<Vec<_>>()
                //     .tap_mut(|vec| vec.sort_unstable())
                //     .into_iter()
                //     .for_each(|item| if *item.1 { print!("1") } else { print!("0") });
                // println!();

                work.push_back(("", st, false));
                while let Some((prev, next, pulse)) = work.pop_front() {
                    let Some(module) = self.0.get(next) else {
                        continue;
                    };

                    match module.r#type {
                        Some(Type::Conj) => {
                            *conjs.get_mut(next).unwrap().get_mut(prev).unwrap() = pulse;

                            let send = !conjs.get(next).unwrap().values().all(|pulse| *pulse);

                            for output in &module.output {
                                work.push_back((next, output, send));
                            }
                        }
                        Some(Type::Flip) => {
                            if pulse {
                                continue;
                            }

                            let entry = flips.entry(next).or_insert(false);
                            *entry ^= true;
                            for output in &module.output {
                                work.push_back((next, output, *entry));
                            }
                        }
                        None => {
                            for output in &module.output {
                                work.push_back((next, output, pulse));
                            }
                            continue;
                        }
                    }
                }
            }
        }

        todo!();
    }
}
