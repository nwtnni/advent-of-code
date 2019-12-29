use std::str;

use aoc::*;

pub struct SlamShuffle(Vec<Technique>);

enum Technique {
    Flip,
    Deal(i64),
    Cut(i64),
}

impl Technique {
    fn shuffle(&self, pos: i64, len: i64) -> i64 {
        use Technique::*;
        match self {
        | Flip => len - 1 - pos,
        | Deal(n) => (pos * n) % len,
        | Cut(n) if *n > 0 => (pos + len - n) % len,
        | Cut(n) => (pos - n) % len,
        }
    }
}

impl Fro for SlamShuffle {
    fn fro(input: &str) -> Self {
        let mut shuffle = Vec::new();
        for line in input.trim().split('\n') {
            let mut iter = line.trim().split_whitespace();
            match iter.next() {
            | Some("deal") => {
                match iter.next() {
                | Some("with") => {
                    shuffle.push(Technique::Deal(
                        iter.nth(1).unwrap().to()
                    ));
                }
                | Some("into") => {
                    shuffle.push(Technique::Flip);
                }
                | _ => unreachable!(),
                }
            }
            | Some("cut") => {
                shuffle.push(Technique::Cut(
                    iter.give().to()
                ));
            }
            | _ => unreachable!(),
            }
        }
        SlamShuffle(shuffle)
    }
}

impl Solution for SlamShuffle {
    fn one(self) -> i64 {
        self.0
            .iter()
            .fold(2019, |pos, next| next.shuffle(pos, 10007))
    }

    fn two(self) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_flip() {
        let flip = Technique::Flip;
        assert_eq!(flip.shuffle(0, 10), 9);
        assert_eq!(flip.shuffle(1, 10), 8);
        assert_eq!(flip.shuffle(2, 10), 7);
        assert_eq!(flip.shuffle(3, 10), 6);
        assert_eq!(flip.shuffle(4, 10), 5);
        assert_eq!(flip.shuffle(5, 10), 4);
        assert_eq!(flip.shuffle(6, 10), 3);
        assert_eq!(flip.shuffle(7, 10), 2);
        assert_eq!(flip.shuffle(8, 10), 1);
        assert_eq!(flip.shuffle(9, 10), 0);
    }

    #[test]
    fn test_cut_pos() {
        let cut = Technique::Cut(3);
        assert_eq!(cut.shuffle(0, 10), 7);
        assert_eq!(cut.shuffle(1, 10), 8);
        assert_eq!(cut.shuffle(2, 10), 9);
        assert_eq!(cut.shuffle(3, 10), 0);
        assert_eq!(cut.shuffle(4, 10), 1);
        assert_eq!(cut.shuffle(5, 10), 2);
        assert_eq!(cut.shuffle(6, 10), 3);
        assert_eq!(cut.shuffle(7, 10), 4);
        assert_eq!(cut.shuffle(8, 10), 5);
        assert_eq!(cut.shuffle(9, 10), 6);
    }

    #[test]
    fn test_cut_neg() {
        let cut = Technique::Cut(-4);
        assert_eq!(cut.shuffle(0, 10), 4);
        assert_eq!(cut.shuffle(1, 10), 5);
        assert_eq!(cut.shuffle(2, 10), 6);
        assert_eq!(cut.shuffle(3, 10), 7);
        assert_eq!(cut.shuffle(4, 10), 8);
        assert_eq!(cut.shuffle(5, 10), 9);
        assert_eq!(cut.shuffle(6, 10), 0);
        assert_eq!(cut.shuffle(7, 10), 1);
        assert_eq!(cut.shuffle(8, 10), 2);
        assert_eq!(cut.shuffle(9, 10), 3);
    }

    #[test]
    fn test_deal() {
        let deal = Technique::Deal(3);
        assert_eq!(deal.shuffle(0, 10), 0);
        assert_eq!(deal.shuffle(1, 10), 3);
        assert_eq!(deal.shuffle(2, 10), 6);
        assert_eq!(deal.shuffle(3, 10), 9);
        assert_eq!(deal.shuffle(4, 10), 2);
        assert_eq!(deal.shuffle(5, 10), 5);
        assert_eq!(deal.shuffle(6, 10), 8);
        assert_eq!(deal.shuffle(7, 10), 1);
        assert_eq!(deal.shuffle(8, 10), 4);
        assert_eq!(deal.shuffle(9, 10), 7);
    }
}
