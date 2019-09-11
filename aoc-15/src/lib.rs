use std::str;

mod day_01;

pub fn solve(day: aoc::Day, part: aoc::Part, input: &str) -> Result<usize, aoc::Error> {
    use aoc::Parse;
    match day {
    | aoc::Day::D01 => day_01::NotQuiteLisp::parse(input),
    | _ => unimplemented!(),
    }.map(|mut sol| sol.solve(part))
}
