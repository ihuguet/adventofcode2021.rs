use std::fs::File;
use std::io::{BufRead, BufReader};

enum Dir {
    Fwd, Down, Up
}

struct Mov {
    dir: Dir,
    val: i32,
}

fn main() {
    let f = File::open("input.txt").expect("Can't open input.txt");
    let f = BufReader::new(f);
    let lines = f.lines().map(|l| l.unwrap());

    let mut movs = Vec::new();
    for line in lines {
        movs.push(parse_mov(&line));
    }

    part1(&movs);
    part2(&movs);
}

fn part1(movs: &Vec<Mov>) {
    let mut hor_pos = 0;
    let mut depth = 0;

    for mov in movs {
        match mov.dir {
            Dir::Fwd  => hor_pos += mov.val,
            Dir::Down => depth += mov.val,
            Dir::Up   => depth -= mov.val,
        }
    }
    
    println!("Part 1: pos horizontal={}, depth={}. Mult={}", hor_pos, depth, hor_pos*depth);
}

fn part2(movs: &Vec<Mov>) {
    let mut hor_pos = 0;
    let mut aim = 0;
    let mut depth = 0;

    for mov in movs {
        match mov.dir {
            Dir::Fwd  => {
                hor_pos += mov.val;
                depth += mov.val * aim;
            },
            Dir::Down => aim += mov.val,
            Dir::Up   => aim -= mov.val,
        }
    }
    
    println!("Part 2: pos horizontal={}, depth={}. Mult={}", hor_pos, depth, hor_pos*depth);
}

fn parse_mov(line: &str) -> Mov {
    let mut split = line.split_ascii_whitespace();

    let dir = match split.next() {
        Some("forward") => Dir::Fwd,
        Some("down") => Dir::Down,
        Some("up") => Dir::Up,
        _ => panic!(),
    };

    let val = split.next().unwrap()
                .parse().expect("Failed parsing");
    
    Mov {dir, val}
}
