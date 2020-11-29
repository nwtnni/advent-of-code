use std::iter;

pub fn fst<A, B>((a, _): (A, B)) -> A {
    a
}

pub fn snd<A, B>((_, b): (A, B)) -> B {
    b
}

pub fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b > 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

// See https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
pub fn mod_inv(a: i64, b: i64) -> i64 {
    let mut i = true;
    let mut r = [a, b];
    let mut s = [1i64, 0i64];

    macro_rules! prev { ($arr:expr) => { $arr[(i ^ true) as usize] } }
    macro_rules! here { ($arr:expr) => { $arr[i as usize] } }

    loop {
        let qi = prev!(r).div_euclid(here!(r));
        let ri = prev!(r).rem_euclid(here!(r));
        if ri == 0 {
            return here!(s).rem_euclid(b);
        }
        prev!(r) = ri;
        prev!(s) -= qi * here!(s);
        i ^= true;
    }
}

pub fn permute(len: u8) -> impl Iterator<Item = (usize, usize)> {
    iter::once((0, 0)).chain(Permutations::new(len))
}

/// Bitset for case-insensitive ASCII letters
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AlphaSet(u32);

impl AlphaSet {
    pub fn new() -> Self {
        AlphaSet::default()
    }

    pub fn insert(&mut self, alpha: char) -> bool {
        let mask = Self::mask(alpha);
        if self.0 & mask > 0 {
            false
        } else {
            self.0 |= mask;
            true
        }
    }

    pub fn contains(&self, alpha: char) -> bool {
        let mask = Self::mask(alpha);
        self.0 & mask > 0
    }

    pub fn and(&self, other: &Self) -> Self {
        Self(self.0 & other.0)
    }

    pub fn or(&self, other: &Self) -> Self {
        Self(self.0 | other.0)
    }

    pub fn xor(&self, other: &Self) -> Self {
        Self(self.0 ^ other.0)
    }

    pub fn not(&self) -> Self {
        Self(!self.0 & 0b0000_0011_1111_1111_1111_1111_1111_1111)
    }

    pub fn sub(&self, other: &Self) -> Self {
        self.and(&other.not())
    }

    fn mask(alpha: char) -> u32 {
        assert!(alpha.is_ascii_alphabetic());
        1 << (((alpha.to_ascii_lowercase() as u32) as u8) - b'a')
    }
}

impl<'a> From<&'a str> for AlphaSet {
    fn from(string: &'a str) -> Self {
        string.chars().collect()
    }
}

impl iter::FromIterator<char> for AlphaSet {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut set = AlphaSet::new();
        for char in iter {
            set.insert(char);
        }
        set
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Dir { P, Z, N, }

impl Dir {
    fn step(&self, start: u8) -> u8 {
        match self {
        | Dir::P => start + 1,
        | Dir::Z => start,
        | Dir::N => start - 1,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Info {
    pos: u8,
    dir: Dir,
}

/// Implementation of the Steinhaus-Johnson-Trotter algorithm
///
/// https://en.wikipedia.org/wiki/Steinhaus%E2%80%93Johnson%E2%80%93Trotter_algorithm
#[derive(Clone, Debug)]
pub struct Permutations {
    /// Upper bound on indices
    bound: u8,

    /// Completed permutations
    count: u8,

    /// Current permutation as indices
    data: Vec<u8>,

    /// Position and direction of each index
    info: Vec<Info>,
}

impl Permutations {
    fn new(len: u8) -> Self {
        let data = (0..len).collect::<Vec<_>>();
        let mut info = (0..len)
            .map(|pos| Info { pos, dir: Dir::N })
            .collect::<Vec<_>>();
        info[0] = Info { pos: 0, dir: Dir::Z };
        Permutations {
            bound: len - 1,
            count: 0,
            data,
            info,
        }
    }
}

impl Iterator for Permutations {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let (init_idx, init_pos, mut init_dir) = self.info.iter_mut()
            .enumerate()
            .rev()
            .filter(|(_, Info { dir, .. })| *dir != Dir::Z)
            .map(|(idx, Info { pos, dir })| (idx, *pos, *dir))
            .next()?;

        let swap_pos = init_dir.step(init_pos);
        let swap_idx = self.data[swap_pos as usize] as usize;
        let swap_dir = self.info[swap_idx].dir;

        if swap_pos == 0 || swap_pos == self.bound {
            init_dir = Dir::Z;
        } else {
            let next_pos = init_dir.step(swap_pos);
            let next_idx = self.data[next_pos as usize] as usize;
            if next_idx > init_idx {
                init_dir = Dir::Z;
            }
        };

        self.info[init_idx] = Info { pos: swap_pos, dir: init_dir };
        self.info[swap_idx] = Info { pos: init_pos, dir: swap_dir };
        self.data.swap(init_pos as usize, swap_pos as usize);
        self.count += 1;

        for (_, Info { pos, dir }) in self.info.iter_mut()
            .enumerate()
            .rev()
            .take_while(|(idx, _)| *idx > init_idx)
        {
            *dir = if *pos < swap_pos { Dir::P } else { Dir::N };
        }

        Some((init_idx, swap_idx))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let p = (1..=self.data.len()).product::<usize>();
        let n = p - self.count as usize - 1;
        (n, Some(n))
    }
}
