use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Eq, PartialEq, Copy, Clone)]
enum Dir {
    East, South, None
}

fn main() {
    let mut grid = parse_input("input.txt");
    let rows_cnt = grid.len();
    let cols_cnt = grid[0].len();

    let mut stalled = false;
    let mut step = 0;
    while !stalled {
        stalled = true;
        step += 1;

        for i in 0..rows_cnt {
            let (mut j, end) = match (grid[i][0], grid[i][cols_cnt - 1]) {
                (Dir::None, Dir::East) => {
                    grid[i][0] = Dir::East;
                    grid[i][cols_cnt - 1] = Dir::None;
                    stalled = false;
                    (2, cols_cnt - 1)
                },
                _ => (1, cols_cnt),
            };
            
            while j < end {
                if grid[i][j] == Dir::None && grid[i][j - 1] == Dir::East {
                    grid[i][j] = Dir::East;
                    grid[i][j - 1] = Dir::None;
                    stalled = false;
                    j += 1;
                }
                j += 1;
            }
        }

        for j in 0..cols_cnt {
            let (mut i, end) = match (grid[0][j], grid[rows_cnt - 1][j]) {
                (Dir::None, Dir::South) => {
                    grid[0][j] = Dir::South;
                    grid[rows_cnt - 1][j] = Dir::None;
                    stalled = false;
                    (2, rows_cnt - 1)
                },
                _ => (1, rows_cnt),
            };

            while i < end {
                if grid[i][j] == Dir::None && grid[i -1][j] == Dir::South {
                    grid[i][j] = Dir::South;
                    grid[i -1][j] = Dir::None;
                    stalled = false;
                    i += 1;
                }
                i += 1;
            }
        }
    }

    println!("Steps={}", step);
}

fn parse_input(filename: &str) -> Vec<Vec<Dir>> {
    let f = File::open(filename).expect(&format!("Error opening {}", filename));
    let reader = BufReader::new(f);
    let lines = reader.lines().map(|l| l.unwrap());

    let mut grid = Vec::new();
    for line in lines {
        grid.push(line.chars().map(|ch| {
            match ch {
                '>' => Dir::East,
                'v' => Dir::South,
                _ => Dir::None,
            }
        }).collect());
    }
    grid
}
