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

        self.0
            .into_iter()
            .flat_map(|(dir, steps)| (0..steps).map(move |_| dir))
            .map(|dir| {
                head.shift_mut(dir);
                chase(&head, &mut tail);
                tail
            })
            .collect::<HashSet<_>>()
            .len() as i64
    }

    fn two(self) -> i64 {
        let mut snake = vec![Pos { x: 0, y: 0 }; 10];

        self.0
            .into_iter()
            .flat_map(|(dir, steps)| (0..steps).map(move |_| dir))
            .map(|dir| {
                snake[0].shift_mut(dir);

                for i in 0..snake.len() - 1 {
                    let (left, right) = snake.split_at_mut(i + 1);
                    chase(&left[i], &mut right[0]);
                }

                snake[9]
            })
            .collect::<HashSet<_>>()
            .len() as i64
    }
}

fn chase(head: &Pos, tail: &mut Pos) {
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
        tail.shift_mut(if head.x > tail.x { Dir::E } else { Dir::W });
        tail.shift_mut(if head.y > tail.y { Dir::S } else { Dir::N });
    }
}
