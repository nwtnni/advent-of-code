use aoc::*;

#[derive(Clone, Debug)]
pub struct ShuttleSearch {
    time: i64,
    buses: Vec<Option<i64>>
}

impl Fro for ShuttleSearch {
    fn fro(input: &str) -> Self {

        let mut iter = input
            .trim()
            .split('\n');

        let time = iter.give().to::<i64>();
        let buses = iter.give()
            .trim()
            .split(',')
            .map(|bus| {
                if bus.trim() == "x" {
                    None
                } else {
                    Some(i64::fro(bus))
                }
            })
            .collect::<Vec<_>>();

        Self {
            time,
            buses,
        }

    }
}

impl Solution for ShuttleSearch {
    fn one(self) -> i64 {
        let (wait, bus) = self
            .buses
            .iter()
            .filter_map(|a| *a)
            .map(|bus| {
                let mut time = self.time;
                while time - bus > 0 {
                    time -= bus;
                }
                (time, bus)
            })
            .max_by_key(|(time, _)| *time)
            .unwrap();

        (bus - wait) * bus
    }

    fn two(self) -> i64 {
        todo!()
    }
}
