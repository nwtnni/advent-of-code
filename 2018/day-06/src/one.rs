extern crate day_06;

use std::collections::HashMap as Map;

use day_06::*;

fn main() {
    let points = INPUT.trim()
        .split('\n')
        .enumerate()
        .map(|(i, s)| (i, Point::parse(s)))
        .collect::<Map<usize, Point>>();

    let (mut max_x, mut max_y) = (0, 0);
    let (mut min_x, mut min_y) = (isize::max_value(), isize::max_value());

    for point in points.values() {
        max_x = isize::max(point.x, max_x);
        max_y = isize::max(point.y, max_y);
        min_x = isize::min(point.x, min_x);
        min_y = isize::min(point.y, min_y);
    }

    let dy = (max_y - min_y) as usize;
    let dx = (max_x - min_x) as usize;

    let mut grid = vec![vec![(isize::max_value(), None); dx]; dy];

    for (id, p1) in &points {
        for yi in 0..dy {
            let y = yi as isize + min_y;
            for xi in 0..dx {
                let x = xi as isize + min_x;
                let p = Point { x, y };
                let d1 = dist(p, *p1);
                let d2 = grid[yi][xi].0;
                if d1 < d2 {
                    grid[yi][xi] = (d1, Some(id)); 
                } else if d1 == d2 {
                    grid[yi][xi].1 = None; 
                }
            }
        }
    }

    for row in &grid {
        for col in row {
            if let Some(id) = col.1 {
                print!("{:2} ", id);
            } else {
                print!("   ");
            }
        }
        println!("");
    }

    let mut score: Map<usize, usize> = Map::default();

    for row in &grid {
        for (_, closest) in row {
            if let Some(id) = closest {
                score.entry(**id)
                    .and_modify(|n| *n += 1)
                    .or_insert(1);
            }
        }
    }

    let dx = dx as usize;
    let dy = dy as usize;

    for yi in 0..dy {
        if let Some(id) = grid[yi][0].1 { score.remove(&id); }
        if let Some(id) = grid[yi][dx - 1].1 { score.remove(&id); }
    }

    for xi in 0..dx {
        if let Some(id) = grid[0][xi].1 { score.remove(&id); }
        if let Some(id) = grid[dy - 1][xi].1 { score.remove(&id); }
    }

    let max = score.iter()
        .max_by_key(|(_, c)| *c)
        .unwrap();

    println!("{:?}", max);
}
