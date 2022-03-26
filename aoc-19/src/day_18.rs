use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use aoc::*;
use priority_queue::PriorityQueue;

#[derive(Clone, Debug)]
pub struct ManyWorldsInterpretation {
    entrances: Vec<Pos>,
    grid: HashMap<Pos, Block>,
}

#[derive(Copy, Clone, Debug)]
pub enum Block {
    Wall,
    Key(char),
    Door(char),
}

impl Block {
    fn is_key(&self) -> bool {
        match self {
            Block::Key(_) => true,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Collapsed {
    Start(u8),
    Key(char),
    Door(char),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeySet(u32);

impl KeySet {
    fn empty() -> Self {
        KeySet(0)
    }

    fn insert(&self, k: char) -> Self {
        KeySet(self.0 | (0b1 << (k as u8 - b'a')))
    }

    fn contains(&self, k: char) -> bool {
        self.0 & (0b1 << (k as u8 - b'a')) > 0
    }
}

impl Fro for ManyWorldsInterpretation {
    fn fro(input: &str) -> Self {
        let mut grid = HashMap::new();
        let mut entrances = Vec::new();
        for (y, line) in input.trim().split_whitespace().enumerate() {
            for (x, char) in line.trim().chars().enumerate() {
                let pos = Pos {
                    x: x as i64,
                    y: y as i64,
                };
                match char {
                    '#' => {
                        grid.insert(pos, Block::Wall);
                    }
                    '@' => entrances.push(pos),
                    c if c.is_ascii_lowercase() => {
                        grid.insert(pos, Block::Key(c));
                    }
                    c if c.is_ascii_uppercase() => {
                        grid.insert(pos, Block::Door(c.to_ascii_lowercase()));
                    }
                    _ => (),
                }
            }
        }
        ManyWorldsInterpretation { entrances, grid }
    }
}

impl ManyWorldsInterpretation {
    fn bfs(&self, start: Pos, end: Pos) -> Option<i64> {
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();

        queue.push_back((start, 0));

        while let Some((node, dis)) = queue.pop_front() {
            if node == end {
                return Some(dis);
            }

            seen.insert(node);

            for dir in Dir::all() {
                let next = node.shift(dir);
                if seen.contains(&next) {
                    continue;
                }
                match self.grid.get(&next) {
                    Some(Block::Wall) => continue,
                    Some(Block::Door(_)) if next != end => continue,
                    Some(Block::Door(_)) => queue.push_back((next, dis + 1)),
                    _ => queue.push_back((next, dis + 1)),
                }
            }
        }

        None
    }

    fn collapse(&self) -> HashMap<Collapsed, Vec<(Collapsed, i64)>> {
        let mut all = self
            .grid
            .iter()
            .filter_map(|(&pos, &block)| match block {
                Block::Key(k) => Some((pos, Collapsed::Key(k))),
                Block::Door(d) => Some((pos, Collapsed::Door(d))),
                _ => None,
            })
            .collect::<Vec<_>>();

        for (i, entrance) in self.entrances.iter().copied().enumerate() {
            all.push((entrance, Collapsed::Start(i as u8)));
        }

        let mut collapsed = HashMap::new();

        for i in 0..all.len() {
            for j in i + 1..all.len() {
                let (pos_a, a) = all[i];
                let (pos_b, b) = all[j];
                if let Some(dis) = self.bfs(pos_a, pos_b) {
                    collapsed.entry(a).or_insert_with(Vec::new).push((b, dis));
                    collapsed.entry(b).or_insert_with(Vec::new).push((a, dis));
                }
            }
        }

        collapsed
    }

    /// ```text
    /// ...    @#@
    /// .@. => ###
    /// ...    @#@
    /// ```
    fn split(&mut self) {
        let p = self.entrances[0];
        self.grid.insert(p, Block::Wall);
        self.grid.insert(Pos { x: p.x + 1, y: p.y }, Block::Wall);
        self.grid.insert(Pos { x: p.x - 1, y: p.y }, Block::Wall);
        self.grid.insert(Pos { x: p.x, y: p.y + 1 }, Block::Wall);
        self.grid.insert(Pos { x: p.x, y: p.y - 1 }, Block::Wall);
        self.entrances.clear();
        self.entrances.push(Pos {
            x: p.x - 1,
            y: p.y - 1,
        });
        self.entrances.push(Pos {
            x: p.x + 1,
            y: p.y - 1,
        });
        self.entrances.push(Pos {
            x: p.x - 1,
            y: p.y + 1,
        });
        self.entrances.push(Pos {
            x: p.x + 1,
            y: p.y + 1,
        });
    }
}

impl Solution for ManyWorldsInterpretation {
    fn one(self) -> i64 {
        let count = self.grid.values().filter(|b| b.is_key()).count();

        let full = (0b1 << count) - 1;

        let collapsed = self.collapse();
        let mut queue = PriorityQueue::new();
        let mut seen = HashSet::new();

        queue.push((Collapsed::Start(0), KeySet::empty()), cmp::Reverse(0));

        while let Some(((node, set), cmp::Reverse(dis))) = queue.pop() {
            if set.0 == full {
                return dis;
            }

            seen.insert((node, set));

            for &(next, delta) in &collapsed[&node] {
                let next_set = match next {
                    Collapsed::Start(_) => continue,
                    Collapsed::Door(d) if !set.contains(d) => continue,
                    Collapsed::Door(_) => set,
                    Collapsed::Key(k) => set.insert(k),
                };

                if seen.contains(&(next, next_set)) {
                    continue;
                }

                match queue.get_priority(&(next, next_set)) {
                    Some(cmp::Reverse(old)) if dis + delta >= *old => (),
                    Some(_) => {
                        queue.change_priority(&(next, next_set), cmp::Reverse(dis + delta));
                    }
                    None => {
                        queue.push((next, next_set), cmp::Reverse(dis + delta));
                    }
                }
            }
        }

        unreachable!()
    }

    // Same as P1 except each node is [Collapsed; 4] instead of Collapsed,
    // so we track the position of four robots at a time.
    fn two(mut self) -> i64 {
        let count = self.grid.values().filter(|b| b.is_key()).count();

        let full = (0b1 << count) - 1;

        self.split();
        let collapsed = self.collapse();

        let mut queue = PriorityQueue::new();
        let mut seen = HashSet::new();
        let mut start = [Collapsed::Start(0); 4];
        for i in 0..4 {
            start[i] = Collapsed::Start(i as u8);
        }
        queue.push((start, KeySet::empty()), cmp::Reverse(0));

        while let Some(((pos, set), cmp::Reverse(dis))) = queue.pop() {
            if set.0 == full {
                return dis;
            }

            seen.insert((pos, set));

            for i in 0..4 {
                let mut next_pos = pos.clone();
                for &(next, delta) in &collapsed[&pos[i]] {
                    next_pos[i] = next;
                    let next_set = match next {
                        Collapsed::Start(_) => continue,
                        Collapsed::Door(d) if !set.contains(d) => continue,
                        Collapsed::Door(_) => set,
                        Collapsed::Key(k) => set.insert(k),
                    };

                    if seen.contains(&(next_pos, next_set)) {
                        continue;
                    }

                    match queue.get_priority(&(next_pos, next_set)) {
                        Some(cmp::Reverse(old)) if dis + delta >= *old => (),
                        Some(_) => {
                            queue.change_priority(&(next_pos, next_set), cmp::Reverse(dis + delta));
                        }
                        None => {
                            queue.push((next_pos, next_set), cmp::Reverse(dis + delta));
                        }
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
    type M = ManyWorldsInterpretation;

    #[test]
    fn test_1_132() {
        let grid = M::fro(
            "
            ########################
            #...............b.C.D.f#
            #.######################
            #.....@.a.B.c.d.A.e.F.g#
            ########################
         ",
        );

        assert_eq!(grid.one(), 132);
    }

    #[test]
    fn test_1_136() {
        let grid = M::fro(
            "
            #################
            #i.G..c...e..H.p#
            ########.########
            #j.A..b...f..D.o#
            ########@########
            #k.E..a...g..B.n#
            ########.########
            #l.F..d...h..C.m#
            #################
         ",
        );

        assert_eq!(grid.one(), 136);
    }

    #[test]
    fn test_1_81() {
        let grid = M::fro(
            "
            ########################
            #@..............ac.GI.b#
            ###d#e#f################
            ###A#B#C################
            ###g#h#i################
            ########################
         ",
        );

        assert_eq!(grid.one(), 81);
    }
}
