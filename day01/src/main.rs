use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;

fn main() {
    let f = File::open("input.txt").unwrap();
    let f = BufReader::new(f);
    let lines = f.lines().map(|l| l.unwrap());

    let mut vals: Vec<i32> = Vec::new();
    for line in lines {
        vals.push(line.parse().unwrap());
    }

    part1(&vals);
    part2(&vals);
}

fn part1(vals: &Vec<i32>) {
    let mut prev = None;
    let mut increases = 0;
    for val in vals {
        if let Some(prev) = prev {
            if val > prev {
                increases += 1;
            }
        }
        prev = Some(val);
    }

    println!("Part 1: value is increased {} times", increases);
}

fn part2(vals: &Vec<i32>) {
    let mut prev = None;
    let mut increases = 0;
    let mut buf = VecDeque::new();
    for &val in vals {
        buf.push_back(val);

        if buf.len() > 3 {
            buf.pop_front();
        } else if buf.len() < 3{
            continue;
        }

        let val: i32 = buf.iter().sum();
        if let Some(prev) = prev {
            if val > prev {
                increases += 1;
            }
        }
        prev = Some(val);
    }

    println!("Part 2: value is increased {} times", increases);
}