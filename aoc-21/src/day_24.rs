use aoc::*;

#[derive(Clone, Debug)]
pub struct ArithmeticLogicUnit(Vec<Asm>);

#[derive(Copy, Clone, Debug)]
enum Asm {
    Inp(Dst),
    Bin(Op, Dst, Src),
}

#[derive(Copy, Clone, Debug)]
enum Src {
    Num(i64),
    Var(usize),
}

#[derive(Copy, Clone, Debug)]
struct Dst(usize);

#[derive(Copy, Clone, Debug)]
enum Op {
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

impl Fro for ArithmeticLogicUnit {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(Asm::fro)
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Fro for Src {
    fn fro(input: &str) -> Self {
        if let Ok(num) = input.parse::<i64>() {
            Src::Num(num)
        } else {
            assert_eq!(input.len(), 1);
            Src::Var((input.chars().give() as u8 - b'w') as usize)
        }
    }
}

impl Fro for Dst {
    fn fro(input: &str) -> Self {
        assert_eq!(input.len(), 1);
        Dst((input.chars().give() as u8 - b'w') as usize)
    }
}

impl Fro for Asm {
    fn fro(input: &str) -> Self {
        let (a, b) = input.split_once(' ').unwrap();
        match a {
            "inp" => Asm::Inp(Dst::fro(b)),
            op => {
                let (l, r) = b.split_once(' ').unwrap();
                let op = Op::fro(op);
                Asm::Bin(op, Dst::fro(l), Src::fro(r))
            }
        }
    }
}

impl Fro for Op {
    fn fro(input: &str) -> Self {
        match input {
            "add" => Op::Add,
            "mul" => Op::Mul,
            "div" => Op::Div,
            "mod" => Op::Mod,
            "eql" => Op::Eql,
            _ => unreachable!(),
        }
    }
}

const DIV: [bool; 14] = [
    false, false, false, true, false, false, true, true, true, false, false, true, true, true,
];
const DX: [i64; 14] = [11, 13, 15, -8, 13, 15, -11, -4, -15, 14, 14, -1, -8, -14];
const DY: [i64; 14] = [6, 14, 14, 10, 9, 12, 8, 13, 12, 6, 9, 15, 4, 10];

impl Solution for ArithmeticLogicUnit {
    fn one(self) -> i64 {
        self.check3((1..=9).rev())
    }

    fn two(self) -> i64 {
        self.check3(1..=9)
    }
}

impl ArithmeticLogicUnit {
    #[allow(dead_code)]
    fn check(&self, model: [u8; 14]) -> bool {
        let mut regs = [0i64; 4];
        let mut input = 0;

        for asm in &self.0 {
            match *asm {
                Asm::Inp(Dst(dst)) => {
                    regs[dst] = model[input] as i64;
                    input += 1;
                }
                Asm::Bin(op, Dst(dst), src) => {
                    let src = match src {
                        Src::Num(num) => num,
                        Src::Var(var) => regs[var],
                    };

                    regs[dst] = match op {
                        Op::Add => regs[dst] + src,
                        Op::Mul => regs[dst] * src,
                        Op::Div => regs[dst] / src,
                        Op::Mod => regs[dst] % src,
                        Op::Eql => (regs[dst] == src) as i64,
                    }
                }
            }
        }

        regs[3] == 0
    }

    // let mut x = 0;
    // x += z;
    //
    // if x < 0 {
    //     return false;
    // }
    //
    // x = x % 26;
    // z = if div { z / 26 } else { z };
    //
    // x += dx;
    // x = if x == w { 1 } else { 0 };
    // x = if x == 0 { 1 } else { 0 };
    //
    // let mut y = 0;
    // y += 25;
    // y *= x;
    // y += 1;
    // z *= y;
    //
    // y = 0;
    // y += w;
    // y += dy;
    // y *= x;
    // z += y;
    #[allow(dead_code)]
    fn check2(&self, model: [i64; 14]) -> bool {
        let mut z = 0;

        for (((w, div), dx), dy) in model.into_iter().zip(DIV).zip(DX).zip(DY) {
            if z < 0 {
                return false;
            }

            let x = (w != (z % 26 + dx)) as i64;
            z = if div { z / 26 } else { z };
            z = z * (25 * x + 1) + ((w + dy) * x);
        }

        z == 0
    }

    fn check3(&self, range: impl Iterator<Item = i64> + Clone) -> i64 {
        for d00 in range.clone() {
            let z0 = d00 + DY[0];

            for d01 in range.clone() {
                let z1 = z0 * 26 + d01 + DY[1];

                for d02 in range.clone() {
                    let z2 = z1 * 26 + d02 + DY[2];

                    for d03 in range.clone() {
                        if z2 % 26 + DX[3] != d03 {
                            continue;
                        }

                        let z3 = z2 / 26;

                        for d04 in range.clone() {
                            let z4 = z3 * 26 + d04 + DY[4];

                            for d05 in range.clone() {
                                let z5 = z4 * 26 + d05 + DY[5];

                                for d06 in range.clone() {
                                    if z5 % 26 + DX[6] != d06 {
                                        continue;
                                    }

                                    let z6 = z5 / 26;

                                    for d07 in range.clone() {
                                        if z6 % 26 + DX[7] != d07 {
                                            continue;
                                        }

                                        let z7 = z6 / 26;

                                        for d08 in range.clone() {
                                            if z7 % 26 + DX[8] != d08 {
                                                continue;
                                            }

                                            let z8 = z7 / 26;

                                            for d09 in range.clone() {
                                                let z9 = z8 * 26 + d09 + DY[9];

                                                for d10 in range.clone() {
                                                    let z10 = z9 * 26 + d10 + DY[10];

                                                    for d11 in range.clone() {
                                                        if z10 % 26 + DX[11] != d11 {
                                                            continue;
                                                        }

                                                        let z11 = z10 / 26;

                                                        for d12 in range.clone() {
                                                            if z11 % 26 + DX[12] != d12 {
                                                                continue;
                                                            }

                                                            let z12 = z11 / 26;

                                                            for d13 in range.clone() {
                                                                if z12 % 26 + DX[13] != d13 {
                                                                    continue;
                                                                }

                                                                return [
                                                                    d00, d01, d02, d03, d04, d05,
                                                                    d06, d07, d08, d09, d10, d11,
                                                                    d12, d13,
                                                                ]
                                                                .into_iter()
                                                                .map(|char| char as u8 + b'0')
                                                                .map(|char| char as char)
                                                                .collect::<String>()
                                                                .tap(|model| i64::fro(&model));
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        unreachable!()
    }
}
