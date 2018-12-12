const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let floor = INPUT.chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            _   => 0,
        })
        .sum::<isize>();

    println!("{}", floor);
}
