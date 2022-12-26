use std::fmt;

use aoc::*;

#[derive(Clone, Debug)]
pub struct FullOfHotAir(Vec<Vec<Digit>>);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Digit {
    Two,
    One,
    Zero,
    Minus,
    DoubleMinus,
}

impl fmt::Display for Digit {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let digit = match self {
            Digit::Two => '2',
            Digit::One => '1',
            Digit::Zero => '0',
            Digit::Minus => '-',
            Digit::DoubleMinus => '=',
        };

        write!(fmt, "{}", digit)
    }
}

impl From<Digit> for i64 {
    fn from(digit: Digit) -> Self {
        match digit {
            Digit::Two => 2,
            Digit::One => 1,
            Digit::Zero => 0,
            Digit::Minus => -1,
            Digit::DoubleMinus => -2,
        }
    }
}

impl Fro for FullOfHotAir {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                line.chars()
                    .map(|char| match char {
                        '2' => Digit::Two,
                        '1' => Digit::One,
                        '0' => Digit::Zero,
                        '-' => Digit::Minus,
                        '=' => Digit::DoubleMinus,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for FullOfHotAir {
    fn one(self) -> i64 {
        self.0
            .into_iter()
            .map(|digits| forward(&digits))
            .sum::<i64>()
            .tap(backward)
            .into_iter()
            .for_each(|digit| {
                print!("{}", digit);
            });

        println!();
        panic!();
    }

    fn two(self) -> i64 {
        unreachable!()
    }
}

fn forward(digits: &[Digit]) -> i64 {
    digits
        .iter()
        .copied()
        .map(i64::from)
        .rev()
        .enumerate()
        .map(|(place, digit)| 5i64.pow(place as u32) * digit)
        .sum()
}

fn backward(mut value: i64) -> Vec<Digit> {
    let mut digits = Vec::new();
    let mut overflow = false;

    while value > 0 {
        let (next, digit) = match (value + overflow as i64) % 5 {
            4 => (true, Digit::Minus),
            3 => (true, Digit::DoubleMinus),
            2 => (false, Digit::Two),
            1 => (false, Digit::One),
            0 => (overflow, Digit::Zero),
            _ => unreachable!(),
        };

        overflow = next;
        digits.push(digit);
        value /= 5;
    }

    if overflow {
        digits.push(Digit::One);
    }

    digits.reverse();
    digits
}

#[cfg(test)]
mod forward {
    use super::forward;
    use super::Digit;

    #[test]
    fn test_1() {
        assert_eq!(forward(&[Digit::One]), 1)
    }

    #[test]
    fn test_2() {
        assert_eq!(forward(&[Digit::Two]), 2)
    }

    #[test]
    fn test_3() {
        assert_eq!(forward(&[Digit::One, Digit::DoubleMinus]), 3)
    }

    #[test]
    fn test_4() {
        assert_eq!(forward(&[Digit::One, Digit::Minus]), 4)
    }

    #[test]
    fn test_5() {
        assert_eq!(forward(&[Digit::One, Digit::Zero]), 5)
    }

    #[test]
    fn test_6() {
        assert_eq!(forward(&[Digit::One, Digit::One]), 6)
    }

    #[test]
    fn test_7() {
        assert_eq!(forward(&[Digit::One, Digit::Two]), 7)
    }

    #[test]
    fn test_8() {
        assert_eq!(forward(&[Digit::Two, Digit::DoubleMinus]), 8)
    }

    #[test]
    fn test_9() {
        assert_eq!(forward(&[Digit::Two, Digit::Minus]), 9)
    }

    #[test]
    fn test_10() {
        assert_eq!(forward(&[Digit::Two, Digit::Zero]), 10)
    }

    #[test]
    fn test_15() {
        assert_eq!(forward(&[Digit::One, Digit::DoubleMinus, Digit::Zero]), 15)
    }

    #[test]
    fn test_20() {
        assert_eq!(forward(&[Digit::One, Digit::Minus, Digit::Zero]), 20)
    }

    #[test]
    fn test_2022() {
        assert_eq!(
            forward(&[
                Digit::One,
                Digit::DoubleMinus,
                Digit::One,
                Digit::One,
                Digit::Minus,
                Digit::Two,
            ]),
            2022,
        )
    }
}

#[cfg(test)]
mod backward {
    use super::backward;
    use super::Digit;

    #[test]
    fn test_1() {
        assert_eq!(backward(1), vec![Digit::One])
    }

    #[test]
    fn test_2() {
        assert_eq!(backward(2), vec![Digit::Two])
    }

    #[test]
    fn test_3() {
        assert_eq!(backward(3), vec![Digit::One, Digit::DoubleMinus])
    }

    #[test]
    fn test_4() {
        assert_eq!(backward(4), vec![Digit::One, Digit::Minus])
    }

    #[test]
    fn test_5() {
        assert_eq!(backward(5), vec![Digit::One, Digit::Zero])
    }

    #[test]
    fn test_6() {
        assert_eq!(backward(6), vec![Digit::One, Digit::One])
    }

    #[test]
    fn test_7() {
        assert_eq!(backward(7), vec![Digit::One, Digit::Two])
    }

    #[test]
    fn test_8() {
        assert_eq!(backward(8), vec![Digit::Two, Digit::DoubleMinus])
    }

    #[test]
    fn test_9() {
        assert_eq!(backward(9), vec![Digit::Two, Digit::Minus])
    }

    #[test]
    fn test_10() {
        assert_eq!(backward(10), vec![Digit::Two, Digit::Zero])
    }

    #[test]
    fn test_15() {
        assert_eq!(
            backward(15),
            vec![Digit::One, Digit::DoubleMinus, Digit::Zero]
        )
    }

    #[test]
    fn test_20() {
        assert_eq!(backward(20), vec![Digit::One, Digit::Minus, Digit::Zero])
    }

    #[test]
    fn test_2022() {
        assert_eq!(
            backward(2022),
            vec![
                Digit::One,
                Digit::DoubleMinus,
                Digit::One,
                Digit::One,
                Digit::Minus,
                Digit::Two,
            ]
        )
    }

    #[test]
    fn test_12345() {
        assert_eq!(
            backward(12345),
            vec![
                Digit::One,
                Digit::Minus,
                Digit::Zero,
                Digit::Minus,
                Digit::Minus,
                Digit::Minus,
                Digit::Zero,
            ]
        )
    }
}
