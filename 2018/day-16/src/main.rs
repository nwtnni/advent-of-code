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

fn flexible(threshold: usize, samples: &[(Reg, Op, Reg)]) -> usize {
    let mut flexible = 0;
    for (before, op, after) in samples {
        let mut success = 0;
        for code in Code::all() {
            let mut attempt = before.clone();
            op.execute(*code, &mut attempt);
            success += if attempt == *after { 1 } else { 0 };
        }
        flexible += if success >= threshold { 1 } else { 0 };
    }
    flexible 
}

fn main() {

    let mut lines = SAMPLES.trim().lines();
    let mut samples: Vec<(Reg, Op, Reg)> = Vec::new();

    loop {
        match (lines.next(), lines.next(), lines.next()) {
        | (Some(b), Some(o), Some(a)) => {
            let (_, b) = before(Input(b)).unwrap();
            let (_, o) = op(Input(o)).unwrap();
            let (_, a) = after(Input(a)).unwrap();
            lines.next();
            samples.push((b, o, a));
        }
        | _ => break,
        };
    }

    let mut possible: Map<usize, Set<&'static Code>> = Map::default();

    for (before, op, after) in samples {
        if !possible.contains_key(&op.c) {
            possible.insert(op.c, Code::all().collect());
        }
        for code in Code::all() {
            let mut attempt = before.clone();
            op.execute(*code, &mut attempt);
            if attempt != after {
                possible.entry(op.c).and_modify(|set| { set.remove(code); });
            }
        }
    }

    let program = PROGRAM.trim()
        .lines()
        .map(|line| op(Input(line)))
        .map(Result::unwrap)
        .map(|(_, op)| op)
        .collect::<Vec<_>>();

    let mut assignment: Map<usize, &'static Code> = Map::default();

    while assignment.len() < 16 {
        for op in 0..16 {
            if possible[&op].len() == 1 {
                let assigned = possible[&op].iter()
                    .next()
                    .unwrap()
                    .clone();
                assignment.insert(op, assigned);
                for other in (0..16).filter(|other| *other != op) {
                    possible.entry(other)
                        .and_modify(|set| { set.remove(assigned); });
                }
            }
        }
    }

    let mut reg = [0, 0, 0, 0];

    for op in program {
        op.execute(*assignment[&op.c], &mut reg);
    }

    println!("{}", reg[0]);
}
