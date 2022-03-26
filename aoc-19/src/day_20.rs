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

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Portal {
    inner: Pos,
    outer: Pos,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Orient {
    Inner,
    Outer,
}

impl Portal {
    fn get(&self, pos: &Pos) -> Pos {
        if self.inner == *pos {
            self.outer
        } else {
            self.inner
        }
    }

    fn set(&mut self, pos: Pos, orient: Orient) {
        match orient {
            Orient::Inner => {
                assert!(self.inner == Pos::default());
                self.inner = pos
            }
            Orient::Outer => {
                assert!(self.outer == Pos::default());
                self.outer = pos
            }
        }
    }

    fn get_depth(&self, pos: &Pos, depth: usize) -> Option<(Pos, usize)> {
        if self.inner == *pos {
            Some((self.outer, depth + 1))
        } else if depth > 0 {
            Some((self.inner, depth - 1))
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Block {
    Tile,
    Wall,
}

impl Fro for DonutMaze {
    fn fro(input: &str) -> Self {
        // 2D grid of characters
        let chars = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut grid = HashMap::new();
        let mut portals = HashMap::<&'static str, Portal>::new();
        let mut labels = HashMap::new();
        let mut start = Pos::default();
        let mut end = Pos::default();

        // ############## |
        // ############## | r
        // ############## |
        // ###       |###
        // ###       |###
        // ###       |###
        // ###       |###
        // ###     ry|###
        // ###       |###
        // ###       |###
        // ###   rx  |###
        // ###-------+###
        // ##############
        // ##############
        // ##############
        //            ---
        //             r
        let r = chars
            .iter()
            .skip(2)
            .position(|line| {
                line[2..line.len() - 2]
                    .iter()
                    .any(|&c| c == ' ' || c.is_ascii_alphabetic())
            })
            .unwrap();

        let rx = chars
            .iter()
            .skip(2 + r)
            .next()
            .map(|line| {
                line[2..line.len() - 2]
                    .iter()
                    .filter(|&&c| c == ' ' || c.is_ascii_alphabetic())
                    .count()
            })
            .unwrap();

        let ry = chars.len() - 4 - r * 2;

        // Grid has 5 X-regions, 5 Y-regions:
        //
        // ```text
        //   01234
        // 0 _____
        // 1 _###_
        // 2 _#_#_
        // 3 _###_
        // 4 _____
        // ```
        let region = |d: usize, rd: usize| -> usize {
            if d < 2 {
                0
            } else if d < 2 + r {
                1
            } else if d < 2 + r + rd {
                2
            } else if d < 2 + r + rd + r {
                3
            } else {
                4
            }
        };

        let alpha = |x: usize, y: usize| -> Option<char> {
            match chars[y][x] {
                c if c.is_ascii_alphabetic() => Some(c),
                _ => None,
            }
        };
        for y in 2..chars.len() - 2 {
            for x in 2..chars[y].len() - 2 {
                let pos = Pos {
                    x: x as i64,
                    y: y as i64,
                };

                if chars[y][x] == '#' {
                    grid.insert(pos, Block::Wall);
                    continue;
                }

                if chars[y][x] != '.' {
                    continue;
                }

                grid.insert(pos, Block::Tile);

                if let Some((portal, orient)) = match (
                    region(x, rx),
                    region(y, ry),
                    alpha(x - 2, y),
                    alpha(x - 1, y),
                    alpha(x + 1, y),
                    alpha(x + 2, y),
                    alpha(x, y - 2),
                    alpha(x, y - 1),
                    alpha(x, y + 1),
                    alpha(x, y + 2),
                ) {
                    (1, _, Some(a), Some(b), None, None, None, None, None, None)
                    | (3, _, None, None, Some(a), Some(b), None, None, None, None)
                    | (_, 1, None, None, None, None, Some(a), Some(b), None, None)
                    | (_, 3, None, None, None, None, None, None, Some(a), Some(b)) => {
                        Some((format!("{}{}", a, b).leak(), Orient::Outer))
                    }
                    (3, _, Some(a), Some(b), None, None, None, None, None, None)
                    | (1, _, None, None, Some(a), Some(b), None, None, None, None)
                    | (_, 3, None, None, None, None, Some(a), Some(b), None, None)
                    | (_, 1, None, None, None, None, None, None, Some(a), Some(b)) => {
                        Some((format!("{}{}", a, b).leak(), Orient::Inner))
                    }
                    _ => None,
                } {
                    if portal == "AA" {
                        start = pos;
                    } else if portal == "ZZ" {
                        end = pos;
                    } else {
                        labels.insert(pos, portal);
                        portals.entry(portal).or_default().set(pos, orient);
                    }
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
        let max_x = self.grid.keys().map(|pos| pos.x).max().unwrap();
        let max_y = self.grid.keys().map(|pos| pos.y).max().unwrap();
        for y in 0..=max_y {
            for x in 0..=max_x {
                let pos = Pos {
                    x: x as i64,
                    y: y as i64,
                };
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
        self.plot();

        let mut queue = PriorityQueue::new();
        let mut seen = HashSet::new();

        println!("{:?}", self.start);

        queue.push(self.start, cmp::Reverse(0));

        while let Some((node, cmp::Reverse(dis))) = queue.pop() {
            if node == self.end {
                return dis;
            }

            seen.insert(node);

            for next in self
                .labels
                .get(&node)
                .into_iter()
                .map(|label| self.portals[label].get(&node))
                .chain(Dir::all().map(|dir| node.shift(dir)))
            {
                if seen.contains(&next) {
                    continue;
                }

                match self.grid.get(&next) {
                    Some(Block::Tile) => (),
                    Some(Block::Wall) | None => continue,
                }

                match queue.get_priority(&next) {
                    Some(cmp::Reverse(old)) if *old <= dis + 1 => {
                        continue;
                    }
                    Some(_) => {
                        queue.change_priority(&next, cmp::Reverse(dis + 1));
                        continue;
                    }
                    None => {
                        queue.push(next, cmp::Reverse(dis + 1));
                    }
                }
            }
        }

        unreachable!()
    }

    fn two(self) -> i64 {
        let mut queue = PriorityQueue::new();
        let mut seen = HashSet::new();

        queue.push((self.start, 0), cmp::Reverse(0));

        while let Some(((node, depth), cmp::Reverse(dis))) = queue.pop() {
            if node == self.end && depth == 0 {
                return dis;
            }

            seen.insert((node, depth));

            if let Some(label) = self.labels.get(&node) {
                if let Some((next, depth)) = self.portals[label].get_depth(&node, depth) {
                    if !seen.contains(&(next, depth)) {
                        queue.push((next, depth), cmp::Reverse(dis + 1));
                    }
                }
            }

            for next in Dir::all().map(|dir| (node.shift(dir), depth)) {
                if seen.contains(&next) {
                    continue;
                }

                match self.grid.get(&next.0) {
                    Some(Block::Tile) => (),
                    Some(Block::Wall) | None => continue,
                }

                match queue.get_priority(&next) {
                    Some(cmp::Reverse(old)) if *old <= dis + 1 => {
                        continue;
                    }
                    Some(_) => {
                        queue.change_priority(&next, cmp::Reverse(dis + 1));
                        continue;
                    }
                    None => {
                        queue.push(next, cmp::Reverse(dis + 1));
                    }
                }
            }
        }

        unreachable!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_1_23() {
        let maze = DonutMaze::fro(
            "         A           
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
             Z       ",
        );
        assert_eq!(maze.one(), 23)
    }

    #[test]
    fn test_1_58() {
        let maze = DonutMaze::fro(
            "                   A               
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
           U   P   P               ",
        );
        assert_eq!(maze.one(), 58)
    }

    #[test]
    fn test_2_396() {
        let maze = DonutMaze::fro(
            "             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     ",
        );
        assert_eq!(maze.two(), 396);
    }
}
