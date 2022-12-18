use aoc::*;

#[derive(Clone, Debug)]
pub struct PyroclasticFlow(Vec<bool>);

impl Fro for PyroclasticFlow {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .chars()
            .map(|char| match char {
                '>' => true,
                '<' => false,
                _ => unreachable!(),
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

struct Well(Vec<u8>);

impl Well {
    fn new() -> Self {
        Well(vec![0b0111_1111])
    }

    fn highest(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .rev()
            .find_map(|(index, occupied)| if *occupied > 0 { Some(index) } else { None })
            .unwrap()
    }

    #[allow(unused)]
    fn debug(&self) {
        for line in self.0.iter().rev().take(self.0.len() - 1) {
            eprint!("|");
            for j in 0..7 {
                eprint!("{}", if line & (1 << j) > 0 { '#' } else { '.' });
            }
            assert_eq!(line & 0b1000_0000, 0);
            eprintln!("|");
        }
        assert_eq!(self.0[0], 0b0111_1111);
        eprintln!("+-------+");
    }
}

impl Solution for PyroclasticFlow {
    fn one(self) -> i64 {
        let mut jets = self.0.into_iter().cycle();
        let mut well = Well::new();

        for mut shape in shape::SHAPES.into_iter().cycle().take(2022) {
            let mut i = well.highest() + 4;

            while well.0.len() < i + 4 {
                well.0.push(0);
            }

            loop {
                let right = jets.next().unwrap();

                let stuck_wall = shape.iter().any(|line| {
                    if right {
                        line & 0b0100_0000 > 0
                    } else {
                        line & 0b0000_0001 > 0
                    }
                });

                let stuck_well = shape.iter().enumerate().any(|(di, line)| {
                    well.0[i + di] & if right { line << 1 } else { line >> 1 } > 0
                });

                if !stuck_wall && !stuck_well {
                    shape
                        .iter_mut()
                        .for_each(|line| if right { *line <<= 1 } else { *line >>= 1 });
                }

                if well.0[i - 1..]
                    .iter()
                    .zip(shape)
                    .any(|(well, shape)| well & shape > 0)
                {
                    well.0[i..]
                        .iter_mut()
                        .zip(shape)
                        .for_each(|(well, shape)| *well |= shape);
                    break;
                } else {
                    i -= 1;
                }
            }
        }

        well.highest() as i64
    }

    fn two(self) -> i64 {
        todo!()
    }
}

#[rustfmt::skip]
mod shape {
    pub const SHAPES: [[u8; 4]; 5] = [H, P, L, V, B];

    const H: [u8; 4] = [
        0b0000_1111 << 2,
        0b0000_0000 << 2,
        0b0000_0000 << 2,
        0b0000_0000 << 2,
    ];

    const P: [u8; 4] = [
        0b0000_0010 << 2,
        0b0000_0111 << 2,
        0b0000_0010 << 2,
        0b0000_0000 << 2,
    ];

    const L: [u8; 4] = [
        0b0000_0111 << 2,
        0b0000_0100 << 2,
        0b0000_0100 << 2,
        0b0000_0000 << 2,
    ];

    const V: [u8; 4] = [
        0b0000_0001 << 2,
        0b0000_0001 << 2,
        0b0000_0001 << 2,
        0b0000_0001 << 2,
    ];

    const B: [u8; 4] = [
        0b0000_0011 << 2,
        0b0000_0011 << 2,
        0b0000_0000 << 2,
        0b0000_0000 << 2,
    ];
}
