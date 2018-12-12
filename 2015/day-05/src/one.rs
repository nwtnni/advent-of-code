const INPUT: &'static str = include_str!("input.txt");

fn is_vowel(c: char) -> bool {
    match c {
    | 'a' | 'e' | 'i' | 'o' | 'u' => true,
    | _ => false,
    }
}

fn is_repeated(cs: &[char]) -> bool {
    cs[0] == cs[1]
}

fn is_naughty(cs: &[char]) -> bool {
    match cs {
    | ['a', 'b'] | ['c', 'd'] | ['p', 'q'] | ['x', 'y'] => true,
    | _ => false,
    }
}

fn main() {

    let mut nice = 0;

    for s in INPUT.split_whitespace() {
        let chars = s.chars()
            .collect::<Vec<_>>();

        let vowels = chars.iter()
            .filter(|c| is_vowel(**c))
            .count();

        let twice = chars.windows(2)
            .filter(|cs| is_repeated(cs))
            .count();

        let naughty = chars.windows(2)
            .filter(|cs| is_naughty(cs))
            .count();

        if vowels >= 3 && twice >= 1 && naughty == 0 {
            nice += 1;
        }
    }

    println!("{}", nice);
}
