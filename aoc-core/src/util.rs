pub fn fst<A, B>((a, _): (A, B)) -> A {
    a 
}

pub fn snd<A, B>((_, b): (A, B)) -> B {
    b
}
