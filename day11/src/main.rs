use std::fs::File;
use std::io::{BufRead, BufReader};

const GRID_SIZE: isize = 10;
type Grid = Vec<Vec<u8>>;

fn main() {
    let f = File::open("input.txt").expect("Can't open input.txt");
    let reader = BufReader::new(f);
    let lines = reader.lines().map(|l| l.unwrap());
    let mut grid: Grid = lines.map(line_to_digits_vec).collect();

    let mut flashes = 0;
    let mut sim_flash_done = false;

    for i in 1.. {
        // inc all
        grid.iter_mut().flatten().for_each(|v| *v += 1);

        let mut flashes_step = 0;
        
        // flash propagation
        loop {
            let mut new_flashes = false;

            for (x, y) in full_2d_range(&grid) {
                if grid[y][x] > 9 {
                    flashes_step += 1;
                    new_flashes = true;
                    grid[y][x] = 0; // do not check again next iteration

                    let x = x as isize;
                    let y = y as isize;
                    for (x, y) in safe_2d_range((x - 1, y - 1), (x + 1, y + 1), (GRID_SIZE, GRID_SIZE)) {
                        if grid[y][x] > 0 { // those with 0 already flashed
                            grid[y][x] += 1;
                        }
                    }
                }
            }

            if !new_flashes {
                break;
            }
        }

        flashes += flashes_step;

        if i == 100 {
            println!("Part 1: flashes={}", flashes);
        }
        if flashes_step == 100 {
            println!("Part 2: steps={}", i);
            sim_flash_done = true;
        }
        if sim_flash_done && i >= 100 {
            break;
        }
    }
}



fn full_2d_range(grid: &Grid) -> impl Iterator<Item = (usize, usize)> {
    let p0 = (0, 0);
    let p1 = (grid[0].len() - 1, grid.len() - 1);
    unchecked_2d_range(p0, p1)
}

fn safe_2d_range(p0: (isize, isize), p1: (isize, isize), size: (isize, isize))
                 -> impl Iterator<Item = (usize, usize)> {
    let (mut x0, mut x1) = min_max(p0.0, p1.0);
    let (mut y0, mut y1) = min_max(p0.1, p1.1);
    let (x_len, y_len) = size;

    for v in [&mut x0, &mut x1] {
        if *v < 0 { *v = 0; }
        if *v >= x_len { *v = x_len - 1; }
    }
    for v in [&mut y0, &mut y1] {
        if *v < 0 { *v = 0; }
        if *v >= y_len { *v = y_len - 1; }
    }
    
    unchecked_2d_range((x0 as usize, y0 as usize), (x1 as usize, y1 as usize))
}

fn unchecked_2d_range((x0, y0): (usize, usize), (x1, y1): (usize, usize))
                       -> impl Iterator<Item = (usize, usize)> {
    (x0..=x1).flat_map(move |x| (y0..=y1).map(move |y| (x, y)))
}

fn min_max(a: isize, b: isize) -> (isize, isize) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

fn line_to_digits_vec(line: String) -> Vec<u8> {
    line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
}
