use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{BinaryHeap, BTreeMap};
use std::cmp::Ordering;

fn main() {
    let grid = parse_input("input.txt");

    let cost = solve(&grid);
    println!("Part 1: cost={}", cost);

    let grid_x5 = get_full_grid(&grid);
    let cost = solve(&grid_x5);
    println!("Part 2: cost={}", cost);
}

fn solve(grid: &Vec<Vec<u8>>) -> u32 {
    let destination = (grid[0].len() - 1, grid.len() - 1);

    let mut visited: BTreeMap<(usize, usize), u32> = BTreeMap::new();
    let mut pri_queue = BinaryHeap::new();
    pri_queue.push(Node { cost: 0, pos: (0, 0), dst: destination });
    let mut min_cost = None;

    while let Some(node) = pri_queue.pop() {
        if node.pos == destination {
            if node.cost < min_cost.unwrap_or(std::u32::MAX) {
                min_cost = Some(node.cost);
            }
            continue;
        }

        if node.best_final_cost() >= min_cost.unwrap_or(std::u32::MAX) {
            continue;
        }
        
        for pos in node.adjacent_positions() {
            let new_cost = node.cost + grid[pos.1 as usize][pos.0 as usize] as u32;
            
            if !visited.contains_key(&pos) || new_cost < visited[&pos] {
                visited.insert(pos, new_cost);
                pri_queue.push(Node {
                    cost: new_cost,
                    pos: pos,
                    dst: destination 
                });
            }
        }
    }

    min_cost.unwrap()
}

fn get_full_grid(grid: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let tile_len = grid.len();
    let full_len = 5 * tile_len;

    let mut grid_x5: Vec<Vec<u8>> = Vec::with_capacity(full_len);
    for _ in 0..full_len {
        grid_x5.push(vec![0; full_len])
    }

    for y in 0..tile_len {
        for x in 0..tile_len {
            grid_x5[y][x] = grid[y][x];
        }
    }

    for y in tile_len..full_len {
        for x in 0..tile_len {
            let mut val = grid_x5[y - tile_len][x] + 1;
            if val > 9 { val = 1; }
            grid_x5[y][x] = val;
        }
    }

    for y in 0..full_len {
        for x in tile_len..full_len {
            let mut val = grid_x5[y][x - tile_len] + 1;
            if val > 9 { val = 1; }
            grid_x5[y][x] = val;
        }
    }

    grid_x5
}

#[derive(Clone, Debug)]
struct Node {
    cost: u32,
    pos: (usize, usize),
    dst: (usize, usize),
}

impl Node {
    fn estimation(&self) -> u32 {
        self.cost + 2 * (self.dst.0 - self.pos.0 + self.dst.1 - self.pos.1) as u32
    }

    fn best_final_cost(&self) -> u32 {
        self.cost + (self.dst.0 - self.pos.0 + self.dst.1 - self.pos.1) as u32
    }

    fn adjacent_positions(&self) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        if self.pos.0 > 0 { positions.push((self.pos.0 - 1, self.pos.1)); }
        if self.pos.1 > 0 { positions.push((self.pos.0, self.pos.1 - 1)); }
        if self.pos.0 < self.dst.0 { positions.push((self.pos.0 + 1, self.pos.1)); }
        if self.pos.1 < self.dst.1 { positions.push((self.pos.0, self.pos.1 + 1)); }
        positions
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {  // make min-heap instead of max-heap
        self.estimation() == other.estimation()
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.estimation().cmp(&self.estimation())
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(filename: &str) -> Vec<Vec<u8>> {
    let f = File::open(filename).expect(&format!("Can't open {}", filename));
    let reader = BufReader::new(f);
    reader.lines().map(|l| line_to_digits_vec(l.unwrap())).collect()
}

fn line_to_digits_vec(line: String) -> Vec<u8> {
    line.chars().map(|c| c.to_digit(10).unwrap() as u8).collect()
}
