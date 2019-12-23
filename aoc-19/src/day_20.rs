use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str;

use aoc::*;
use priority_queue::PriorityQueue;

pub struct DonutMaze {
    grid: HashMap<Pos, Block>,
    labels: HashMap<Pos, &'static str>,
    portals: HashMap<&'static str, Portal>,
    start: Pos,
    end: Pos,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Portal(Pos, Pos);

impl Portal {
    fn new(pos: Pos) -> Self {
        Portal(pos, Pos::default())
    }

    fn set(&mut self, pos: Pos) {
        self.1 = pos;
    }

    fn get(&self, pos: &Pos) -> Pos {
        let Portal(a, b) = self;
        if a == pos { *b } else { *a }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Block {
    Tile,
    Wall,
}

impl Fro for DonutMaze {
    fn fro(input: &str) -> Self {
        let chars = input.lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut grid = HashMap::new();
        let mut portals = HashMap::new();
        let mut labels = HashMap::new();
        let mut start = Pos::default();
        let mut end = Pos::default();

        let portal = |i: usize, j: usize| -> Option<&'static str> {
            if chars[i + 1][j].is_ascii_alphabetic() && chars[i + 2][j].is_ascii_alphabetic() {
                Some(format!("{}{}", chars[i + 1][j], chars[i + 2][j]).leak())
            } else if chars[i - 2][j].is_ascii_alphabetic() && chars[i - 1][j].is_ascii_alphabetic() {
                Some(format!("{}{}", chars[i - 2][j], chars[i - 1][j]).leak())
            } else if chars[i][j + 1].is_ascii_alphabetic() && chars[i][j + 2].is_ascii_alphabetic() {
                Some(format!("{}{}", chars[i][j + 1], chars[i][j + 2]).leak())
            } else if chars[i][j - 2].is_ascii_alphabetic() && chars[i][j - 1].is_ascii_alphabetic() {
                Some(format!("{}{}", chars[i][j - 2], chars[i][j - 1]).leak())
            } else {
                None
            }
        };

        for (y, (i, line)) in chars.iter()
            .enumerate()
            .filter(|(_, line)| line.contains(&'#'))
            .enumerate()
        {

            //  ___###___###___
            //   0  1  2  3  4
            let mut region = 0;
            let mut x = -1;

            for (j, c) in line.iter().enumerate() {

                match (c, region) {
                | ('#', 0) | ('.', 0) => {
                    region = 1;
                    x += 1;
                }
                | (_, 0) => continue,
                | ('#', 1) | ('.', 1) => {
                    x += 1;
                }
                | (_, 1) => {
                    region = 2;
                    x += 1;
                }
                | ('#', 2) | ('.', 2) => {
                    region = 3;
                    x += 1;
                }
                | (_, 2) => {
                    x += 1;
                }
                | ('#', 3) | ('.', 3) => {
                    x += 1;
                }
                | (_, 3) => {
                    region = 4;
                }
                | _ => break,
                }

                let pos = Pos { x, y: y as i64 };

                match c {
                | '#' => { grid.insert(pos, Block::Wall); },
                | '.' => {
                    grid.insert(pos, Block::Tile);
                    if let Some(label) = portal(i, j) {
                        if label == "AA" {
                            start = pos;
                            continue;
                        }

                        if label == "ZZ" {
                            end = pos;
                            continue;
                        }

                        labels.insert(pos, label);
                        match portals.get_mut(label) {
                        | None => { portals.insert(label, Portal::new(pos)); },
                        | Some(portal) => portal.set(pos),
                        }
                    }
                }
                | _ => (),
                }
            }
        }

        DonutMaze {
            grid,
            labels,
            portals,
            start,
            end,
        }
    }
}

impl DonutMaze {
    #[allow(unused)]
    fn plot(&self) {
        let max_x = self.grid.keys()
            .map(|pos| pos.x)
            .max()
            .unwrap();
        let max_y = self.grid.keys()
            .map(|pos| pos.y)
            .max()
            .unwrap();
        for y in 0..max_y {
            for x in 0..max_x {
                let pos = Pos { x: x as i64, y: y as i64 };
                if self.labels.contains_key(&pos) {
                    print!("*");
                } else if let Some(Block::Tile) = self.grid.get(&pos) {
                    print!(".");
                } else if let Some(Block::Wall) = self.grid.get(&pos) {
                    print!("█");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
    }
}

impl Solution for DonutMaze {
    fn one(self) -> i64 {

        let mut queue = PriorityQueue::new();
        let mut seen = HashSet::new();

        queue.push(self.start, cmp::Reverse(0));

        while let Some((node, cmp::Reverse(dis))) = queue.pop() {

            if node == self.end {
                return dis;
            }

            seen.insert(node);

            for next in self.labels.get(&node)
                .into_iter()
                .map(|label| self.portals[label].get(&node))
                .chain(Dir::all().map(|dir| node.shift(dir)))
            {
                if seen.contains(&next) {
                    continue;
                }

                match self.grid.get(&next) {
                | Some(Block::Tile) => (),
                | Some(Block::Wall)
                | None => continue,
                }

                match queue.get_priority(&next) {
                | Some(cmp::Reverse(old)) if *old <= dis + 1 => {
                    continue;
                }
                | Some(_) => {
                    queue.change_priority(&next, cmp::Reverse(dis + 1));
                    continue;
                }
                | None => {
                    queue.push(next, cmp::Reverse(dis + 1));
                }
                }
            }
        }

        unreachable!()
    }

    fn two(self) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1_23() {
        let maze = DonutMaze::fro("
                   A           
                   A           
            #######.#########  
            #######.........#  
            #######.#######.#  
            #######.#######.#  
            #######.#######.#  
            #####  B    ###.#  
          BC...##  C    ###.#  
            ##.##       ###.#  
            ##...DE  F  ###.#  
            #####    G  ###.#  
            #########.#####.#  
          DE..#######...###.#  
            #.#########.###.#  
          FG..#########.....#  
            ###########.#####  
                       Z       
                       Z       
        ");
        assert_eq!(maze.one(), 23)
    }

    #[test]
    fn test_1_58() {
        let maze = DonutMaze::fro("
                               A               
                               A               
              #################.#############  
              #.#...#...................#.#.#  
              #.#.#.###.###.###.#########.#.#  
              #.#.#.......#...#.....#.#.#...#  
              #.#########.###.#####.#.#.###.#  
              #.............#.#.....#.......#  
              ###.###########.###.#####.#.#.#  
              #.....#        A   C    #.#.#.#  
              #######        S   P    #####.#  
              #.#...#                 #......VT
              #.#.#.#                 #.#####  
              #...#.#               YN....#.#  
              #.###.#                 #####.#  
            DI....#.#                 #.....#  
              #####.#                 #.###.#  
            ZZ......#               QG....#..AS
              ###.###                 #######  
            JO..#.#.#                 #.....#  
              #.#.#.#                 ###.#.#  
              #...#..DI             BU....#..LF
              #####.#                 #.#####  
            YN......#               VT..#....QG
              #.###.#                 #.###.#  
              #.#...#                 #.....#  
              ###.###    J L     J    #.#.###  
              #.....#    O F     P    #.#...#  
              #.###.#####.#.#####.#####.###.#  
              #...#.#.#...#.....#.....#.#...#  
              #.#####.###.###.#.#.#########.#  
              #...#.#.....#...#.#.#.#.....#.#  
              #.###.#####.###.###.#.#.#######  
              #.#.........#...#.............#  
              #########.###.###.#############  
                       B   J   C               
                       U   P   P               
        ");
        assert_eq!(maze.one(), 58)
    }

}
