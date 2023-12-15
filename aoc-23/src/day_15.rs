use aoc::*;

#[derive(Clone, Debug)]
pub struct LensLibrary(Vec<String>);

impl Fro for LensLibrary {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split(',')
            .map(|line| line.to_owned())
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

fn hash(string: &str) -> u8 {
    let mut hash = 0u8;
    for byte in string.bytes() {
        hash = hash.wrapping_add(byte);
        hash = hash.wrapping_mul(17);
    }
    hash
}

impl Solution for LensLibrary {
    fn one(self) -> i64 {
        self.0
            .iter()
            .map(String::as_str)
            .map(hash)
            .map(|hash| hash as i64)
            .sum()
    }

    fn two(self) -> i64 {
        let mut map = vec![Vec::<(&str, i64)>::new(); 256];

        'outer: for line in &self.0 {
            match line.split_once('=') {
                None => {
                    let index = hash(line.trim_end_matches('-')) as usize;
                    map[index as usize].retain(|(k, _)| *k != line.trim_end_matches('-'));
                }
                Some((a, b)) => {
                    let index = hash(a) as usize;
                    for (k, v) in &mut map[index as usize] {
                        if *k == a {
                            *v = b.to::<i64>();
                            continue 'outer;
                        }
                    }

                    map[index].push((a, b.to::<i64>()));
                }
            }
        }

        map.iter()
            .enumerate()
            .map(|(index, slot)| {
                (1 + index as i64)
                    * slot
                        .iter()
                        .enumerate()
                        .map(|(index, (_, length))| (1 + index as i64) * *length)
                        .sum::<i64>()
            })
            .sum()
    }
}
