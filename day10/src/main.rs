use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("input.txt").expect("Can't open input.txt");
    let reader = BufReader::new(f);

    let mut errors_score = 0;
    let mut completions_scores = Vec::new();

    for line in reader.lines() {
        match validate_line(line.unwrap()) {
            Err(syntax_err) => {
                errors_score += syntax_err.ch.error_score();
            },
            Ok(completion_str) => {
                completions_scores.push(calc_completion_score(&completion_str));
            }
        }
    }

    completions_scores.sort_unstable();
    let middle_score = completions_scores[completions_scores.len() / 2];

    println!("Part 1: syntax errors score = {}", errors_score);
    println!("Part 2: middle completion score = {}", middle_score);
}

fn validate_line(line: String) -> SyntaxResult {
    let mut stack = Vec::new();
    for ch in line.chars() {
        match ch {
            '(' | '[' | '{' | '<' => {
                stack.push(ch);
            }
            ')' | ']' | '}' | '>' => {
                let open_ch = stack.pop().ok_or(SyntaxErr::from_char(ch))?;
                let close_ch = CloseCh::new(ch);
                if close_ch.open_char() != open_ch {
                    return Err(SyntaxErr::new(close_ch));
                }
            }
            c => panic!("Unknown closing char {}", c),
        }
    }

    let completion = stack.iter().rev().map(|c| {
        CloseCh::from_open_char(*c)
    }).collect();

    Ok(completion)
}

fn calc_completion_score(completion_str: &Vec<CloseCh>) -> u64 {
    completion_str.iter().fold(0, |prev, val| {
        prev * 5 + val.autocomplete_score() as u64
    })
}

struct CloseCh {
    ch: char,
}

struct SyntaxErr {
    ch: CloseCh,
}

impl CloseCh {
    fn new(ch: char) -> Self {
        Self { ch }
    }

    fn from_open_char(open_ch: char) -> Self {
        let ch = match open_ch {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            c => panic!("Unknown opening char {}", c),
        };
        Self { ch }
    }

    fn open_char(&self) -> char {
        self.data().0
    }

    fn autocomplete_score(&self) -> u32 {
        self.data().1
    }

    fn error_score(&self) -> u32 {
        self.data().2
    }

    fn data(&self) -> (char, u32, u32) {
        match self.ch {
            ')' => ('(', 1, 3),
            ']' => ('[', 2, 57),
            '}' => ('{', 3, 1197),
            '>' => ('<', 4, 25137),
            c => panic!("Unknown closing char {}", c),
        }
    }
}

impl SyntaxErr {
    fn new(ch: CloseCh) -> Self {
        SyntaxErr { ch }
    }

    fn from_char(ch: char) -> Self {
        SyntaxErr { ch: CloseCh::new(ch) }
    }
}

type SyntaxResult = Result<Vec<CloseCh>, SyntaxErr>;
