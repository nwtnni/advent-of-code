use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp;
use std::str::FromStr;


use aoc::*;

#[derive(Debug, Clone)]
pub struct Placeholder(Vec<Digit>);

impl Fro for Placeholder {
    fn fro(input: &str) -> Self {
        Placeholder(
            input.split("")
                .filter_map(|s| Digit::from_str(s).ok())
                .collect()
        )
    }
}

impl Solution for Placeholder {
    fn one(self) -> i32 {
        let w = 25;
        let h = 6;

        let mut layers = Vec::<Vec<Digit>>::new();
        for row in 0..(self.0.len() / h / w){
            let mut layer = Vec::new();
            for pic in 0..h * w {
                layer.push(self.0[row * h * w + pic]);
            }
            layers.push(layer);
        }

        let (layer, _) = layers.iter()
            .enumerate()
            .map(|(i, layer)| (i, layer.iter().filter(|d| **d == Digit::D0).count()))
            .min_by_key(|(_, zeroes)| *zeroes)
            .unwrap();

        let (a, b) = layers[layer]
            .iter()
            .map(|d| ((*d == Digit::D1) as i32, (*d == Digit::D2) as i32))
            .fold((0, 0), |(l, r), (a, b)| (l + a, r + b));
        
        (a * b)

    }

    fn two(self) -> i32 {
        let w = 25;
        let h = 6;

        let mut layers = Vec::<Vec<Digit>>::new();
        for row in 0..(self.0.len() / h / w){
            let mut layer = Vec::new();
            for pic in 0..h * w {
                layer.push(self.0[row * h * w + pic]);
            }
            layers.push(layer);
        }

        let mut line = String::new();
        for i in 0..h * w {
            line.push_str(&(layers.iter()
                .rev()
                .map(|layer| layer[i])
                .fold(Digit::D2, |color, pixel| {
                    match (color, pixel) {
                    | (_, Digit::D2) => color,
                    | _ => pixel,
                    }
                }) as i32).to_string());

            if i % 25 == 0 {
                println!("{}", line);
                line.clear();
            }

        }

        println!("{}", line);


        0
    }
}
