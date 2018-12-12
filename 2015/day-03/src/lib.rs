#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Dir {
    N, S, E, W
}

impl Dir {
    pub fn parse(dir: char) -> Self {
        match dir {
        | '^' => Dir::N,
        | 'v' => Dir::S,
        | '>' => Dir::E,
        | '<' => Dir::W,
        | dir => panic!("[INPUT ERROR]: invalid direction {}", dir),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pos {
    x: isize,
    y: isize,
}

impl Pos {
    pub fn shift(&mut self, dir: Dir) {
        match dir {
        | Dir::N => self.y += 1, 
        | Dir::S => self.y -= 1,
        | Dir::E => self.x += 1,
        | Dir::W => self.x -= 1,
        }
    }
}
