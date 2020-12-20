use std::fmt;
use std::mem;

use indexmap::IndexMap;
use indexmap::IndexSet;

use aoc::*;

/// Image size in bits for part two.
///
/// Had to set to 24 (the example input) for debugging.
const IMAGE_SIZE: usize = 96;

#[derive(Clone, Debug)]
pub struct JurassicJigsaw(IndexMap<usize, Tile>);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Tile([u16; 10]);

impl Tile {
    fn top(&self) -> u16 {
        self.0[0]
    }

    fn right(&self) -> u16 {
        let mut right = 0u16;
        let right_mask = 0b1;
        for i in 0..self.0.len() {
            right |= (((self.0[i] & right_mask) > 0) as u16) << i;
        }
        right
    }

    fn bot(&self) -> u16 {
        self.0[9]
    }

    fn left(&self) -> u16 {
        let mut left = 0u16;
        let left_mask = 0b10_0000_0000;
        for i in 0..self.0.len() {
            left |= (((self.0[i] & left_mask) > 0) as u16) << i;
        }
        left
    }

    fn flip_mut(&mut self) {
        for row in &mut self.0 {
            *row = row.reverse_bits() >> 6;
        }
    }

    fn rotate_mut(&mut self) {
        let mut grid = [0u16; 10];
        for (i, row) in self.0.iter().enumerate() {
            let mut mask = 0b10_0000_0000;
            for col in 0..10 {
                grid[col] |= (((row & mask) > 0) as u16) << i;
                mask >>= 1;
            }
        }
        self.0 = grid;
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.0 {
            let mut mask = 0b10_0000_0000;
            while mask > 0 {
                if row & mask > 0 {
                    write!(fmt, "#")?;
                } else {
                    write!(fmt, ".")?;
                }
                mask >>= 1;
            }
            writeln!(fmt)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Image([u128; IMAGE_SIZE]);

impl Image {
    fn flip_mut(&mut self) {
        for line in &mut self.0 {
            *line = line.reverse_bits() >> (128 - IMAGE_SIZE);
        }
    }

    fn rotate_mut(&mut self) {
        let mut grid = [0u128; IMAGE_SIZE];
        for (i, row) in self.0.iter().enumerate() {
            let mut mask = 0b1 << (IMAGE_SIZE - 1);
            for col in 0..IMAGE_SIZE {
                grid[col] |= (((row & mask) > 0) as u128) << i;
                mask >>= 1;
            }
        }
        self.0 = grid;
    }
}

impl Fro for JurassicJigsaw {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split("\n\n")
            .map(|tile| {
                let mut iter = tile.trim().split('\n');
                let id = iter.give().trim().split(' ').nth(1).unwrap().trim_end_matches(':').to::<usize>();
                let mut grid = [0u16; 10];
                let index = &mut 0;
                for line in iter {
                    let mut set = 0b10_0000_0000;
                    let mut tile = 0;
                    for char in line.chars() {
                        match char {
                            '.' => (),
                            '#' => tile |= set,
                            _ => unreachable!(),
                        }
                        set >>= 1;
                    }
                    grid[mem::replace(index, *index + 1)] = tile;
                }

                (id, Tile(grid))
            })
            .collect::<IndexMap<_, _>>()
            .tap(Self)
    }
}

impl Solution for JurassicJigsaw {
    fn one(self) -> i64 {
        let len = (self.0.len() as f64).sqrt().round() as usize;
        let mut assignment = Vec::new();
        let mut tiles = self.0.iter().map(|(id, tile)| (*id, *tile)).collect::<IndexSet<_>>();

        if !recurse(&mut assignment, &mut tiles, len) {
            panic!()
        }

        (assignment[index(0, 0, len)].0
        * assignment[index(0, len - 1, len)].0
        * assignment[index(len - 1, 0, len)].0
        * assignment[index(len - 1, len - 1, len)].0) as i64
    }

    fn two(self) -> i64 {
        let len = (self.0.len() as f64).sqrt().round() as usize;
        let mut assignment = Vec::new();
        let mut tiles = self.0.iter().map(|(id, tile)| (*id, *tile)).collect::<IndexSet<_>>();

        if !recurse(&mut assignment, &mut tiles, len) {
            panic!()
        }

        // 12x12 grid with 8 bits per sub-grid = 96x96
        let mut image = Image([0; IMAGE_SIZE]);

        for (index, (_, tile)) in assignment.iter().enumerate() {
            let (r, c) = invert(index, len);
            for (dr, row) in tile.0.iter().skip(1).enumerate().take(8) {
                image.0[r * 8 + dr] |= (((row & 0b01_1111_1110) >> 1) as u128) << ((len - 1 - c) * 8);
            }
        }

        //                   #
        // #    ##    ##    ###
        //  #  #  #  #  #  #
        let monster = [
            0b0000_0000_0000_0000_0010u128,
            0b1000_0110_0001_1000_0111u128,
            0b0100_1001_0010_0100_1000u128,
        ];

        for transform in 0..8 {
            match transform {
                0 => (),
                1 | 2 | 3 | 5 | 6 | 7 => image.rotate_mut(),
                4 => {
                    image.rotate_mut();
                    image.flip_mut();
                }
                _ => unreachable!(),
            }

            let mut monsters = 0;

            for dy in 0..IMAGE_SIZE - 3 {
                for dx in 0..IMAGE_SIZE - 20 {
                    if monster
                        .iter()
                        .enumerate()
                        .all(|(y, row)| (image.0[dy + y] & (row << dx)) == (row << dx))
                    {
                        monsters += 1;
                    }
                }
            }

            if monsters > 0 {
                let total = image.0.iter().map(|line| line.count_ones()).sum::<u32>();
                let monster = monster.iter().map(|line| line.count_ones()).sum::<u32>();
                return (total - (monsters * monster)) as i64;
            }
        }

        unreachable!()
    }
}

fn recurse(
    assignment: &mut Vec<(usize, Tile)>,
    tiles: &mut IndexSet<(usize, Tile)>,
    len: usize,
) -> bool {
    if assignment.len() == len * len {
        return true;
    }

    for tile in tiles.iter().copied().collect::<Vec<_>>() {
        let mut modify = tile;
        for i in 0..8 {
            match i {
                0 => (),
                1 | 2 | 3 | 5 | 6 | 7 => modify.1.rotate_mut(),
                4 => {
                    modify.1.rotate_mut();
                    modify.1.flip_mut();
                }
                _ => unreachable!(),
            }

            assignment.push(modify);
            if satisfied(&*assignment, len) {
                tiles.remove(&tile);
                if recurse(assignment, tiles, len) {
                    return true;
                }
                tiles.insert(tile);
            }
            assignment.pop();
        }
    }

    false
}

fn satisfied(
    assignment: &[(usize, Tile)],
    len: usize,
) -> bool {
    let tile = assignment.last().unwrap();
    let (row, col) = invert(assignment.len() - 1, len);

    if col > 0 {
        if let Some(left) = assignment.get(index(row, col - 1, len)) {
            if left.1.right() != tile.1.left() {
                return false;
            }
        }
    }

    if row > 0 {
        if let Some(top) = assignment.get(index(row - 1, col, len)) {
            if top.1.bot() != tile.1.top() {
                return false;
            }
        }
    }

    true
}

fn index(row: usize, col: usize, len: usize) -> usize {
    row * len + col
}

fn invert(index: usize, len: usize) -> (usize, usize) {
    (index / len, index % len)
}
