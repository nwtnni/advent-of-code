use std::collections::HashSet as Set;
use std::mem;
use std::str;

pub struct PerfectlySphericalHousesInAVacuum(Vec<aoc::Dir>);

impl str::FromStr for PerfectlySphericalHousesInAVacuum {
    type Err = aoc::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars() 
                .map(parse_dir)
                .collect()
        ))
    }
}

fn parse_dir(c: char) -> aoc::Dir {
    match c {
    | '^' => aoc::Dir::N,
    | 'v' => aoc::Dir::S,
    | '>' => aoc::Dir::E,
    | '<' => aoc::Dir::W,
    | _ => unreachable!(),
    }
}

impl aoc::Solution for PerfectlySphericalHousesInAVacuum {
    fn one(&mut self) -> i32 {
        use aoc::Pos;
        self.0.iter()
            .chain(&[aoc::Dir::N]) // Dummy
            .scan(Pos::default(), |pos, dir| Some(mem::replace(pos, pos.shift(*dir))))
            .collect::<Set<_>>()
            .len() as i32
    }

    fn two(&mut self) -> i32 {
        use aoc::Pos;
        self.0.iter()
            .chain(&[aoc::Dir::N, aoc::Dir::N]) // Dummy
            .scan((Pos::default(), Pos::default(), false), |(human, robot, flip), dir| {
                if mem::replace(flip, !*flip) {
                    Some(mem::replace(human, human.shift(*dir)))
                } else {
                    Some(mem::replace(robot, robot.shift(*dir)))
                }
            })
            .collect::<Set<_>>()
            .len() as i32
    }
}

#[cfg(test)]
mod tests {

    use aoc::Solution;

    type PSHV = super::PerfectlySphericalHousesInAVacuum;

    #[test]
    fn test_1_0() {
        assert_eq!(">".parse::<PSHV>().unwrap().one(), 2)
    }

    #[test]
    fn test_1_1() {
        assert_eq!("^>v<".parse::<PSHV>().unwrap().one(), 4)
    }

    #[test]
    fn test_1_2() {
        assert_eq!("^v^v^v^v^v".parse::<PSHV>().unwrap().one(), 2)
    }

    #[test]
    fn test_2_0() {
        assert_eq!("^v".parse::<PSHV>().unwrap().two(), 3)
    }

    #[test]
    fn test_2_1() {
        assert_eq!("^>v<".parse::<PSHV>().unwrap().two(), 3)
    }

    #[test]
    fn test_2_2() {
        assert_eq!("^v^v^v^v^v".parse::<PSHV>().unwrap().two(), 11)
    }
}
