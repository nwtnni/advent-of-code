const INPUT: &'static str = include_str!("input.txt");

const GRID_SIZE: usize = 300;

fn cell_iter(cell_size: usize, x: usize, y: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..cell_size)
        .map(move |dx| x + dx)
        .flat_map(move |x| {
            (0..cell_size).map(move |dy| (x, y + dy))
        })
}

fn main() {
    
    let serial = INPUT.trim().parse::<isize>().unwrap();
    let mut power = vec![vec![0; GRID_SIZE]; GRID_SIZE];

    for y in 1..=GRID_SIZE {
        for x in 1..=GRID_SIZE {
            let rack_id = 10 + x as isize;
            let p1 = rack_id * y as isize;
            let p2 = p1 + serial; 
            let p3 = p2 * rack_id;
            if p3 < 100 {
                power[y - 1][x - 1] = -5;
            } else {
                power[y - 1][x - 1] = ((p3 % 1000) / 100) - 5;
            }
        }
    }

    let (mut max_x, mut max_y) = (0, 0);
    let mut max_power = 0;
    let mut max_size = 0;

    for cell_size in 0..300 {
        println!("{}", cell_size);
        for y in 1..=GRID_SIZE - cell_size {
            for x in 1..=GRID_SIZE - cell_size {
                let power = cell_iter(cell_size, x, y)
                    .map(|(cx, cy)| power[cy - 1][cx - 1])
                    .sum::<isize>();

                if power > max_power {
                    max_x = x;
                    max_y = y;
                    max_power = power;
                    max_size = cell_size;
                }
            }
        }

    }

    println!("{},{},{}", max_x, max_y, max_size);
}
