use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("input.txt").expect("Can't open input.txt");
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    let mut positions = line.trim().split(',')
        .map(parse_num)
        .collect::<Vec<i32>>();
    positions.sort();

    part1(&positions);
    part2(&positions);
}

fn part1(positions: &[i32]) {
    let med = median(&positions);

    let mut sum = 0;
    for pos in positions {
        sum += i32::abs(med - pos);
    }
        
    println!("Print 1: aligning pos={}, fuel={}", med, sum);
}

fn part2(positions: &[i32]) {
    let max_dist = *positions.iter().last().unwrap();
    let fuel_usages = create_fuel_usages_table(max_dist);

    // brute force :-(
    let mut min = (-1, std::i32::MAX);
    for pos_guess in 0..=max_dist {
        let mut sum = 0;
        for pos in positions {
            sum += fuel_usages[i32::abs(pos_guess - pos) as usize];
        }
        if sum < min.1 {
            min = (pos_guess, sum);
        }
    }
        
    println!("Print 2: aligning pos={}, fuel={}", min.0, min.1);
}

fn parse_num(num_str: &str) -> i32 {
    num_str.parse().unwrap()
}

fn median(positions: &[i32]) -> i32 {
    let len = positions.len();
    if len % 2 == 1 {
        positions[len/2]
    } else {
        (positions[len/2 - 1] + positions[len/2]) / 2
    }
}

fn create_fuel_usages_table(max_dist: i32) -> Vec<i32> {
    let mut table = Vec::with_capacity(1 + max_dist as usize);
    
    let mut prev = 0;
    for dist in 0..=max_dist {
        prev += dist;
        table.push(prev);
    }

    table
}
