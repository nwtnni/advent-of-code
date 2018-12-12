const INPUT: &'static str = include_str!("input.txt");

fn escape(c: char) -> usize {
    match c {
    | '"' | '\\' => 2,
    | _ => 1,
    }
}

fn main() {
    let mut total = 0;
    for s in INPUT.trim().split('\n') {
        total += s.chars()
            .map(escape)
            .sum::<usize>() as isize -
            s.len() as isize +
            2;
    }
    println!("{}", total)
}
