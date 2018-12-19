#[macro_use]
extern crate nom;

use nom::types::CompleteStr as Input;

use day_16::*;

const INPUT: &'static str = include_str!("input.txt");

named! {
    code<Input, Code>,
    alt!(
        value!(Code::ADDR, tag!("addr")) |
        value!(Code::ADDI, tag!("addi")) |
        value!(Code::MULR, tag!("mulr")) |
        value!(Code::MULI, tag!("muli")) |
        value!(Code::BANR, tag!("banr")) |
        value!(Code::BANI, tag!("bani")) |
        value!(Code::BORR, tag!("borr")) |
        value!(Code::BORI, tag!("bori")) |
        value!(Code::SETR, tag!("setr")) |
        value!(Code::SETI, tag!("seti")) |
        value!(Code::GTIR, tag!("gtir")) |
        value!(Code::GTRI, tag!("gtri")) |
        value!(Code::GTRR, tag!("gtrr")) |
        value!(Code::EQIR, tag!("eqir")) |
        value!(Code::EQRI, tag!("eqri")) |
        value!(Code::EQRR, tag!("eqrr"))
    )
}

named! {
    op<Input, (Code, Op)>,
    ws!(do_parse!(
        c: code >>
        l: value >>
        r: value >>
        d: value >>
        (c, Op { c: 0, l, r, d })
    ))
}

named! {
    ip<Input, usize>,
    ws!(preceded!(tag!("#ip"), value))
}

named! {
    program<Input, (usize, Vec<(Code, Op)>)>,
    ws!(do_parse!(
        ip: ip >>
        ops: many1!(op) >>
        (ip, ops)
    ))
}

fn main() {

    let (_, (ip_reg, ops)) = program(Input(INPUT)).unwrap();
    
    let mut ip = 0;
    let mut regs = [1, 0, 0, 0, 0, 0];

    while ip < ops.len() {
        let (code, op) = ops[ip];
        regs[ip_reg] = ip;
        op.execute(code, &mut regs);
        ip = regs[ip_reg] + 1;
    }

    println!("{}", regs[0]);

}
