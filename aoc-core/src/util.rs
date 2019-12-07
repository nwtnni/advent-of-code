use std::iter;

pub fn fst<A, B>((a, _): (A, B)) -> A {
    a 
}

pub fn snd<A, B>((_, b): (A, B)) -> B {
    b
}

pub fn permute(len: usize) -> impl Iterator<Item = (usize, usize)> {
    iter::once((0, 0)).chain(Permutations::new(len))
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Dir { P, Z, N, }

impl Dir {
    fn step(&self, start: usize) -> usize {
        match self {
        | Dir::P => start + 1,
        | Dir::Z => start,
        | Dir::N => start - 1,
        }
    }
}

/// Implementation of the Steinhaus-Johnson-Trotter algorithm
///
/// https://en.wikipedia.org/wiki/Steinhaus%E2%80%93Johnson%E2%80%93Trotter_algorithm
#[derive(Clone, Debug)]
pub struct Permutations {
    data: Vec<(Dir, usize)>,
    done: usize,
}

impl Permutations {
    fn new(len: usize) -> Self {
        let mut data = (0..len)
            .map(|idx| (Dir::N, idx))
            .collect::<Vec<_>>();
        data[0] = (Dir::Z, 0);
        Permutations {
            data,
            done: 0,
        }
    }
}

impl Iterator for Permutations {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let (init_idx, init_dir, init_ele) = self.data
            .iter()
            .enumerate()
            .filter(|(_, (dir, _))| *dir != Dir::Z)
            .max_by_key(|(_, (_, ele))| ele)
            .map(|(idx, (dir, ele))| (idx, *dir, *ele))?;

        let swap_idx = init_dir.step(init_idx);
        let swap_dir = if swap_idx == 0
        || swap_idx == self.data.len() - 1
        || self.data[init_dir.step(swap_idx)].1 > init_ele {
            Dir::Z
        } else {
            init_dir
        };

        self.data.swap(init_idx, swap_idx);
        self.data[swap_idx].0 = swap_dir;
        self.done += 1;

        for (idx, (dir, ele)) in self.data.iter_mut().enumerate() {
            if *ele > init_ele && *dir == Dir::Z {
                *dir = if idx < swap_idx {
                    Dir::P
                } else {
                    Dir::N
                };
            }
        }

        Some((init_idx, swap_idx))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let p = (1..=self.data.len()).product::<usize>();
        let n = p - self.done - 1;
        (n, Some(n))
    }
}
