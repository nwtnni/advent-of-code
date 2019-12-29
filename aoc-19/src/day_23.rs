use std::str;
use std::sync::mpsc;
use std::thread;
use std::time;

use aoc::*;

static TIMEOUT: time::Duration = time::Duration::from_millis(1);

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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Packet {
    a: i64,
    x: i64,
    y: i64,
}

struct Network {
    macro_rx: mpsc::Receiver<Packet>,
    micro_txs: Vec<mpsc::Sender<Packet>>,
}

impl CategorySix {
    fn spawn_network(&self) -> Network {
        let (macro_tx, macro_rx) = mpsc::channel();
        let micro_txs = (0..50)
            .map(|address| self.spawn(address, macro_tx.clone()))
            .collect::<Vec<_>>();
        Network {
            macro_rx,
            micro_txs,
        }
    }

    fn spawn(&self, address: i64, macro_tx: mpsc::Sender<Packet>) -> mpsc::Sender<Packet> {

        let mut nic = self.0.clone();

        nic.input(address);

        let (micro_tx, micro_rx) = mpsc::channel();

        thread::spawn(move || {

            // Input state
            let mut is = IS::X;
            let mut ix;
            let mut iy = 0;

            // Output state
            let mut os = OS::A;
            let mut oa = 0;
            let mut ox = 0;
            let mut oy;

            loop {
                use intcode::Yield::*;
                match (nic.step(), is, os) {
                | (Halt, _, _) => return,
                | (Step, _, _) => continue,
                | (Input(i), IS::X, _) => {
                    match micro_rx.recv_timeout(TIMEOUT) {
                    | Ok(Packet { x, y, .. }) => {
                        is = IS::Y;
                        ix = x;
                        iy = y;
                        nic[i] = ix;
                    }
                    | Err(mpsc::RecvTimeoutError::Timeout) => {
                        nic[i] = -1;
                    }
                    | Err(mpsc::RecvTimeoutError::Disconnected) => {
                        return;
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
                    os = OS::A;
                    oy = y;
                    macro_tx
                        .send(Packet { a: oa, x: ox, y: oy })
                        .expect("macro_tx disconnected");
                }
                }
            }
        });

        micro_tx
    }
}

impl Solution for CategorySix {
    fn one(self) -> i64 {
        let Network { macro_rx, micro_txs } = self.spawn_network();
        loop {
            match macro_rx.recv_timeout(TIMEOUT) {
            | Ok(Packet { a: 255, x: _, y }) => {
                return y;
            }
            | Ok(Packet { a, x, y }) => {
                micro_txs[a as usize]
                    .send(Packet { a, x, y })
                    .expect("micro_tx disconnected");
            }
            | Err(mpsc::RecvTimeoutError::Timeout) => continue,
            | Err(mpsc::RecvTimeoutError::Disconnected) => panic!("macro_rx disconnected"),
            }
        }
    }

    fn two(self) -> i64 {
        let Network { macro_rx, micro_txs } = self.spawn_network();
        let mut packet = None;
        let mut latest = None;
        loop {
            match macro_rx.recv_timeout(TIMEOUT) {
            | Ok(Packet { a: 255, x, y }) => {
                packet = Some(Packet { a: 255, x, y });
            }
            | Ok(Packet { a, x, y }) => {
                micro_txs[a as usize]
                    .send(Packet { a, x, y })
                    .expect("micro_tx disconnected");
            }
            | Err(mpsc::RecvTimeoutError::Timeout) => {
                if let Some(Packet { x, y, .. }) = packet {
                    if latest == packet {
                        return y;
                    }

                    latest = packet;

                    micro_txs[0]
                        .send(Packet { a: 0, x, y })
                        .expect("micro_tx disconnected");
                }
            }
            | Err(mpsc::RecvTimeoutError::Disconnected) => panic!("macro_rx disconnected"),
            }

        }
    }
}
