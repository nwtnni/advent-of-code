const INPUT: &'static str = include_str!("input.txt");

const GRID_SIZE: usize = 300;

fn main() {

    let serial = INPUT.trim().parse::<isize>().unwrap();
    let mut power = vec![vec![0; GRID_SIZE]; GRID_SIZE];

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let (xi, yi) = (x as isize + 1, y as isize + 1);
            let rack_id = 10 + xi;
            let p = ((rack_id * yi) + serial) * rack_id;
            let p = if p < 100 { -5 } else { ((p % 1000) / 100) - 5 };

            power[y][x] = p
                + power.get(y - 1).map(|r| r[x]).unwrap_or(0)
                + power[y].get(x - 1).unwrap_or(&0)
                - power.get(y - 1).and_then(|r| r.get(x - 1)).unwrap_or(&0);
        }
    }

    let (mut max_x, mut max_y) = (0, 0);
    let mut max_power = 0;
    let mut max_size = 0;

    for cell_size in 0..300 {
        for y in 1..=GRID_SIZE - cell_size {
            for x in 1..=GRID_SIZE - cell_size {
                let power = power[y + cell_size - 1][x + cell_size - 1]
                    + power[y - 1][x - 1]
                    - power[y + cell_size - 1][x - 1]
                    - power[y - 1][x + cell_size - 1];

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
