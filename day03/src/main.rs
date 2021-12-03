use std::fs::File;
use std::io::{BufRead, BufReader};

enum Bit {
    Zero, One
}

fn main() {
    let f = File::open("input.txt").expect("Can't open input.txt");
    let f = BufReader::new(f);
    let lines = f.lines().map(|l| l.unwrap());

    let mut values = Vec::new();
    for line in lines {
        values.push(i32::from_str_radix(&line, 2).unwrap());
    }

    part1(&values);
    part2(&values);
}

fn part1(values: &Vec<i32>) {
    let mut counts_ones = [0;12];
    let mut counts_zeros = [0;12];
    for &val in values {
        for i in 0..12 {
            match read_bit(val, i) {
                Bit::One => counts_ones[i] += 1,
                Bit::Zero => counts_zeros[i] += 1,
            }
        }
    }

    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..12 {
        if counts_ones[i] > counts_zeros[i] {
            gamma |= 1 << i;
        } else {
            epsilon |= 1 << i;
        }
    }

    println!("Part 1: gamma={}, epsilon={}, mult={}", gamma, epsilon, gamma*epsilon);
}

fn part2(values: &Vec<i32>) {
    let mut o2_vals = values.clone();
    let mut co2_vals = values.clone();

    for i in (0..12).rev() {
        if o2_vals.len() > 1 {
            let (values_bit_1, values_bit_0) = split_by_bit_value(&o2_vals, i);
            o2_vals = if values_bit_1.len() >= values_bit_0.len() {
                values_bit_1
            } else {
                values_bit_0
            };
        }

        if co2_vals.len() > 1 {
            let (values_bit_1, values_bit_0) = split_by_bit_value(&co2_vals, i);
            co2_vals = if values_bit_0.len() <= values_bit_1.len() {
                values_bit_0
            } else {
                values_bit_1
            };
        }
    }
    assert_eq!(o2_vals.len(), 1);
    assert_eq!(co2_vals.len(), 1);

    let o2 = o2_vals[0];
    let co2 = co2_vals[0];

    println!("Part 2: O2={}, CO2={}, mult={}", o2, co2, o2*co2);
}

fn split_by_bit_value(vals: &Vec<i32>, bit_pos: usize) -> (Vec<i32>, Vec<i32>) {
    let mut values_bit_1 = Vec::new();
    let mut values_bit_0 = Vec::new();

    for &val in vals {
        match read_bit(val, bit_pos) {
            Bit::One => values_bit_1.push(val),
            Bit::Zero => values_bit_0.push(val),
        }
    }

    (values_bit_1, values_bit_0)
}

fn read_bit(num: i32, bit_pos: usize) -> Bit {
    let bit = (num >> bit_pos) & 1;
    match bit {
        1 => Bit::One,
        0 => Bit::Zero,
        _ => panic!(),
    }
}