use aoc::*;

#[derive(Clone, Debug)]
pub struct CustomCustoms(Vec<Vec<AsciiSet>>);

impl Fro for CustomCustoms {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split("\n\n")
            .map(|group| group.split_whitespace().map(AsciiSet::from).collect())
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for CustomCustoms {
    fn one(self) -> i64 {
        self.0
            .into_iter()
            .map(|group| group.into_iter().fold(AsciiSet::none(), AsciiSet::or).len())
            .sum::<usize>() as i64
    }

    fn two(self) -> i64 {
        self.0
            .into_iter()
            .map(|group| {
                group
                    .into_iter()
                    .fold(AsciiSet::from(LOWERS), AsciiSet::and)
                    .len()
            })
            .sum::<usize>() as i64
    }
}

#[cfg(test)]
mod tests {

    use aoc::Fro as _;
    use aoc::Solution as _;

    static EXAMPLE: &str = "
        abc\n\
        \n\
        a\n\
        b\n\
        c\n\
        \n\
        ab\n\
        ac\n\
        \n\
        a\n\
        a\n\
        a\n\
        a\n\
        \n\
        b\n\
    ";

    #[test]
    fn part_one() {
        assert_eq!(super::CustomCustoms::fro(EXAMPLE).one(), 11);
    }

    #[test]
    fn part_two() {
        assert_eq!(super::CustomCustoms::fro(EXAMPLE).two(), 6);
    }
}
