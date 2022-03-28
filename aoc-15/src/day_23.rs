use aoc::*;

#[derive(Clone, Debug)]
pub struct OpeningTheTuringLock(Vec<Instruction>);

#[derive(Copy, Clone, Debug)]
enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(i64),
    Jie(Register, i64),
    Jio(Register, i64),
}

#[derive(Copy, Clone, Debug)]
enum Register {
    A = 0,
    B = 1,
}

impl Fro for OpeningTheTuringLock {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(Instruction::fro)
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Fro for Instruction {
    fn fro(input: &str) -> Self {
        let (operation, arguments) = input.split_once(' ').unwrap();
        match operation {
            "hlf" => return Instruction::Hlf(Register::fro(arguments)),
            "tpl" => return Instruction::Tpl(Register::fro(arguments)),
            "inc" => return Instruction::Inc(Register::fro(arguments)),
            "jmp" => return Instruction::Jmp(i64::fro(arguments)),
            _ => (),
        }

        let (register, offset) = arguments.split_once(", ").unwrap();
        let register = Register::fro(register);
        let offset = i64::fro(offset);

        match operation {
            "jie" => Instruction::Jie(register, offset),
            "jio" => Instruction::Jio(register, offset),
            _ => unreachable!(),
        }
    }
}

impl Fro for Register {
    fn fro(input: &str) -> Self {
        match input {
            "a" => Register::A,
            "b" => Register::B,
            _ => unreachable!(),
        }
    }
}

impl Solution for OpeningTheTuringLock {
    fn one(self) -> i64 {
        self.execute([0, 0])
    }

    fn two(self) -> i64 {
        self.execute([1, 0])
    }
}

impl OpeningTheTuringLock {
    fn execute(&self, mut registers: [i64; 2]) -> i64 {
        let mut ip = 0i64;

        while ip >= 0 && ip < self.0.len() as i64 {
            match self.0[ip as usize] {
                Instruction::Hlf(register) => registers[register as usize] >>= 1,
                Instruction::Tpl(register) => registers[register as usize] *= 3,
                Instruction::Inc(register) => registers[register as usize] += 1,
                Instruction::Jmp(offset) => {
                    ip += offset;
                    continue;
                }
                Instruction::Jie(register, offset) if registers[register as usize] & 1 == 0 => {
                    ip += offset;
                    continue;
                }
                Instruction::Jie(_, _) => (),
                Instruction::Jio(register, offset) if registers[register as usize] == 1 => {
                    ip += offset;
                    continue;
                }
                Instruction::Jio(_, _) => (),
            }

            ip += 1;
        }

        registers[1]
    }
}
