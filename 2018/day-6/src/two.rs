extern crate day_6;

use day_6::*;

const MAX_DISTANCE: isize = 10000;

fn main() {
    let points = INPUT.trim()
        .split('\n')
        .map(Point::parse)
        .collect::<Vec<_>>();

    let mut size = 0;
    for x in -MAX_DISTANCE..MAX_DISTANCE {
        for y in -MAX_DISTANCE..MAX_DISTANCE {
            let q = Point { x, y };
            let d = points.iter()
                .map(|p| dist(*p, q))
                .sum::<isize>();
            if d < MAX_DISTANCE {
                size += 1;
            }
        }
    }

    println!("{}", size);
}
