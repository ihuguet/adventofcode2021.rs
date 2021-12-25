use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let puzzle_input = parse_input("input.txt");

    let mut max_input = [9; 14];
    let mut min_input = [1; 14];
    let mut stack = Vec::new();

    for i in 0..14 {
        let (zdiv, xadd, yadd) = puzzle_input[i];

        if zdiv == 1 {
            stack.push((i, yadd));
        } else {
            let (i_prev, yadd_prev) = stack.pop().unwrap();

            max_input[i] = max_input[i_prev] + yadd_prev + xadd;
            if max_input[i] > 9 {
                max_input[i_prev] = 9 - (yadd_prev + xadd);
                max_input[i] = 9;
            } 
            
            min_input[i] = min_input[i_prev] + yadd_prev + xadd;
            if min_input[i] < 1 {
                min_input[i_prev] = 1 - (yadd_prev + xadd);
                min_input[i] = 1;
            }
        }
    }

    let input = max_input.iter()
        .map(|n| char::from_digit(*n as u32, 10).unwrap())
        .collect::<String>();
    println!("Part 1: input={}", input);

    let input = min_input.iter()
        .map(|n| char::from_digit(*n as u32, 10).unwrap())
        .collect::<String>();
    println!("Part 2: input={}", input);
}

fn parse_input(filename: &str) -> Vec<(i32, i32, i32)>{
    let f = File::open(filename).expect(&format!("Can't open {}", filename));
    let reader = BufReader::new(f);
    let mut lines = reader.lines().map(|l| l.unwrap());

    let mut values = Vec::new();
    for _ in 0..14 {
        let mut lines: Vec<String> = lines.by_ref().take(18).collect();
        let zdiv = parse_input_val(&mut lines, 4);
        let xadd = parse_input_val(&mut lines, 5);
        let yadd = parse_input_val(&mut lines, 15);
        values.push((zdiv, xadd, yadd));
    }
    values
}

fn parse_input_val(lines: &mut Vec<String>, line_num: usize) -> i32 {
    lines[line_num].split(' ')
        .nth(2).unwrap()
        .parse::<i32>().unwrap()
}