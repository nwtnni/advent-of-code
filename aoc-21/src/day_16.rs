use aoc::*;

#[derive(Clone, Debug)]
pub struct PacketDecoder(Vec<u8>);

impl Fro for PacketDecoder {
    fn fro(input: &str) -> Self {
        let mut bits = Vec::new();
        for char in input.trim().chars() {
            match char {
                '0' => bits.extend([0, 0, 0, 0]),
                '1' => bits.extend([0, 0, 0, 1]),
                '2' => bits.extend([0, 0, 1, 0]),
                '3' => bits.extend([0, 0, 1, 1]),
                '4' => bits.extend([0, 1, 0, 0]),
                '5' => bits.extend([0, 1, 0, 1]),
                '6' => bits.extend([0, 1, 1, 0]),
                '7' => bits.extend([0, 1, 1, 1]),
                '8' => bits.extend([1, 0, 0, 0]),
                '9' => bits.extend([1, 0, 0, 1]),
                'A' => bits.extend([1, 0, 1, 0]),
                'B' => bits.extend([1, 0, 1, 1]),
                'C' => bits.extend([1, 1, 0, 0]),
                'D' => bits.extend([1, 1, 0, 1]),
                'E' => bits.extend([1, 1, 1, 0]),
                'F' => bits.extend([1, 1, 1, 1]),
                _ => unreachable!(),
            }
        }
        Self(bits)
    }
}

impl Solution for PacketDecoder {
    fn one(self) -> i64 {
        let (packet, _) = parse(&self.0).unwrap();
        fn recurse(packet: &Packet) -> usize {
            packet.version
                + match &packet.kind {
                    Kind::Literal(_) => 0,
                    Kind::Operator(_, packets) => packets.iter().map(recurse).sum(),
                }
        }
        recurse(&packet) as i64
    }

    fn two(self) -> i64 {
        let (packet, _) = parse(&self.0).unwrap();
        fn recurse(packet: &Packet) -> usize {
            match &packet.kind {
                Kind::Literal(literal) => *literal,
                Kind::Operator(mode, packets) => match mode {
                    Mode::Sum => packets.iter().map(recurse).sum(),
                    Mode::Product => packets.iter().map(recurse).product(),
                    Mode::Min => packets.iter().map(recurse).min().unwrap(),
                    Mode::Max => packets.iter().map(recurse).max().unwrap(),
                    Mode::Gt => {
                        if recurse(&packets[0]) > recurse(&packets[1]) {
                            1
                        } else {
                            0
                        }
                    }
                    Mode::Lt => {
                        if recurse(&packets[0]) < recurse(&packets[1]) {
                            1
                        } else {
                            0
                        }
                    }
                    Mode::Eq => {
                        if recurse(&packets[0]) == recurse(&packets[1]) {
                            1
                        } else {
                            0
                        }
                    }
                },
            }
        }
        recurse(&packet) as i64
    }
}

#[derive(Clone, Debug)]
struct Packet {
    version: usize,
    kind: Kind,
}

#[derive(Clone, Debug)]
enum Kind {
    Literal(usize),
    Operator(Mode, Vec<Packet>),
}

#[derive(Copy, Clone, Debug)]
enum Mode {
    Sum,
    Product,
    Min,
    Max,
    Gt,
    Lt,
    Eq,
}

fn parse(bits: &[u8]) -> Option<(Packet, &[u8])> {
    if bits.iter().all(|bit| *bit == 0) {
        return None;
    }

    let version = decode(&bits[0..3]);
    let id = decode(&bits[3..6]);

    match id {
        4 => {
            let mut literal = Vec::new();
            let mut index = 6;
            while bits[index] == 1 {
                literal.extend(&bits[index + 1..index + 5]);
                index += 5;
            }

            literal.extend(&bits[index + 1..index + 5]);
            index += 5;

            Some((
                Packet {
                    version,
                    kind: Kind::Literal(decode(&literal)),
                },
                &bits[index..],
            ))
        }
        op => {
            let mode = match op {
                0 => Mode::Sum,
                1 => Mode::Product,
                2 => Mode::Min,
                3 => Mode::Max,
                5 => Mode::Gt,
                6 => Mode::Lt,
                7 => Mode::Eq,
                _ => unreachable!(),
            };

            match bits[6] {
                0 => {
                    let len = decode(&bits[7..22]);
                    let mut packets = Vec::new();
                    let mut next = &bits[22..22 + len];

                    while let Some((packet, remaining)) = parse(next) {
                        packets.push(packet);
                        next = remaining;
                    }

                    Some((
                        Packet {
                            version,
                            kind: Kind::Operator(mode, packets),
                        },
                        &bits[22 + len..],
                    ))
                }
                1 => {
                    let count = decode(&bits[7..18]);
                    let mut packets = Vec::new();
                    let mut next = &bits[18..];

                    for _ in 0..count {
                        let (packet, remaining) = parse(next).unwrap();
                        packets.push(packet);
                        next = remaining;
                    }

                    Some((
                        Packet {
                            version,
                            kind: Kind::Operator(mode, packets),
                        },
                        next,
                    ))
                }
                _ => unreachable!(),
            }
        }
    }
}

fn decode(bits: &[u8]) -> usize {
    let mut value = 0;
    let mut index = 0;
    for bit in bits.iter().rev() {
        value |= (*bit as usize) << index;
        index += 1;
    }
    value
}
