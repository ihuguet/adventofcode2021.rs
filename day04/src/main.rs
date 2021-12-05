use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

struct Board {
    nums: Vec<Vec<i32>>,
    checks: u32,
    complete: bool,
}

fn main() {
    let f = File::open("input.txt").expect("Can't open input.txt");
    let reader = BufReader::new(f);
    let mut lines = reader.lines();

    let numbers = parse_numbers(&mut lines);
    let mut boards = parse_boards(&mut lines);

    let mut first = None;
    let mut last = None;

    for num in numbers {
        for (i, board) in boards.iter_mut().enumerate() {
            if board.complete {
                continue;
            }

            if let Some(pos) = board.get_pos(num) {
                board.check(pos);
                if board.is_row_checked(pos.0) || board.is_col_checked(pos.1) {
                    board.complete = true;
                    if first.is_none() {
                        first = Some((i,num));
                    } else {
                        last = Some((i,num));
                    }
                }
            }
        }
    }

    let first = first.take().unwrap();
    let last = last.take().unwrap();

    println!("Part 1: score={}", boards[first.0].score() * first.1);
    println!("Part 2: score={}", boards[last.0].score() * last.1);
}

impl Board {
    fn new(nums: Vec<Vec<i32>>) -> Board {
        Board { nums, checks: 0, complete: false }
    }

    fn get_pos(&self, num: i32) -> Option<(usize,usize)> {
        for (row_i, row) in self.nums.iter().enumerate() {
            for (col_i, &n) in row.iter().enumerate() {
                if n == num {
                    return Some((row_i, col_i));
                }
            }
        }
        None
    }

    fn is_checked(&self, pos: (usize,usize)) -> bool {
        self.checks & (1 << (5*pos.0 + pos.1)) != 0
    }

    fn check(&mut self, pos: (usize,usize)) {
        self.checks |= 1 << (5*pos.0 + pos.1)
    }

    fn is_row_checked(&self, row: usize) -> bool {
        const MASK: u32 = 0b11111;
        (self.checks >> 5*row) & MASK == MASK
    }

    fn is_col_checked(&self, col: usize) -> bool {
        const MASK: u32 = 0b0000100001000010000100001;
        (self.checks >> col) & MASK == MASK
    }

    fn score(&self) -> i32{
        let mut score = 0;
        for (row_i, row) in self.nums.iter().enumerate() {
            for (col_i, num) in row.iter().enumerate() {
                if !self.is_checked((row_i, col_i)) {
                    score += num;
                }
            }
        }
        score
    }
}

fn parse_numbers<T: BufRead>(lines: &mut Lines<T>) -> Vec<i32> {
    lines.next().unwrap().unwrap()
       .split(',')
       .map(|l| l.parse::<i32>().unwrap())
       .collect()
}

fn parse_boards<T: BufRead>(lines: &mut Lines<T>) -> Vec<Board> {
    let mut boards = Vec::new();
    while let Some(board) = parse_board(lines) {
        boards.push(board);
    }
    boards
}

fn parse_board<T: BufRead>(lines: &mut Lines<T>) -> Option<Board> {
    lines.next()?.unwrap();
    
    let mut rows = Vec::new();
    for _ in 0..5 {
        let row = parse_board_line(&lines.next()?.unwrap());
        rows.push(row);
    }

    Some(Board::new(rows))
}

fn parse_board_line(line: &String) -> Vec<i32> {
    line.split_whitespace()
       .map(|l| l.parse::<i32>().unwrap())
       .collect()
}