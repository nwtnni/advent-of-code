use std::iter;

use aoc::*;

#[derive(Clone, Debug)]
pub struct GrovePositioningSystem(Vec<i64>);

impl Fro for GrovePositioningSystem {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(i64::fro)
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for GrovePositioningSystem {
    fn one(self) -> i64 {
        let values = self.0;
        let mut forward = (1..values.len()).chain(iter::once(0)).collect::<Vec<_>>();
        let mut backward = iter::once(values.len() - 1)
            .chain(0..values.len() - 1)
            .collect::<Vec<_>>();

        dbg!(&backward);

        let mut debug = 0;
        for _ in 0..values.len() {
            print!("{},", values[debug]);
            debug = forward[debug];
        }
        println!();

        for i in 0..values.len() {
            let mut next = forward[i];

            for _ in 0..match values[i] {
                value if value < 0 => (value - 1).rem_euclid(values.len() as i64),
                value => value % values.len() as i64,
            } {
                // dbg!((next, values[next]));
                next = forward[next];
            }
            // dbg!((next, values[next]));

            // Remove
            forward[backward[i]] = forward[i];
            backward[forward[i]] = backward[i];

            // Insert
            forward[i] = next;
            backward[i] = backward[next];

            forward[backward[i]] = i;
            backward[forward[i]] = i;

            for i in 0..values.len() {
                assert_eq!(forward[backward[i]], i);
                assert_eq!(backward[forward[i]], i);

                assert_eq!(backward[forward[forward[i]]], forward[i]);
                assert_eq!(forward[forward[backward[i]]], forward[i]);

                assert_eq!(forward[backward[backward[i]]], backward[i]);
                assert_eq!(backward[backward[forward[i]]], backward[i]);

                assert_eq!(forward[backward[forward[i]]], forward[i]);
                assert_eq!(forward[backward[forward[i]]], forward[i]);
            }

            // let mut debug = 0;
            // for _ in 0..values.len() {
            //     print!("{},", values[debug]);
            //     debug = forward[debug];
            // }
            // println!();
        }

        let mut next = values.iter().position(|value| *value == 0).unwrap();
        let mut total = 0;

        for i in 1..=3000 {
            next = forward[next];

            if [1000, 2000, 3000].contains(&i) {
                total += dbg!(values[next]);
            }
        }

        total
    }

    fn two(self) -> i64 {
        todo!()
    }
}
