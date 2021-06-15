use std::collections::HashMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct ReindeerOlympics(Vec<Reindeer>);

#[derive(Copy, Clone, Debug)]
struct Reindeer {
    name: &'static str,
    speed: i64,
    flying: i64,
    resting: i64,
}

impl Fro for ReindeerOlympics {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .filter_map(|line| {
                let line = line.strip_suffix(" seconds.")?;
                let (left, resting) = line.split_once(" seconds, but then must rest for ")?;
                let (left, flying) = left.split_once(" km/s for ")?;
                let (name, speed) = left.split_once(" can fly ")?;
                Some(Reindeer {
                    name: name.leak(),
                    speed: i64::fro(speed),
                    flying: i64::fro(flying),
                    resting: i64::fro(resting),
                })
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for ReindeerOlympics {
    fn one(self) -> i64 {
        let mut position = HashMap::new();

        for time in 0..2503 {
            for reindeer in &self.0 {
                if time % (reindeer.flying + reindeer.resting) < reindeer.flying {
                    *position.entry(reindeer.name).or_insert(0) += reindeer.speed;
                }
            }
        }

        position.values().max().copied().unwrap()
    }

    fn two(self) -> i64 {
        let mut position = HashMap::new();
        let mut score = HashMap::new();

        for time in 0..2503 {
            for reindeer in &self.0 {
                if time % (reindeer.flying + reindeer.resting) < reindeer.flying {
                    *position.entry(reindeer.name).or_insert(0) += reindeer.speed;
                }
            }

            let winning = position
                .iter()
                .max_by_key(|(_, distance)| *distance)
                .map(|(name, _)| *name)
                .unwrap();

            *score.entry(winning).or_insert(0) += 1;
        }

        score.values().max().copied().unwrap()
    }
}
