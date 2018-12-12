extern crate day_06;

use day_06::*;

const INPUT: &'static str = include_str!("input.txt");

fn main() {

    let mut grid = vec![vec![false; 1000]; 1000];
    
    for insn in INPUT.trim().split('\n').map(Instruction::parse) {
        for pos in insn.start.to(insn.end) {
            grid[pos.y][pos.x] = match insn.mode {
            | Mode::On => true,
            | Mode::Off => false,
            | Mode::Toggle => !grid[pos.y][pos.x],
            };
        }
    }

    let lit = grid.into_iter()
        .map(|row| row.into_iter().filter(|light| *light).count())
        .sum::<usize>();

    println!("{}", lit);
}
