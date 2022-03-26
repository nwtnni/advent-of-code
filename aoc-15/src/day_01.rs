use std::iter;
use std::mem;

use aoc::*;

#[derive(Clone, Debug)]
pub struct NotQuiteLisp(Vec<i64>);

impl Fro for NotQuiteLisp {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .chars()
            .map(|char| match char {
                '(' => 1,
                ')' => -1,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for NotQuiteLisp {
    fn one(self) -> i64 {
        self.0.into_iter().sum()
    }

    fn two(self) -> i64 {
        self.0
            .into_iter()
            .chain(iter::once(0))
            .scan(0, |floor, next| Some(mem::replace(floor, *floor + next)))
            .position(|floor| floor == -1)
            .unwrap() as i64
    }
}

#[cfg(test)]
mod tests {

    use aoc::Fro as _;
    use aoc::Solution as _;

    macro_rules! impl_test {
        ($name:ident, $method:ident, $input:expr, $value:expr) => {
            #[test]
            fn $name() {
                assert_eq!(super::NotQuiteLisp::fro($input).$method(), $value)
            }
        };
    }

    impl_test!(part_one_0, one, "(())", 0);
    impl_test!(part_one_1, one, "()()", 0);
    impl_test!(part_one_2, one, "(((", 3);
    impl_test!(part_one_3, one, "(()(()(", 3);
    impl_test!(part_one_4, one, "))(((((", 3);
    impl_test!(part_one_5, one, "())", -1);
    impl_test!(part_one_6, one, "))(", -1);
    impl_test!(part_one_7, one, ")))", -3);
    impl_test!(part_one_8, one, ")())())", -3);

    impl_test!(part_two_0, two, ")", 1);
    impl_test!(part_two_1, two, "()())", 5);
}
