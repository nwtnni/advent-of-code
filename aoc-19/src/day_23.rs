use std::str;
use std::sync::mpsc;
use std::thread;
use std::time;

use aoc::*;

static TIMEOUT: time::Duration = time::Duration::from_secs(1);

pub struct CategorySix(intcode::Program);

impl Fro for CategorySix {
    fn fro(input: &str) -> Self {
        CategorySix(intcode::Program::fro(input))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum IS {
    X,
    Y,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum OS {
    A,
    X,
    Y,
}

impl Solution for CategorySix {
    fn one(self) -> i64 {

        let (macro_tx, macro_rx) = mpsc::channel();

        let network = (0..50).map(|address| {

            let (micro_tx, micro_rx) = mpsc::channel();
            let macro_tx = macro_tx.clone();
            let mut nic = self.0.clone();
            nic.input(address).unwrap();

            thread::spawn(move || {
                let mut is = IS::X;
                let mut os = OS::A;

                let mut iy = 0;

                let mut oa = 0;
                let mut ox = 0;

                loop {
                    use intcode::Yield::*;
                    match (nic.step(), is, os) {
                    | (Halt, _, _) => break,
                    | (Step, _, _) => continue,
                    | (Input(i), IS::X, _) => {
                        match micro_rx.recv_timeout(TIMEOUT) {
                        | Ok((x, y)) => {
                            is = IS::Y;
                            iy = y;
                            nic[i] = x;
                        }
                        | Err(mpsc::RecvTimeoutError::Timeout) => {
                            nic[i] = -1;
                        }
                        | Err(mpsc::RecvTimeoutError::Disconnected) => {
                            panic!("micro_rx disconnected");
                        }
                        }
                    }
                    | (Input(i), IS::Y, _) => {
                        is = IS::X;
                        nic[i] = iy;
                    }
                    | (Output(a), _, OS::A) => {
                        os = OS::X;
                        oa = a;
                    }
                    | (Output(x), _, OS::X) => {
                        os = OS::Y;
                        ox = x;
                    }
                    | (Output(y), _, OS::Y) => {
                        macro_tx
                            .send((oa, ox, y))
                            .expect("macro_tx disconnected");
                    }
                    }
                }
            });

            micro_tx
        })
        .collect::<Vec<_>>();

        loop {
            match macro_rx.recv_timeout(TIMEOUT) {
            | Ok((255, _, y)) => {
                return y;
            }
            | Ok((a, x, y)) => {
                println!("{:02} <- ({}, {})", a, x, y);
                network[a as usize]
                    .send((x, y))
                    .expect("micro_tx disconnected");
            }
            | Err(mpsc::RecvTimeoutError::Timeout) => (),
            | Err(mpsc::RecvTimeoutError::Disconnected) => panic!("macro_rx disconnected"),
            }
        }
    }

    fn two(self) -> i64 {
        unimplemented!()
    }
}
