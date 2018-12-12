const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let sum = INPUT.split_whitespace()
        .map(|s| s.parse::<isize>())
        .filter_map(|s| s.ok())
        .sum::<isize>();

    println!("{}", sum);
}
