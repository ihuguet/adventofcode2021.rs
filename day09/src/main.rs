use std::fs::File;
use std::io::{BufRead, BufReader};

type HeightsMap = Vec<Vec<u8>>;

fn main() {
    let f = File::open("input.txt").expect("Can't open input.txt");
    let reader = BufReader::new(f);
    let lines = reader.lines().map(|l| l.unwrap());
    let heights: HeightsMap = lines.map(line_to_digits_vec).collect();

    let x_len = heights[0].len();
    let y_len = heights.len();

    let mut risk = 0;
    let mut basins = Vec::new();
    for y in 0..y_len {
        for x in 0..x_len {
            if is_low_point(&heights, (x, y)) {
                risk += 1 + heights[y][x] as u32;
                basins.push(get_basin_size(heights.clone(), (x, y)));
            }
        }
    }

    basins.sort();
    let mult = basins.iter().rev().take(3).fold(1, |prev, val| prev * val);

    println!("Part 1: risk={}", risk);
    println!("Part 2: mult={}", mult);
}

fn is_low_point(heights: &HeightsMap, (x, y): (usize, usize)) -> bool {
    let x_max = heights[0].len() - 1;
    let y_max = heights.len() - 1;
    let height = heights[y][x];

    if x > 0 && heights[y][x - 1] <= height { return false; }
    if y > 0 && heights[y - 1][x] <= height { return false; }
    if x < x_max && heights[y][x + 1] <= height { return false; }
    if y < y_max && heights[y + 1][x] <= height { return false; }

    true
}

fn get_basin_size(mut heights: HeightsMap, (x, y): (usize, usize)) -> u32 {
    count_basin_points_from(&mut heights, (x, y))
}

fn count_basin_points_from(heights: &mut HeightsMap, (x, y): (usize, usize)) -> u32 {
    let x_max = heights[0].len() - 1;
    let y_max = heights.len() - 1;

    // sum current point and change it for height = 9 so it's not checked again
    let mut sum = 1;
    heights[y][x] = 9;

    // sum recursively adjacent points
    if x > 0 && heights[y][x - 1] != 9 {
        sum += count_basin_points_from(heights, (x - 1, y));
    }
    if y > 0 && heights[y - 1][x] != 9 {
        sum += count_basin_points_from(heights, (x, y - 1));
    }
    if x < x_max && heights[y][x + 1] != 9 {
        sum += count_basin_points_from(heights, (x + 1, y));
    }
    if y < y_max && heights[y + 1][x] != 9 {
        sum += count_basin_points_from(heights, (x, y + 1));
    }

    sum
}

fn line_to_digits_vec(line: String) -> Vec<u8> {
    line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
}