use std::fmt;

use aoc::*;

const W: usize = 25;
const H: usize = 6;

#[derive(Clone, Debug)]
pub struct SpaceImageFormat(Vec<Layer>);

#[derive(Clone, Debug)]
pub struct Layer(Vec<Pixel>);

impl fmt::Display for Layer {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for pixel in &self.0 {
            write!(fmt, "{}", pixel)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Pixel {
    Black = 0,
    White = 1,
    Transparent = 2,
}

impl Pixel {
    fn from_char(c: char) -> Self {
        match c {
        | '0' => Pixel::Black,
        | '1' => Pixel::White,
        | '2' => Pixel::Transparent,
        | _ => unreachable!()
        }
    }

    fn merge(bot: Self, top: Self) -> Self {
        match top {
        | Pixel::Transparent => bot,
        | _ => top,
        }
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
        | Pixel::Black => write!(fmt, "█"),
        | Pixel::White => write!(fmt, " "),
        | Pixel::Transparent => Ok(()),
        }
    }
}

impl Fro for SpaceImageFormat {
    fn fro(input: &str) -> Self {
        let mut layers = Vec::new();
        let mut iter = input.chars();
        let pixels = input.len();

        for _ in 0..pixels / W / H {
            layers.push(
                iter.by_ref()
                    .take(W * H)
                    .map(Pixel::from_char)
                    .collect::<Vec<_>>()
                    .tap(Layer)
            );
        }

        SpaceImageFormat(layers)
    }
}

impl Solution for SpaceImageFormat {
    fn one(self) -> i64 {
        let (layer, _) = self.0.iter()
            .enumerate()
            .map(|(i, layer)| (i, layer.0.iter().filter(|d| **d == Pixel::Black).count()))
            .min_by_key(|(_, zeroes)| *zeroes)
            .unwrap();

        let w = self.0[layer].0.iter()
            .filter(|p| **p == Pixel::White)
            .count() as i64;

        let t = self.0[layer].0.iter()
            .filter(|p| **p == Pixel::Transparent)
            .count() as i64;

        w * t
    }

    fn two(self) -> i64 {
        let mut layer = Layer(Vec::new());
        for row in 0..H {
            for col in 0..W {
                let visible = self.0.iter()
                    .rev()
                    .map(|layer| layer.0[row * W + col])
                    .fold(Pixel::Transparent, Pixel::merge);
                layer.0.push(visible);
            }
            println!("{}", layer);
            layer.0.clear();
        }
        0
    }
}
