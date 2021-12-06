use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("input.txt").expect("Can't open input.txt");
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();

    let mut counts = [0u64;9]; // index=timer, value=num of fishes with that timer
    line.trim().split(',')
        .map(parse_num)
        .for_each(|n| counts[n] += 1);
    
    for i in 0..256 {
        counts.rotate_left(1);
        counts[6] += counts[8];

        if i == 79 {
            let count = counts.iter().sum::<u64>();
            println!("Part 1: count={}", count);
        }
    }

    let count = counts.iter().sum::<u64>();
    println!("Part 2: count={}", count);
}

fn parse_num(num_str: &str) -> usize {
    num_str.parse().unwrap()
}
