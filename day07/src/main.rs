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
    let sum = positions.iter().fold(0, |acc, pos| acc + i32::abs(med - pos));
    println!("Print 1: aligning pos={}, fuel={}", med, sum);
}

fn part2(positions: &[i32]) {
    // brute force :-(
    let max_dist = *positions.iter().last().unwrap();
    let min = (0..=max_dist)
        .map(|pos_guess| {
            let sum = positions.iter().fold(0, |acc, pos| acc + fuel_usage(pos_guess - pos));
            (pos_guess, sum)
        })
        .min_by_key(|a| a.1).unwrap();
        
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

fn fuel_usage(mut dist: i32) -> i32 {
    dist = i32::abs(dist);
    dist * (dist + 1) / 2
}
