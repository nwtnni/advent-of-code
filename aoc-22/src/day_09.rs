use std::collections::HashSet;

use aoc::*;

#[derive(Clone, Debug)]
pub struct RopeBridge(Vec<(Dir, i64)>);

impl Fro for RopeBridge {
    fn fro(input: &str) -> Self {
        input
            .trim()
            .split('\n')
            .map(|line| {
                let (dir, steps) = line.split_once(' ').unwrap();
                let dir = match dir {
                    "R" => Dir::E,
                    "L" => Dir::W,
                    "U" => Dir::N,
                    "D" => Dir::S,
                    _ => unreachable!(),
                };
                (dir, i64::fro(steps))
            })
            .collect::<Vec<_>>()
            .tap(Self)
    }
}

impl Solution for RopeBridge {
    fn one(self) -> i64 {
        let mut head = Pos { x: 0, y: 0 };
        let mut tail = Pos { x: 0, y: 0 };
        let mut visited = HashSet::new();

        for (dir, steps) in self.0 {
            for _ in 0..steps {
                head.shift_mut(dir);

                if head.x - tail.x == 2 && (head.y == tail.y) {
                    tail.shift_mut(Dir::E);
                } else if head.x - tail.x == -2 && (head.y == tail.y) {
                    tail.shift_mut(Dir::W);
                } else if head.y - tail.y == 2 && (head.x == tail.x) {
                    tail.shift_mut(Dir::S);
                } else if head.y - tail.y == -2 && (head.x == tail.x) {
                    tail.shift_mut(Dir::N);
                } else if head.x != tail.x
                    && head.y != tail.y
                    && ((head.x - tail.x).abs() + (head.y - tail.y).abs()) > 2
                {
                    let lr = if head.x > tail.x { Dir::E } else { Dir::W };
                    let ud = if head.y > tail.y { Dir::S } else { Dir::N };
                    tail.shift_mut(lr);
                    tail.shift_mut(ud);
                }

                visited.insert(tail);
                println!("H {},{} T {},{}", head.x, head.y, tail.x, tail.y);
            }
        }
        visited.len() as i64
    }

    fn two(self) -> i64 {
        let mut snake = vec![Pos { x: 0, y: 0 }; 10];
        let mut visited = HashSet::new();

        for (dir, steps) in self.0 {
            for _ in 0..steps {
                snake[0].shift_mut(dir);

                for i in 0..snake.len() - 1 {
                    let (a, b) = snake.split_at_mut(i + 1);
                    let head = &a[a.len() - 1];
                    let tail = &mut b[0];

                    if head.x - tail.x == 2 && (head.y == tail.y) {
                        tail.shift_mut(Dir::E);
                    } else if head.x - tail.x == -2 && (head.y == tail.y) {
                        tail.shift_mut(Dir::W);
                    } else if head.y - tail.y == 2 && (head.x == tail.x) {
                        tail.shift_mut(Dir::S);
                    } else if head.y - tail.y == -2 && (head.x == tail.x) {
                        tail.shift_mut(Dir::N);
                    } else if head.x != tail.x
                        && head.y != tail.y
                        && ((head.x - tail.x).abs() + (head.y - tail.y).abs()) > 2
                    {
                        let lr = if head.x > tail.x { Dir::E } else { Dir::W };
                        let ud = if head.y > tail.y { Dir::S } else { Dir::N };
                        tail.shift_mut(lr);
                        tail.shift_mut(ud);
                    }
                }

                visited.insert(snake[9]);
            }
        }
        visited.len() as i64
    }
}
