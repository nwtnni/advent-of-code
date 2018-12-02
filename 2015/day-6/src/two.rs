extern crate day_6;

use std::isize;

use day_6::*;

const INPUT: &'static str = include_str!("input.txt");

fn main() {

    let mut grid = vec![vec![0isize; 1000]; 1000];
    
    for insn in INPUT.trim().split('\n').map(Instruction::parse) {
        for pos in insn.start.to(insn.end) {
            grid[pos.y][pos.x] = match insn.mode {
            | Mode::On => grid[pos.y][pos.x] + 1,
            | Mode::Off => isize::max(0, grid[pos.y][pos.x] - 1),
            | Mode::Toggle => grid[pos.y][pos.x] + 2,
            };
        }
    }

    let lit = grid.into_iter()
        .map(|row| row.into_iter().sum::<isize>())
        .sum::<isize>();

    println!("{}", lit);
}
