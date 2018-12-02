extern crate md5;

const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let prefix = INPUT.trim();
    let mut n = 1;
    loop {
        let data = format!("{}{}", prefix, n);
        let digest = md5::compute(&data);
        let hex = format!("{:x}", digest);
        if &hex[0..6] == "000000" {
            println!("{}", hex);
            println!("{}", n);
            break;
        }
        n += 1;
    }
}
