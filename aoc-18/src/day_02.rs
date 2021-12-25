use aoc::*;

#[derive(Clone, Debug)]
pub struct InventoryManagementSystem(Vec<String>);

impl Fro for InventoryManagementSystem {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(String::from)
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for InventoryManagementSystem {
    fn one(self) -> i64 {
        let mut twos = 0;
        let mut threes = 0;

        for id in &self.0 {
            let mut counter = [0; 26];

            for char in id.chars() {
                counter[(char as u8 - b'a') as usize] += 1;
            }

            if counter.iter().any(|count| *count == 2) {
                twos += 1;
            }

            if counter.iter().any(|count| *count == 3) {
                threes += 1;
            }
        }

        twos * threes
    }

    fn two(self) -> i64 {
        for i in 0..self.0.len() {
            for j in i..self.0.len() {
                let mut differences = self.0[i]
                    .chars()
                    .zip(self.0[j].chars())
                    .enumerate()
                    .filter(|(_, (a, b))| a != b);

                match (differences.next(), differences.next()) {
                    (None, None) | (Some(_), Some(_)) => continue,
                    (None, Some(_)) => unreachable!(),
                    (Some((k, _)), None) => {
                        println!("{}{}", &self.0[i][0..k], &self.0[i][k + 1..]);
                        panic!("See `stdout` for solution.")
                    }
                }
            }
        }
        unreachable!()
    }
}
