use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::VecDeque;
use std::fmt;

#[derive(Clone, PartialEq)]
enum Token {
    Open, Close, Num(u8)
}

fn main() {
    let f = File::open("input.txt").expect("Error opening input.txt");
    let reader = BufReader::new(f);
    let lines = reader.lines().map(|l| l.unwrap());

    let mut nums = Vec::new();
    for line in lines {
        nums.push(parse_line(&line));
    }

    part1(&nums);
    part2(&nums);
}

fn part1(nums: &Vec<VecDeque<Token>>) {
    let mut result = nums[0].clone();

    for num in &nums[1..] {
        result = sum(&result, num);
    }

    result.make_contiguous();
    let mag = magnitude(result.as_slices().0);

    println!("Part 1: magnitude={}", mag);
}

fn part2(nums: &Vec<VecDeque<Token>>) {
    let mut max = 0;

    for i in 0..nums.len() {
        for j in 0..nums.len() {
            let mut result = sum(&nums[i], &nums[j]);
            result.make_contiguous();
            let mag = magnitude(result.as_slices().0);
            if mag > max {
                max = mag;
            }
        }
    }

    println!("Part 2: max magnitude={}", max);
}

fn sum(tokens1: &VecDeque<Token>, tokens2: &VecDeque<Token>) -> VecDeque<Token> {
    let mut result = VecDeque::with_capacity(2 + tokens1.len() + tokens2.len());
    result.push_front(Token::Open);
    result.extend(tokens1.clone());
    result.extend(tokens2.clone());
    result.push_back(Token::Close);

    loop {
        let explodes = reduce_all_explodes(&mut result);
        let splits = reduce_one_split(&mut result);
        if !explodes && !splits {
            break;
        }
    }

    result
}

fn reduce_all_explodes(tokens: &mut VecDeque<Token>) -> bool {
    let mut has_explode = false;
    loop {
        let mut lvl = 0;
        let mut new_explode = false;
        for i in 0..tokens.len() {
            match tokens[i] {
                Token::Open => lvl += 1,
                Token::Close if lvl > 4 => {
                    explode(tokens, i);
                    new_explode = true;
                    break;
                },
                Token::Close => lvl -= 1,
                _ => (),
            }
        }
        if new_explode {
            has_explode = true;
        } else {
            break;
        }
    }
    has_explode
}

fn explode(tokens: &mut VecDeque<Token>, end: usize) {
    let src_left = match tokens[end - 2] {
        Token::Num(n) => n,
        _ => panic!(),
    };
    let src_right = match tokens[end - 1] {
        Token::Num(n) => n,
        _ => panic!(),
    };

    if let Some((dst_left_pos, dst_left)) = find_prev_num(tokens, end - 3) {
        tokens[dst_left_pos] = Token::Num(src_left + dst_left);
    }
    if let Some((dst_right_pos, dst_right)) = find_next_num(tokens, end) {
        tokens[dst_right_pos] = Token::Num(src_right + dst_right);
    }

    tokens.remove(end);
    tokens.remove(end - 1);
    tokens.remove(end - 2);
    tokens[end - 3] = Token::Num(0);
}

fn reduce_one_split(tokens: &mut VecDeque<Token>) -> bool {
    // assumed no explodes must be done
    for i in 0..tokens.len() {
        match tokens[i] {
            Token::Num(n) if n > 9 => {
                tokens[i] = Token::Open;
                tokens.insert(i + 1, Token::Num(n / 2));
                tokens.insert(i + 2, Token::Num(n / 2 + n % 2));
                tokens.insert(i + 3, Token::Close);
                return true;
            },
            _ => (),
        }
    }
    false
}

fn find_prev_num(tokens: &mut VecDeque<Token>, pos: usize) -> Option<(usize, u8)> {
    for pos in (1..pos).rev() {
        if let Token::Num(n) = tokens[pos] {
            return Some((pos, n));
        }
    }
    None
}

fn find_next_num(tokens: &mut VecDeque<Token>, pos: usize) -> Option<(usize, u8)> {
    for pos in pos + 1..tokens.len() - 1 {
        if let Token::Num(n) = tokens[pos] {
            return Some((pos, n));
        }
    }
    None
}

fn magnitude(tokens: &[Token]) -> u32 {
    let mut pos = 1;
    let v1 = next_pair_val(tokens, &mut pos);
    let v2 = next_pair_val(tokens, &mut pos);
    3 * v1 + 2 * v2
}

fn next_pair_val(tokens: &[Token], pos: &mut usize) -> u32 {
    if let Token::Num(n) = tokens[*pos] {
        *pos += 1;
        n as u32
    } else {
        let start = *pos;
        *pos += find_pair_close(&tokens[start..]) + 1;
        magnitude(&tokens[start..*pos])
    }
}

fn find_pair_close(tokens: &[Token]) -> usize {
    let mut lvl = 0;
    for i in 0..tokens.len() {
        match tokens[i] {
            Token::Open => lvl += 1,
            Token::Close if lvl > 1 => lvl -= 1,
            Token::Close => return i,
            _ => (),
        }
    }
    0
}

fn parse_line(line: &str) -> VecDeque<Token> {
    let mut tokens = VecDeque::new();
    let mut num_buf = String::new();
    for ch in line.chars() {
        match ch {
            '[' => tokens.push_back(Token::Open),
            ch @ '0'..='9' => num_buf.push(ch),
            ch @ (']' | ',') => {
                if num_buf.len() > 0 {
                    let num = num_buf.parse().unwrap();
                    tokens.push_back(Token::Num(num));
                    num_buf.clear();
                }
                if ch == ']' {
                    tokens.push_back(Token::Close);
                }
            },
            _ => panic!(),
        }
    }
    tokens
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Token::Open => write!(f, "["),
            Token::Close =>  write!(f, "]"),
            Token::Num(n) => write!(f, "{} ", n),
        }
    }
}

impl fmt::Debug for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let tokens = parse_line("[[1,2],[[3,4],5]]");

        let expected = VecDeque::from(vec![
            Token::Open,
                Token::Open, Token::Num(1), Token::Num(2), Token::Close,
                Token::Open,
                    Token::Open, Token::Num(3), Token::Num(4), Token::Close,
                    Token::Num(5),
                Token::Close,
            Token::Close
        ]);
        assert_eq!(tokens, expected);
    }

    #[test]
    fn magnitude_test() {
        let mut tokens = parse_line("[[1,2],[[3,4],5]]");
        tokens.make_contiguous();
        assert_eq!(magnitude(tokens.as_slices().0), 143);
    }
}