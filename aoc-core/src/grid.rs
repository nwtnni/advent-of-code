use std::fmt;
use std::iter;
use std::ops;
use std::slice;
use std::str;

use crate::Tap as _;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Grid<T>(Vec<Vec<T>>);

impl<T> Grid<T> {
    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn width(&self) -> usize {
        self.0[0].len()
    }

    pub fn around(&self, i: usize, j: usize) -> impl Iterator<Item = &T> {
        self.adjacent(i, j).chain(self.diagonal(i, j))
    }

    pub fn adjacent(&self, i: usize, j: usize) -> impl Iterator<Item = &T> {
        adjacent(self.width(), self.height(), i, j).map(move |(i, j)| &self[i][j])
    }

    pub fn diagonal(&self, i: usize, j: usize) -> impl Iterator<Item = &T> {
        diagonal(self.width(), self.height(), i, j).map(move |(i, j)| &self[i][j])
    }

    pub fn rows(&self) -> slice::Iter<Vec<T>> {
        self.0.iter()
    }

    pub fn rows_mut(&self) -> slice::Iter<Vec<T>> {
        self.0.iter()
    }

    pub fn iter(&self) -> iter::Flatten<slice::Iter<Vec<T>>> {
        self.into_iter()
    }

    pub fn iter_mut(&mut self) -> iter::Flatten<slice::IterMut<Vec<T>>> {
        self.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type IntoIter = iter::Flatten<slice::Iter<'a, Vec<T>>>;
    type Item = &'a T;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter().flatten()
    }
}

impl<'a, T> IntoIterator for &'a mut Grid<T> {
    type IntoIter = iter::Flatten<slice::IterMut<'a, Vec<T>>>;
    type Item = &'a mut T;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut().flatten()
    }
}

pub fn around(w: usize, h: usize, i: usize, j: usize) -> impl Iterator<Item = (usize, usize)> {
    adjacent(w, h, i, j).chain(diagonal(w, h, i, j))
}

pub fn adjacent(w: usize, h: usize, i: usize, j: usize) -> impl Iterator<Item = (usize, usize)> {
    iter::empty()
        .chain(if i > 0 { Some((i - 1, j)) } else { None })
        .chain(if i < h - 1 { Some((i + 1, j)) } else { None })
        .chain(if j > 0 { Some((i, j - 1)) } else { None })
        .chain(if j < w - 1 { Some((i , j + 1)) } else { None })
}

pub fn diagonal(w: usize, h: usize, i: usize, j: usize) -> impl Iterator<Item = (usize, usize)> {
    iter::empty()
        .chain(if i > 0 && j > 0 { Some((i - 1, j - 1)) } else { None })
        .chain(if i < h - 1 && j < w - 1 { Some((i + 1, j + 1)) } else { None })
        .chain(if i < h - 1 && j > 0 { Some((i + 1, j - 1)) } else { None })
        .chain(if i > 0 && j < w - 1 { Some((i - 1 , j + 1)) } else { None })
}

impl str::FromStr for Grid<i64> {
    type Err = crate::Error;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        string
            .trim()
            .split('\n')
            .map(|line| line.chars().map(|char| (char as u8 - b'0') as i64).collect())
            .collect::<Vec<_>>()
            .tap(Self)
            .tap(Ok)
    }
}

impl str::FromStr for Grid<char> {
    type Err = crate::Error;
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        string
            .trim()
            .split('\n')
            .map(|line| line.chars().collect())
            .collect::<Vec<_>>()
            .tap(Self)
            .tap(Ok)
    }
}

impl<T> ops::Deref for Grid<T> {
    type Target = Vec<Vec<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> ops::DerefMut for Grid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> ops::Index<usize> for Grid<T> {
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        self.0.index(index)
    }
}

impl<T> ops::IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

impl<T> ops::Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        self.0.index(i).index(j)
    }
}

impl<T> ops::IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        self.0.index_mut(i).index_mut(j)
    }
}

impl<T> ops::Index<[usize; 2]> for Grid<T> {
    type Output = T;
    fn index(&self, [i, j]: [usize; 2]) -> &Self::Output {
        self.0.index(i).index(j)
    }
}

impl<T> ops::IndexMut<[usize; 2]> for Grid<T> {
    fn index_mut(&mut self, [i, j]: [usize; 2]) -> &mut Self::Output {
        self.0.index_mut(i).index_mut(j)
    }
}

impl<T: fmt::Debug> fmt::Debug for Grid<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.0 {
            for col in row {
                write!(fmt, "{:?}", col)?;
            }
            writeln!(fmt)?;
        }
        Ok(())
    }
}
