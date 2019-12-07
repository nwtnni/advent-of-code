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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Info {
    pos: usize,
    dir: Dir,
}

/// Implementation of the Steinhaus-Johnson-Trotter algorithm
///
/// https://en.wikipedia.org/wiki/Steinhaus%E2%80%93Johnson%E2%80%93Trotter_algorithm
#[derive(Clone, Debug)]
pub struct Permutations {
    /// Upper bound on indices
    bound: usize,

    /// Completed permutations
    count: usize,

    /// Current permutation as indices
    data: Vec<usize>,

    /// Position and direction of each index
    info: Vec<Info>,
}

impl Permutations {
    fn new(len: usize) -> Self {
        let data = (0..len).collect::<Vec<_>>();
        let mut info = (0..len)
            .map(|pos| Info { pos, dir: Dir::N })
            .collect::<Vec<_>>();
        info[0] = Info { pos: 0, dir: Dir::Z };
        Permutations {
            bound: data.len() - 1,
            count: 0,
            data,
            info,
        }
    }
}

impl Iterator for Permutations {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let (init_idx, &Info { pos: init_pos, dir: init_dir }) = self.info.iter()
            .enumerate()
            .rev()
            .filter(|(_, Info { dir, .. })| *dir != Dir::Z)
            .next()?;

        let swap_pos = init_dir.step(init_pos);
        let swap_idx = self.data[swap_pos];
        let swap_dir = self.info[swap_idx].dir;

        let next_dir = if swap_pos == 0
        || swap_pos == self.bound
        || self.data[init_dir.step(swap_pos)] > init_idx {
            Dir::Z
        } else {
            init_dir
        };

        self.info[init_idx] = Info { pos: swap_pos, dir: next_dir };
        self.info[swap_idx] = Info { pos: init_pos, dir: swap_dir };
        self.data.swap(init_pos, swap_pos);
        self.count += 1;

        for (_, Info { pos, dir }) in self.info.iter_mut()
            .enumerate()
            .rev()
            .take_while(|(idx, _)| *idx > init_idx)
        {
            *dir = if *pos < swap_pos {
                Dir::P
            } else {
                Dir::N
            };
        }

        Some((init_idx, swap_idx))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let p = (1..=self.data.len()).product::<usize>();
        let n = p - self.count - 1;
        (n, Some(n))
    }
}
