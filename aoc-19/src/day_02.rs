use std::str;

pub struct ProgramAlarm(Vec<i32>);

impl str::FromStr for ProgramAlarm {
    type Err = aoc::Error;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        input.trim()
            .split(',')
            .map(|line| line.parse::<i32>())
            .collect::<Result<Vec<_>, _>>()
            .map(ProgramAlarm)
            .map_err(aoc::Error::InvalidInt)
    }
}

impl aoc::Solution for ProgramAlarm {
    fn one(&mut self) -> i32 {
        self.0[1] = 12;
        self.0[2] = 2;

        let mut ip = 0;

        while self.0[ip] != 99 {

            let a = self.0[ip + 1] as usize;
            let b = self.0[ip + 2] as usize;
            let dst = self.0[ip + 3] as usize;

            if self.0[ip] == 1 {
                self.0[dst] = self.0[a] + self.0[b];
            } else {
                self.0[dst] = self.0[a] * self.0[b];
            }

            ip += 4;
        }

        self.0[0]
    }

    fn two(&mut self) -> i32 {
        unimplemented!()
    }
}
