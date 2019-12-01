mod day_01;

pub fn solve(day: aoc::Day, part: aoc::Part, input: &str) -> Result<i32, aoc::Error> {
    use aoc::Parse;
    match day {
    | aoc::Day::D01 => day_01::TheTyrannyOfTheRocketEquation::parse(input),
    | _ => unimplemented!(),
    }.map(|mut sol| sol.solve(part))
}
