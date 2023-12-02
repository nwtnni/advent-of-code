use aoc::*;

#[derive(Clone, Debug)]
pub struct CubeConundrum(Vec<Vec<Draw>>);

impl Fro for CubeConundrum {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let (_, game) = line.split_once(": ").unwrap();
                let mut draws = Vec::new();
                for rolls in game.split(';').map(str::trim) {
                    let mut draw = Draw::default();
                    for roll in rolls.split(',').map(str::trim) {
                        let (n, color) = roll.split_once(' ').unwrap();
                        match color {
                            "red" => draw.r = n.parse::<i64>().unwrap(),
                            "green" => draw.g = n.parse::<i64>().unwrap(),
                            "blue" => draw.b = n.parse::<i64>().unwrap(),
                            _ => unreachable!(),
                        }
                    }
                    draws.push(draw);
                }
                draws
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

#[derive(Clone, Default, Debug)]
struct Draw {
    r: i64,
    g: i64,
    b: i64,
}

impl Solution for CubeConundrum {
    fn one(self) -> i64 {
        self.0
            .iter()
            .enumerate()
            .filter_map(|(index, game)| {
                if game
                    .iter()
                    .all(|draw| draw.r <= 12 && draw.g <= 13 && draw.b <= 14)
                {
                    Some(index + 1)
                } else {
                    None
                }
            })
            .sum::<usize>() as i64
    }

    fn two(self) -> i64 {
        self.0
            .iter()
            .map(|game| {
                let r = game.iter().map(|draw| draw.r).max().unwrap();
                let g = game.iter().map(|draw| draw.g).max().unwrap();
                let b = game.iter().map(|draw| draw.b).max().unwrap();
                r * g * b
            })
            .sum::<i64>()
    }
}
