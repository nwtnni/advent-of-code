#[macro_use]
extern crate nom;

use std::collections::{HashMap as Map, HashSet as Set};

use nom::{
    digit1 as nom_digit1,
    types::CompleteStr as Input,
};

use day_16::*;

const SAMPLES: &'static str = include_str!("input_1.txt");
const PROGRAM: &'static str = include_str!("input_2.txt");

type Reg = [usize; 4];

named! {
    value<Input, usize>,
    map!(nom_digit1, |s| s.parse::<usize>().unwrap())
}

named! {
    reg<Input, Reg>,
    delimited!(
        tag!("["),
        map!(separated_list!(tag!(", "), value), |r| [r[0], r[1], r[2], r[3]]),
        tag!("]")
    )
}

named! {
    op<Input, Op>,
    ws!(do_parse!(
        c: value >>
        l: value >>
        r: value >>
        d: value >>
        (Op { c, l, r, d })
    ))
}

named! {
    before<Input, Reg>,
    ws!(preceded!(tag!("Before:"), reg))
}

named! {
    after<Input, Reg>,
    ws!(preceded!(tag!("After:"), reg))
}

fn flexible(samples: &[(Reg, Op, Reg)]) -> usize {
    const THRESHOLD: usize = 3;
    let mut flexible = 0;
    for (before, op, after) in samples {
        let mut success = 0;
        for code in Code::all() {
            let mut attempt = before.clone();
            op.execute(*code, &mut attempt);
            success += if attempt == *after { 1 } else { 0 };
        }
        flexible += if success >= THRESHOLD { 1 } else { 0 };
    }
    flexible 
}

fn main() {

    let mut lines = SAMPLES.trim().split("\n");
    let mut samples: Vec<(Reg, Op, Reg)> = Vec::new();

    loop {
        if let Ok((_, before)) = before(Input(lines.next().unwrap())) {
            let (_, op) = op(Input(lines.next().unwrap())).unwrap();
            let (_, after) = after(Input(lines.next().unwrap())).unwrap();
            let _ = lines.next().unwrap();
            samples.push((before, op, after));
        } else {
            break
        }
    }

}
