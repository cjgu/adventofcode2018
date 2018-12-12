use multiarray::Array2D;

fn fuel_cell_power(x: i32, y: i32, serial: i32) -> i32 {
    let rack_id = x + 10;
    let power_level = ((rack_id * y) + serial) * rack_id;

    let digit = (power_level / 100) % 10;

    (digit as i32) - 5
}

fn total_power(fuel_cell: &Array2D<i32>, x: usize, y: usize, size: usize) -> i32 {
    let mut sum = 0;
    for i in 0..size {
        for j in 0..size {
            sum += fuel_cell[[x + i, y + j]];
        }
    }

    sum
}

fn main() {
    let serial = 8772;

    let mut fuel_cell = Array2D::new([300, 300], 0i32);

    // Calculate power levels
    for x in 0..300 {
        for y in 0..300 {
            fuel_cell[[x as usize, y as usize]] = fuel_cell_power(x, y, serial);
        }
    }

    // Find max total power in 3x3
    let mut max = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    for x in 0usize..(300 - 2) {
        for y in 0usize..(300 - 2) {
            let total_power = total_power(&fuel_cell, x, y, 3);

            if total_power > max {
                max = total_power;
                max_x = x;
                max_y = y;
            }
        }
    }

    println!("Part 1: X,Y = {},{}. Power = {}", max_x, max_y, max);

    let mut max = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut max_size = 1;

    for size in 1usize..300 + 1 {
        for x in 0usize..(300 - size + 1) {
            for y in 0usize..(300 - size + 1) {
                let total_power = total_power(&fuel_cell, x, y, size);

                if total_power > max {
                    max = total_power;
                    max_x = x;
                    max_y = y;
                    max_size = size;
                }
            }
        }
        println!("Working on size {}, current max: {}", size, max);
    }

    println!(
        "Part 2: X,Y,Size = {},{},{}. Power = {}",
        max_x, max_y, max_size, max
    );
}
