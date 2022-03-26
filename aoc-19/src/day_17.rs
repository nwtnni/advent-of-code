use std::collections::HashSet;
use std::thread;
use std::time;

use aoc::*;

pub struct SetAndForget(intcode::Program);

impl Fro for SetAndForget {
    fn fro(input: &str) -> Self {
        SetAndForget(intcode::Program::fro(input))
    }
}

fn parse_dir(output: i64) -> Dir {
    match output as u8 {
        b'^' => Dir::N,
        b'v' => Dir::S,
        b'>' => Dir::E,
        b'<' => Dir::W,
        _ => unreachable!(),
    }
}

impl SetAndForget {
    fn scaffold(&mut self) -> (HashSet<Pos>, Pos, Dir) {
        let mut scaffold = HashSet::new();
        let mut pos = Pos::default();
        let mut dir = Dir::N;
        let mut x = 0;
        let mut y = 0;
        while let Some(next) = self.0.output() {
            match next as u8 {
                b'#' => {
                    scaffold.insert(Pos { x, y });
                    x += 1;
                }
                b'\n' => {
                    y += 1;
                    x = 0;
                }
                b'v' | b'<' | b'>' | b'^' => {
                    dir = parse_dir(next);
                    pos = Pos { x, y };
                    scaffold.insert(pos);
                    x += 1;
                }
                _ => {
                    x += 1;
                }
            }
        }
        (scaffold, pos, dir)
    }

    #[allow(unused)]
    fn plot(&mut self) {
        let (scaffold, pos, dir) = self.scaffold();
        for y in 0..80 {
            for x in 0..80 {
                if x == pos.x && y == pos.y {
                    match dir {
                        Dir::N => print!("^"),
                        Dir::S => print!("v"),
                        Dir::E => print!(">"),
                        Dir::W => print!("<"),
                    }
                } else if scaffold.contains(&Pos { x, y }) {
                    print!("█");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

impl Solution for SetAndForget {
    fn one(mut self) -> i64 {
        let (scaffold, _, _) = self.scaffold();
        let mut sum = 0;
        let get = |x, y| scaffold.contains(&Pos { x, y });
        for y in 0..80i64 {
            for x in 0..80i64 {
                if get(x, y) && get(x + 1, y) && get(x - 1, y) && get(x, y + 1) && get(x, y - 1) {
                    sum += x * y;
                }
            }
        }
        sum
    }

    /// ```text
    ///                     γ:12
    ///                 █████████████
    ///                 █           █
    ///                 █           █
    ///                 █           █
    ///              θ:8█           █φ:12
    ///                 █           █
    ///                 █           █
    ///         ι:12    █           █     Ι:6
    ///     █████████████           █   ███████
    ///     █                       █   █     █
    ///     █                       █Ϊ:8█     █
    /// ϊ:12█                       █   █ε:8  █Θ:6
    ///     █                       █████████ █
    ///  μ:8█                      Γ:12 █   █ █
    /// █████████                 █████████████
    /// █   █   █                 █     █   █
    /// █   █   █                 █     █   █
    /// █   █   █λ:6              █  α:4    █
    /// █   █   █                 █ ████^   █
    /// █   █   █             Φ:12█ █       █
    /// █   █████                 █ █       █δ:12
    /// █    κ:4                  █ █β:6    █
    /// █                         █ █       █
    /// █ν:12                     █ █       █
    /// █                         █ █████████
    /// █  ο:8            Ε:12    █    ξ:8
    /// █████████     █████████████
    ///         █Α:6  █
    ///       ███████ █
    ///       █ █   █ █
    ///       █ █   █ █
    ///       █ █   █ █Δ:8
    ///    Β:6█ █   █ █
    ///       █ █   █ █
    ///       █████████
    ///         █Ξ:8█
    ///         █   █ζ:12
    ///     π:12█   █   ψ:12
    ///         █████████████
    ///             █       █
    ///             █████████████
    ///              η:12   █   █
    ///                     █   █χ:8
    ///                     █υ:8█
    ///                   █████████
    ///                   █ █   █ █
    ///                ϋ:4█ █   █ █
    ///                   █ █   █ █τ:6
    ///                   ███████ █
    ///                 ρ:12█ ω:6 █
    ///                     ███████
    ///                       σ:6
    ///
    /// L,4,L,6,L,8,L,12,L,8,R,12,L,12,L,8,R,12,L,12,L,4,L,6,L,8,L,12,L,8,R,12,L,12,R,12,L,6,L,6,L,8,L,4,L,6,L,8,L,12,R,12,L,6,L,6,L,8,L,8,R,12,L,12,R,12,L,6,L,6,L,8
    ///  α   β   ξ   δ    ε   φ    γ    θ   ι    ϊ    κ   λ   μ   ν    ο   π    ψ    ρ    σ   τ   υ   ϋ   ω   χ   η    ζ    Α   Β   Ξ   Δ   Ε    Φ    Γ    Θ   Ι   Ϊ
    ///
    /// P: B,C,C,B,C,A,B,A,C,A
    /// A: R,12,L,6,L,6,L,8
    /// B: L,4,L,6,L,8,L,12
    /// C: L,8,R,12,L,12
    /// ```
    fn two(mut self) -> i64 {
        self.0[0] = 2;

        println!("\x1B[?25l");

        let mut buffer = String::new();
        let mut clear = false;
        let mut input = ascii("B,C,C,B,C,A,B,A,C,A\n")
            .chain(ascii("R,12,L,6,L,6,L,8\n"))
            .chain(ascii("L,4,L,6,L,8,L,12\n"))
            .chain(ascii("L,8,R,12,L,12\n"))
            .chain(ascii("y\n"));

        loop {
            use intcode::Yield::*;
            match self.0.step() {
                Halt => panic!("halt"),
                Step => (),
                Output(o) if o > 255 => {
                    println!("\x1B[?25h");
                    break o;
                }
                Output(o) if o == 10 && clear => {
                    clear = false;
                    print!("{}", buffer);
                    thread::sleep(time::Duration::from_millis(25));
                    print!("\x1B[2J");
                    buffer.clear();
                }
                Output(o) => {
                    clear = o == 10;
                    buffer.push(o as u8 as char);
                }
                Input(i) => {
                    self.0[i] = input.give();
                }
            }
        }
    }
}

fn ascii(string: &str) -> impl Iterator<Item = i64> + '_ {
    string.chars().map(|c| c as u8).map(|c| c as i64)
}
