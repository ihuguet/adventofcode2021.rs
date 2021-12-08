use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("input.txt").expect("Can't open input.txt");
    let reader = BufReader::new(f);
    let lines = reader.lines().map(|l| l.unwrap());

    let mut part1_count = 0;
    let mut outs_sum = 0;

    for line in lines {
        let mut split = line.split(" | ");
        let all_digits: Vec<&str> = split.next().unwrap().split_whitespace().collect();
        let out_digits: Vec<&str> = split.next().unwrap().split_whitespace().collect();
        
        // part 1 counts
        for digit in &out_digits {
            match digit.len() {
                2 | 4 | 3 | 7 => part1_count += 1,
                _ => (),
            }
        }

        let mut solved_digits: [String;10] = Default::default();

        // identify unique digits
        for digit in &all_digits {
            let idx = match digit.len() {
                2 => 1,
                4 => 4,
                3 => 7,
                7 => 8,
                _ => continue,
            };
            solved_digits[idx].push_str(digit);
        }

        // identify digits that can be deduced from the already known ones
        for digit in &all_digits {
            if digit.len() == 5 {
                if digit.has_all_chars_in_str(&solved_digits[1]) {
                    solved_digits[3].push_str(digit);
                }
            }
            else if digit.len() == 6 {
                if digit.has_all_chars_in_str(&solved_digits[4]) {  // only 9 has all segments from 4
                    solved_digits[9].push_str(digit);
                } else if digit.has_all_chars_in_str(&solved_digits[1]) { // only 0 has all segments from 1 but not from 4
                    solved_digits[0].push_str(digit);
                } else { // only 6 doesn't match with previous conditions
                    solved_digits[6].push_str(digit);
                }
            }
        }

        // identify the remaining ones (2, 5)
        for digit in all_digits {
            if digit.len() == 5 {
                if digit == &solved_digits[3] {
                    continue;
                } else if solved_digits[6].as_str().has_all_chars_in_str(digit) { // only 5 has all its segments also in 6
                    solved_digits[5].push_str(digit);
                } else { // only 2 doesn't have all segments from 6
                    solved_digits[2].push_str(digit);
                }
            }
        }

        // now we know all digits, get output value
        let mut out = String::new();
        for digit in out_digits {
            let num = solved_digits.iter().position(|s| {
                digit.has_same_chars_than_str(s)
            }).unwrap();
            out.push((b'0' + num as u8) as char);
        }
        let out: i32 = out.parse().unwrap();

        // sum output value to accumulator
        outs_sum += out;
    }

    println!("Part 1: count={}", part1_count);
    println!("Part 2: sum={}", outs_sum);
}

trait CharsMatch {
    fn has_all_chars_in_str(&self, s: &str) -> bool;
    fn has_same_chars_than_str(&self, s: &str) -> bool;
}

impl CharsMatch for &str {
    fn has_all_chars_in_str(&self, s: &str) -> bool {
        s.chars().all(|c| self.contains(c))
    }

    fn has_same_chars_than_str(&self, s: &str) -> bool {
        self.len() == s.len() && self.has_all_chars_in_str(s)
    }
}
