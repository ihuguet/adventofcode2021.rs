use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy)]
enum Action { On, Off }
type Cuboid = ((i32, i32), (i32, i32), (i32, i32));
const RANGE_PART1: Cuboid = ((-50, 50), (-50, 50), (-50, 50));

fn main() {
    let actions = parse_input_actions("input.txt");
    let actions_part1 = actions
        .iter()
        .map(|(action, cuboid)| (*action, reduce_cuboid_to_range(cuboid, &RANGE_PART1)))
        .collect();

    let cubes_on = solve(&actions_part1);
    println!("Part 1: cubes on count={}", cubes_on);

    let cubes_on = solve(&actions);
    println!("Part 2: cubes on count={}", cubes_on);
}

fn solve(actions: &Vec<(Action, Cuboid)>) -> u64 {
    let mut cubes_on = 0u64;

    for i in 0..actions.len() {
        let (action, cuboid) = actions[i];
        let (prev_on, prev_off) = count_cubes_on_off(&actions[..i], &cuboid);
        match action {
            Action::On => cubes_on += prev_off,
            Action::Off => cubes_on -= prev_on,
        }
    }

    cubes_on
}

fn count_cubes_on_off(prev_actions: &[(Action, Cuboid)], range: &Cuboid) -> (u64, u64) {
    let mut cubes_on = 0;

    for i in 0..prev_actions.len() {
        let (action, cuboid) = &prev_actions[i];
        let cuboid = reduce_cuboid_to_range(cuboid, range);

        if count_cubes(&cuboid) == 0 {
            continue;
        }

        let (prev_on, prev_off) = count_cubes_on_off(&prev_actions[..i], &cuboid);
        match action {
            Action::On => cubes_on += prev_off,
            Action::Off => cubes_on -= prev_on,
        }
    }
    
    let cubes_num = count_cubes(range);
    (cubes_on, cubes_num - cubes_on)
}

fn reduce_cuboid_to_range(cuboid: &Cuboid, range: &Cuboid) -> Cuboid {
    (
        (cuboid.0.0.max(range.0.0), cuboid.0.1.min(range.0.1)),
        (cuboid.1.0.max(range.1.0), cuboid.1.1.min(range.1.1)),
        (cuboid.2.0.max(range.2.0), cuboid.2.1.min(range.2.1))
    )
}

fn count_cubes(cuboid: &Cuboid) -> u64 {
    if cuboid.0.1 < cuboid.0.0 || cuboid.1.1 < cuboid.1.0 || cuboid.2.1 < cuboid.2.0 {
        0
    } else {
        (cuboid.0.1 - cuboid.0.0 + 1) as u64 
        * (cuboid.1.1 - cuboid.1.0 + 1) as u64
        * (cuboid.2.1 - cuboid.2.0 + 1) as u64
    }
}

fn parse_input_actions(filename: &str) -> Vec<(Action, Cuboid)> {
    let f = File::open(filename).expect(&format!("Error opening {}", filename));
    let reader = BufReader::new(f);
    reader.lines().map(|l| parse_line(&l.unwrap())).collect()
}

fn parse_line(line: &str) -> (Action, Cuboid) {
    let (action, ranges) = match &line[..3] {
        "on " => (Action::On, &line[3..]),
        "off" => (Action::Off, &line[4..]),
        _ => panic!("Action should be 'on' or 'off'"),
    };
    let mut ranges = ranges.split(',').map(|s| parse_range(s));
    let ranges = (ranges.next().unwrap(), ranges.next().unwrap(), ranges.next().unwrap());

    (action, ranges)
}

fn parse_range(range: &str) -> (i32, i32) {
    let mut nums_iter = range[2..].split("..").map(|n| n.parse::<i32>().unwrap());
    let a = nums_iter.next().unwrap();
    let b = nums_iter.next().unwrap();
    (a.min(b), a.max(b))
}
