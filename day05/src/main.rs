use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::collections::BTreeMap;

type Grid = BTreeMap<Point,u32>;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Point {
    x: usize,
    y: usize,
}

fn main() {
    let f = File::open("input.txt").expect("Can't open input.txt");
    let reader = BufReader::new(f);
    let mut lines = reader.lines();

    let mut grid1 = Grid::new();
    let mut grid2 = Grid::new();

    while let Some((p1, p2)) = parse_points(&mut lines) {
        if p1.x == p2.x || p1.y == p2.y {
            grid1.draw_line(p1.clone(), p2.clone());
        }
        grid2.draw_line(p1, p2);
    }

    let count = grid1.iter().filter(|(_point,&val)| val > 1).count();
    println!("Part 1: points with more than 1 line: {}", count);

    let count = grid2.iter().filter(|(_point,&val)| val > 1).count();
    println!("Part 2: points with more than 1 line: {}", count);
}

trait GridT {
    fn draw_line(&mut self, p1: Point, p2: Point);
}

impl GridT for Grid {
    fn draw_line(&mut self, mut p1: Point, p2: Point) {
        let x_diff = p2.x as i32 - p1.x as i32;
        let y_diff = p2.y as i32 - p1.y as i32;

        assert!(x_diff == 0 || y_diff == 0 || i32::abs(x_diff) == i32::abs(y_diff));

        loop {
            *self.entry(Point{x:p1.x,y:p1.y}).or_insert(0) += 1;
            if p1 == p2 {break;}
            if x_diff > 0 { p1.x += 1; }
            else if x_diff < 0 { p1.x -= 1; }
            if y_diff > 0 { p1.y += 1; }
            else if y_diff < 0 { p1.y -= 1; }
        }
    }
}

fn parse_points<T: BufRead>(lines: &mut Lines<T>) -> Option<(Point,Point)> {
    let line = lines.next()?.unwrap();
    let mut split = line.split(" -> ");
    let p1_str = split.next()?;
    let p2_str = split.next()?;
    Some((parse_point(p1_str), parse_point(p2_str)))
}

fn parse_point(p_str: &str) -> Point {
    let mut split = p_str.split(',');
    let x = split.next().unwrap().parse().unwrap();
    let y = split.next().unwrap().parse().unwrap();
    Point {x, y}
}
