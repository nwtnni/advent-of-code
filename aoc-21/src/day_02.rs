use aoc::*;

#[derive(Clone, Debug)]
pub struct Dive(Vec<Command>);

#[derive(Copy, Clone, Debug)]
enum Command {
    Forward(i64),
    Down(i64),
    Up(i64),
}

impl Fro for Dive {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(Command::fro)
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Fro for Command {
    fn fro(input: &str) -> Self {
        let (direction, amount) = input.split_once(' ').unwrap();
        let amount = i64::fro(amount);
        match direction {
            "forward" => Command::Forward(amount),
            "down" => Command::Down(amount),
            "up" => Command::Up(amount),
            _ => unreachable!(),
        }
    }
}

impl Solution for Dive {
    fn one(self) -> i64 {
        let mut x = 0;
        let mut z = 0;

        for command in self.0 {
            match command {
                Command::Forward(dx) => x += dx,
                Command::Down(dz) => z += dz,
                Command::Up(dz) => z -= dz,
            }
        }

        x * z
    }

    fn two(self) -> i64 {
        let mut a = 0;
        let mut x = 0;
        let mut z = 0;

        for command in self.0 {
            match command {
                Command::Forward(dx) => {
                    x += dx;
                    z += a * dx;
                }
                Command::Down(da) => a += da,
                Command::Up(da) => a -= da,
            }
        }

        x * z
    }
}
