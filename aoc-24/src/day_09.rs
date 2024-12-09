use std::collections::BTreeMap;

use aoc::*;

#[derive(Clone, Debug)]
pub struct DiskFragmenter(Vec<File>);

#[derive(Clone, Debug)]
struct File {
    size: i64,
    free: i64,
}

impl Fro for DiskFragmenter {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .as_bytes()
            .chunks(2)
            .map(|chunk| {
                let size = (chunk[0] - b'0') as i64;
                let free = (chunk.get(1).copied().unwrap_or(b'0') - b'0') as i64;
                File { size, free }
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for DiskFragmenter {
    fn one(self) -> i64 {
        let mut disk = vec![None; self.all()];
        let used = self.used();

        let mut i = 0;
        for (id, file) in self.0.iter().enumerate() {
            disk[i..][..file.size as usize].fill(Some(id));
            i += (file.size + file.free) as usize;
        }

        let mut r = disk.len();
        for l in 0..used {
            if disk[l].is_some() {
                continue;
            }

            r = (r - 1) - disk[..r].iter().rev().position(Option::is_some).unwrap();

            disk[l] = disk[r];
            disk[r] = None;
        }

        disk.iter()
            .copied()
            .take(used)
            .map(Option::unwrap)
            .enumerate()
            .map(|(i, id)| i * id)
            .sum::<usize>() as i64
    }

    fn two(self) -> i64 {
        let mut free = BTreeMap::new();
        let mut used = BTreeMap::new();

        let mut i = 0;
        for (id, file) in self.0.iter().enumerate() {
            let j = i + file.size as usize;
            let k = i + (file.size + file.free) as usize;

            used.insert(id, i);
            free.insert(j, file.free);
            i = k;
        }

        for (id, file) in self.0.iter().enumerate().rev() {
            let j = used.get_mut(&id).unwrap();

            let Some(i) = free
                .iter()
                .find(|(_, size)| **size >= file.size)
                .filter(|(i, _)| **i < *j)
                .map(|(i, _)| *i)
            else {
                continue;
            };

            // New location
            let hole = free.remove(&i).unwrap();
            if hole > file.size {
                free.insert(i + file.size as usize, hole - file.size);
            }

            // Old location
            free.insert(*j, file.size);
            *j = i;
        }

        used.iter()
            .map(|(id, i)| (id, *i, i + self.0[*id].size as usize))
            .flat_map(|(id, i, j)| (i..j).map(move |i| i * id))
            .sum::<usize>() as i64
    }
}

impl DiskFragmenter {
    fn used(&self) -> usize {
        self.0.iter().map(|file| file.size).sum::<i64>() as usize
    }

    fn all(&self) -> usize {
        self.0.iter().map(|file| file.size + file.free).sum::<i64>() as usize
    }
}
