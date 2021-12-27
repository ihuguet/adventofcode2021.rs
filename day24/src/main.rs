/// Explanation:
/// 
/// The input is the same 18 instructions repeated 14 times. There only change
/// the value of the 2nd argument in 3 of the lines. I'll call this values
/// `zdiv`, `xadd` and `yadd`. `zdiv` is always 1 or 26.
/// - line 5 is `div z <zdiv>`
/// - line 6 is `add x <xadd>`
/// - line 16 is `add y <yadd>`
/// 
/// First, the program calculate the value `x = z % 26 + xval` with `z` from
/// last iteration. Then, it compares this value with the input digit, and makes
/// different things depending on wether the comparison is true or false.
/// 
/// In the iterations where `zdiv` is 1, `xadd` is always > 9. Since the input
/// is 1-9, the comparison will be always false in this case. When `zdiv` is 26,
/// `xval` is always a small negative number, so it is possible for the
/// comparison to be true or false.
/// 
/// The algorithm then is this one
/// if zdiv == 1:
///     z = z * 26 + INPUT_DIGIT + yval
/// else if zdiv == 26:
///     x = z % 26 + xval
///     if x == INPUT_DIGIT:
///         z = z / 26
///     else:
///         z = z + INPUT_DIGIT + yval
/// 
/// z can't become negative in any iteration, so to achieve that z == 0 in last
/// one, we need `x == INPUT_DIGIT` to be true and `z < 26`.
/// 
/// `yval` is always < 17. This means `(z*26 + INPUT_DIGIT + yval) / 26 == z`
/// 
/// `zdiv == 1` a total of 7 times, so `z * 26` is done 7 times. We need to do
/// `z / 26` at least 7 times in orther to `z == 0` being possible.
/// Since `zdiv == 26` happens just 7 times, all the 7 times z must be divided,
/// and that means that all the 7 times `x == INPUT_DIGIT` must be true.
/// 
/// Then, what we need to do is start with the better value (99,999,999,999,999
/// for part1, 11,111,111,111,111 for part2) and in iterations with `zdiv == 26`
/// find the INPUT_DIGIT that makes `z % 26 + xval == INPUT_DIGIT` true. If it's
/// not possible, adjust previous INPUT_DIGITs to the max or min that make it
/// possible.
/// 
/// Example using my input, for part 1:
/// iter 0 (zdiv == 1): z0 = IN0 + yval0
/// iter 1 (zdiv == 1): z1 = 26 * (IN0 + yval0) + IN1 + yval1
/// iter 2 (zdiv == 1): z2 = 26 * (26 * (IN0 + yval0) + IN1 + yval1) + IN2 + yval2
/// iter 3 (zdiv == 26):
///     x = z2 % 26 + xval3 = IN2 + yval2 + xval3
///     if x <= 9: IN3 = x
///     else: IN3 = 9 and reduce IN2 to make the condition true
///     z3 = z2 / 26 = 26 * (IN0 + yval0) + IN1 + yval1     <-- this is z1!!
/// iter 4 (zdiv == 26):
///     x = z3 % 26 + xval4 = IN1 + yval1 + xval4
///     if x <= 9: IN4 = x
///     else: IN4 = 9 and reduce IN1 to make the condition true
///     z4 = z3 / 26 = IN0 + yval0     <-- this is z0!!
/// 
/// As we can see, we can push results of iterations where `zdiv == 1` into a
/// stack, and pop them for calculations at iterations where `zdiv == 26`

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
        let lines: Vec<String> = lines.by_ref().take(18).collect();
        let zdiv = parse_input_val(&lines, 4);
        let xadd = parse_input_val(&lines, 5);
        let yadd = parse_input_val(&lines, 15);
        values.push((zdiv, xadd, yadd));
    }
    values
}

fn parse_input_val(lines: &Vec<String>, line_num: usize) -> i32 {
    lines[line_num].split(' ')
        .nth(2).unwrap()
        .parse::<i32>().unwrap()
}