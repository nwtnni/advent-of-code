const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let mut floor = 0;
    let mut count = 0;
    for c in INPUT.chars() {
        count += 1;
        floor += match c {
        | '(' => 1,
        | ')' => -1,
        | _   => 0,
        };

        if floor == -1 { break }
    }
    println!("{}", count);
}
