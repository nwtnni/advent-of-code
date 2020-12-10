use aoc::*;

#[derive(Clone, Debug)]
pub struct PasswordPhilosophy(Vec<Policy>);

#[derive(Clone, Debug)]
struct Policy {
    lo: usize,
    hi: usize,
    letter: char,
    password: String,
}

impl Fro for PasswordPhilosophy {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let mut iter = line.trim().split_whitespace();

                let mut lo_hi = iter.give().split('-');
                let lo = lo_hi.give().to::<usize>();
                let hi = lo_hi.give().to::<usize>();

                let letter = iter.give().chars().next().unwrap();

                let password = iter.give().to_owned();

                Policy {
                    lo,
                    hi,
                    letter,
                    password,
                }
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for PasswordPhilosophy {
    fn one(self) -> i64 {
        self.0
            .into_iter()
            .filter(|policy| {
                let count = policy.password.chars().filter(|c| *c == policy.letter).count();
                count >= policy.lo && count <= policy.hi
            })
            .count()
            as i64
    }

    fn two(self) -> i64 {
        self.0
            .into_iter()
            .filter(|policy| {
                let lo = policy.password.chars().nth(policy.lo - 1).unwrap() == policy.letter;
                let hi = policy.password.chars().nth(policy.hi - 1).unwrap() == policy.letter;
                lo ^ hi
            })
            .count()
            as i64
    }
}

#[cfg(test)]
mod tests {

    use aoc::Fro as _;
    use aoc::Solution as _;

    static EXAMPLE: &str = "\
        1-3 a: abcde\n\
        1-3 b: cdefg\n\
        2-9 c: ccccccccc\
    ";

    #[test]
    fn part_one() {
        assert_eq!(super::PasswordPhilosophy::fro(EXAMPLE).one(), 2);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::PasswordPhilosophy::fro(EXAMPLE).two(), 1);
    }
}
