use std::collections::HashMap as Map;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let mut nice = 0;
    for s in INPUT.split_whitespace() {
        let mut chars = s.chars()
            .collect::<Vec<_>>();

        let sandwich = chars.windows(3)
            .filter(|cs| cs[0] == cs[2])
            .count();
        
        let mut repeat = 0; 
        let mut seen: Map<&[char], usize> = Map::default();

        for (i, cs) in chars.windows(2).enumerate() {
            if seen.contains_key(&cs) {
                if i > seen[&cs] + 1 {
                    repeat += 1;
                }
            } else {
                seen.insert(cs, i); 
            }
        }

        if sandwich > 0 && repeat > 0 {
            nice += 1
        }
    }

    println!("{}", nice);
}
