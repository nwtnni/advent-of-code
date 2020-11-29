const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let ids = INPUT.trim()
        .split_whitespace()
        .collect::<Vec<_>>();

    for i in 0..ids.len() {
        for j in i + 1..ids.len() {

            let differ = ids[i].chars()
                .zip(ids[j].chars().enumerate())
                .filter(|(a, (_, b))| a != b)
                .collect::<Vec<_>>();

            if differ.len() == 1 {
                let mut common = ids[j].to_string();
                common.remove((differ[0].1).0);
                println!("{}", common);
                return
            }
        }
    }
}
