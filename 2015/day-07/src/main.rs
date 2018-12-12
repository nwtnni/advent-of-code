use std::collections::HashMap as Map;

pub type Var = String;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Const(u16),
    Var(Var),
}

impl Value {
    fn parse(s: &str) -> Self {
        if let Ok(signal) = s.parse::<u16>() {
            Value::Const(signal)
        } else {
            Value::Var(s.to_string())
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Unop {
    Not,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Binop {
    And,
    Or,
    LShift,
    RShift,
}

impl Binop {
    fn eval(&self, l: u16, r: u16) -> u16 {
        match self {
        | Binop::And => l & r,
        | Binop::Or => l | r,
        | Binop::LShift => l << r,
        | Binop::RShift => l >> r,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Exp {
    Binary(Binop, Value, Value),
    Unary(Unop, Value),
    Value(Value),
}

impl Exp {
    fn parse(s: &str) -> Self {
        let mut iter = s.trim()
            .split_whitespace()
            .peekable();

        if iter.clone().count() == 1 {
            return Exp::Value(Value::parse(s))
        }

        if let Some(&"NOT") = iter.peek() {
            let _ = iter.next();
            let val = iter.next()
                .map(Value::parse)
                .unwrap();
            return Exp::Unary(Unop::Not, val)
        }

        let lhs = iter.next()
            .map(Value::parse)
            .unwrap();

        let op = match iter.next() {
        | Some("AND") => Binop::And,
        | Some("OR") => Binop::Or,
        | Some("LSHIFT") => Binop::LShift,
        | Some("RSHIFT") => Binop::RShift,
        | _ => unreachable!(),
        };

        let rhs = iter.next()
            .map(Value::parse)
            .unwrap();

        Exp::Binary(op, lhs, rhs)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Stm {
    exp: Exp,
    var: Var,
}

impl Stm {
    fn parse(s: &str) -> Self {
        let mut iter = s.trim().split("->");
        let exp = iter.next()
            .map(str::trim)
            .map(Exp::parse)
            .unwrap();
        let var = iter.next()
            .map(str::trim)
            .map(str::to_string)
            .unwrap();
        Stm { exp, var }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program(Vec<Stm>);

impl Program {
    pub fn parse(s: &str) -> Self {
        let mut stms = Vec::default();
        for line in s.trim().split("\n") {
            let stm = Stm::parse(line);
            stms.push(stm);
        }
        Program(stms)
    }

    pub fn run(self) -> Map<String, u16> {
        let mut signals = Map::default(); 
        let mut current = self.0;
        let mut retain = Vec::default();
        
        while !current.is_empty() {
            while let Some(stm) = current.pop() {
                match stm.exp {
                | Exp::Value(Value::Const(s)) => {
                    signals.insert(stm.var, s);
                }
                | Exp::Value(Value::Var(ref v)) 
                    if signals.contains_key(v) => {
                    let v = signals[v];
                    signals.insert(stm.var, v);
                }
                | Exp::Binary(op, Value::Const(l), Value::Const(r)) => {
                    signals.insert(stm.var, op.eval(l, r));
                }
                | Exp::Binary(op, Value::Const(s), Value::Var(ref v)) 
                    if signals.contains_key(v) => {
                    let v = signals[v];
                    signals.insert(stm.var, op.eval(s, v));
                }
                | Exp::Binary(op, Value::Var(ref v), Value::Const(s))
                    if signals.contains_key(v) => {
                    let v = signals[v];
                    signals.insert(stm.var, op.eval(v, s));
                }
                | Exp::Binary(op, Value::Var(ref l), Value::Var(ref r))
                    if signals.contains_key(l)
                    && signals.contains_key(r) => {
                    let l = signals[l];
                    let r = signals[r];
                    signals.insert(stm.var, op.eval(l, r));
                }
                | Exp::Unary(Unop::Not, Value::Const(s)) => {
                    signals.insert(stm.var, !s);
                }
                | Exp::Unary(Unop::Not, Value::Var(ref v))
                    if signals.contains_key(v) => {
                    let v = signals[v];
                    signals.insert(stm.var, !v);
                }
                | _ => {
                    retain.push(stm);
                }
                }
            }
            current = retain.drain(..).collect();
        }

        signals
    }
}

const INPUT: &'static str = include_str!("input-one.txt");

fn main() {
    let program = Program::parse(INPUT);
    let signals = program.run();
    println!("{}", signals["a"]);
}
