const INPUT: &'static str = include_str!("input.txt");

fn lexed(s: &str) -> usize {
    let mut iter = s.trim()
        .chars()
        .peekable();

    let mut count = 0;

    while iter.peek().is_some() {
        match iter.next() {
        | Some('\\') => {
            match iter.next() {
            | Some('\\')
            | Some('"') => count += 1,
            | Some('x') => {
                iter.next();
                iter.next();
                count += 1;
            }
            | _ => unreachable!(),
            }
        }
        | Some('"') => (),
        | Some(_) => count += 1,
        | None => (),
        }
    }

    count
}

fn main() {
    let mut total = 0;
    for s in INPUT.trim().split('\n') {
        total += s.len() - lexed(s);
    }
    println!("{}", total);
}
