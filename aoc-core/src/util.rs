use std::collections::HashMap;
use std::fmt;
use std::hash::Hash;
use std::iter;
use std::iter::FromIterator;
use std::ops::Deref;
use std::ops::DerefMut;
use std::str;

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

    macro_rules! prev {
        ($arr:expr) => {
            $arr[(i ^ true) as usize]
        };
    }
    macro_rules! here {
        ($arr:expr) => {
            $arr[i as usize]
        };
    }

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

/// Bitset for alphanumeric ASCII: `[a-zA-Z0-9]`
#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AsciiSet(u64);

pub static LOWERS: &str = "abcdefghijklmnopqrstuvwxyz";
pub static UPPERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub static DIGITS: &str = "0123456789";

impl AsciiSet {
    pub fn none() -> Self {
        Self::default()
    }

    pub fn from_case_insensitive(string: &str) -> Self {
        string
            .chars()
            .map(|char| char.to_ascii_lowercase())
            .collect()
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

    pub fn remove(&mut self, alpha: char) -> bool {
        let mask = Self::mask(alpha);
        if self.0 & mask > 0 {
            self.0 &= !mask;
            true
        } else {
            false
        }
    }

    pub fn contains(&self, alpha: char) -> bool {
        let mask = Self::mask(alpha);
        self.0 & mask > 0
    }

    pub fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn and<T: Into<Self>>(self, other: T) -> Self {
        Self(self.0 & other.into().0)
    }

    pub fn and_mut<T: Into<Self>>(&mut self, other: T) {
        *self = self.and(other);
    }

    pub fn or<T: Into<Self>>(self, other: T) -> Self {
        Self(self.0 | other.into().0)
    }

    pub fn or_mut<T: Into<Self>>(&mut self, other: T) {
        *self = self.or(other);
    }

    pub fn xor<T: Into<Self>>(self, other: T) -> Self {
        Self(self.0 ^ other.into().0)
    }

    pub fn xor_mut<T: Into<Self>>(&mut self, other: T) {
        *self = self.xor(other);
    }

    pub fn not<T: Into<Self>>(self, universe: T) -> Self {
        Self(!self.0 & universe.into().0)
    }

    pub fn not_mut<T: Into<Self>>(&mut self, universe: T) {
        *self = self.not(universe);
    }

    fn mask(alpha: char) -> u64 {
        let bit = match alpha {
            'a'..='z' => alpha as u8 - b'a',
            'A'..='Z' => (alpha as u8 - b'A') + 26,
            '0'..='9' => (alpha as u8 - b'0') + 52,
            other => panic!("Invalid value in `AsciiSet`: {:?}", other),
        };
        1 << bit
    }
}

impl<'a> From<&'a str> for AsciiSet {
    fn from(string: &'a str) -> Self {
        string.chars().collect()
    }
}

impl From<char> for AsciiSet {
    fn from(char: char) -> Self {
        let mut set = AsciiSet::none();
        set.insert(char);
        set
    }
}

impl iter::FromIterator<char> for AsciiSet {
    fn from_iter<T: IntoIterator<Item = char>>(iter: T) -> Self {
        let mut set = AsciiSet::none();
        for char in iter {
            set.insert(char);
        }
        set
    }
}

impl fmt::Debug for AsciiSet {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{{")?;
        for char in LOWERS.chars().chain(UPPERS.chars()).chain(DIGITS.chars()) {
            if self.contains(char) {
                write!(fmt, "{}", char)?;
            }
        }
        write!(fmt, "}}")
    }
}

impl IntoIterator for AsciiSet {
    type IntoIter = IntoIter;
    type Item = char;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            all: LOWERS.chars().chain(UPPERS.chars()).chain(DIGITS.chars()),
            set: self,
        }
    }
}

#[derive(Clone, Debug)]
pub struct IntoIter {
    all: iter::Chain<iter::Chain<str::Chars<'static>, str::Chars<'static>>, str::Chars<'static>>,
    set: AsciiSet,
}

impl Iterator for IntoIter {
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        for next in self.all.by_ref() {
            // A bit inefficient to use `contains` rather
            // than the underlying bits directly, but that's okay.
            if self.set.contains(next) {
                return Some(next);
            }
        }
        None
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Dir {
    P,
    Z,
    N,
}

impl Dir {
    fn step(&self, start: u8) -> u8 {
        match self {
            Dir::P => start + 1,
            Dir::Z => start,
            Dir::N => start - 1,
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
    count: usize,

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
        info[0] = Info {
            pos: 0,
            dir: Dir::Z,
        };
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
        let (init_idx, init_pos, mut init_dir) = self
            .info
            .iter_mut()
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

        self.info[init_idx] = Info {
            pos: swap_pos,
            dir: init_dir,
        };
        self.info[swap_idx] = Info {
            pos: init_pos,
            dir: swap_dir,
        };
        self.data.swap(init_pos as usize, swap_pos as usize);
        self.count += 1;

        for (_, Info { pos, dir }) in self
            .info
            .iter_mut()
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
        let n = p - self.count - 1;
        (n, Some(n))
    }
}

pub struct Counter<T>(pub HashMap<T, usize>);

impl<T> Deref for Counter<T> {
    type Target = HashMap<T, usize>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Counter<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> Default for Counter<T> {
    fn default() -> Self {
        Self(HashMap::new())
    }
}

impl<A: Eq + Hash> FromIterator<A> for Counter<A> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut counts = Self::default();
        counts.extend(iter);
        counts
    }
}

impl<A: Eq + Hash> Extend<A> for Counter<A> {
    fn extend<T: IntoIterator<Item = A>>(&mut self, iter: T) {
        iter.into_iter()
            .for_each(|key| *self.0.entry(key).or_insert(0) += 1);
    }
}
