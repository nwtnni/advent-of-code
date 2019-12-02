use std::str;

pub struct NotQuiteLisp(String);

impl str::FromStr for NotQuiteLisp {
    type Err = aoc::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(NotQuiteLisp(s.to_owned()))
    }
}

fn delta(c: char) -> i32 {
    match c {
    | '(' =>  1,
    | ')' => -1,
    | _   =>  0,
    }
}

impl aoc::Solution for NotQuiteLisp {
    fn one(self) -> i32 {
        self.0.chars()
            .map(delta)
            .sum()
    }

    fn two(self) -> i32 {
        self.0.chars()
            .map(delta)
            .scan(0, |this, next| {
                if *this == -1 { None } else { Some(*this += next) }
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {

    use aoc::Solution;

    macro_rules! impl_test {
        ($name:ident, $method:ident, $input:expr, $value:expr) => {
            #[test]
            fn $name() {
                assert_eq!(super::NotQuiteLisp(String::from($input)).$method(), $value)
            }
        }
    }

    impl_test!(test_1_0, one, "(())", 0);
    impl_test!(test_1_1, one, "()()", 0);
    impl_test!(test_1_2, one, "(((", 3);
    impl_test!(test_1_3, one, "(()(()(", 3);
    impl_test!(test_1_4, one, "))(((((", 3);
    impl_test!(test_1_5, one, "())", -1);
    impl_test!(test_1_6, one, "))(", -1);
    impl_test!(test_1_7, one, ")))", -3);
    impl_test!(test_1_8, one, ")())())", -3);

    impl_test!(test_2_0, two, ")", 1);
    impl_test!(test_2_1, two, "()())", 5);
}
