use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{BTreeMap, BTreeSet};

type Grid = BTreeMap<usize, BTreeSet<usize>>;

enum Fold {
    X(usize),
    Y(usize),
}

fn main() {
    let f = File::open("input.txt").expect("Can't open input.txt");
    let reader = BufReader::new(f);
    let mut lines = reader.lines().map(|l| l.unwrap());

    let mut points = parse_grid(&mut lines);
    let folds = parse_folds(&mut lines);

    let mut first_fold_done = false;
    for fold in folds {
        match fold {
            Fold::X(num) => fold_x(&mut points, num),
            Fold::Y(num) => fold_y(&mut points, num),
        }

        if !first_fold_done {
            first_fold_done = true;
            println!("Part 1: points count={}", count_points(&points));
        }
    }

    println!("Part 2: code:");
    print_points(&points);
}

fn fold_x(points: &mut Grid, num: usize) {
    for (_, row) in points.iter_mut() {
        let folded_cols: Vec<usize> = row.range(num + 1..).copied().collect();
        for x in folded_cols {
            row.insert(num - (x - num));
            row.remove(&x);
        }
        row.remove(&num);
    }
}

fn fold_y(points: &mut Grid, num: usize) {
    let folded_rows = points.split_off(&(num + 1));
    for (y, mut row) in folded_rows {
        points.entry(num - (y - num)).or_default().append(&mut row);
    }
    points.remove(&num);
}

fn count_points(points: &Grid) -> u32 {
    points.iter()
        .map(|(_y, row)| row.len() as u32)
        .sum::<u32>()
}

fn print_points(points: &Grid) {
    for (_, row) in points {
        let last = *row.iter().last().unwrap();
        for x in 0..=last {
            if row.contains(&x) {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!();
    }
}

fn parse_grid<T>(lines: &mut T) -> Grid
where
    T: Iterator<Item = String>
{
    let mut points = Grid::new();
    for line in lines {
        if line.trim() == "" { break; }
        let (x, y) = parse_points(&line);
        points.entry(y).or_default().insert(x);
    }
    points
}

fn parse_points(line: &str) -> (usize, usize) {
    let mut split = line.split(',').map(|s| s.parse::<usize>().unwrap());
    (split.next().unwrap(), split.next().unwrap())
}

fn parse_folds<T>(lines: &mut T) -> Vec<Fold>
where
    T: Iterator<Item = String>
{
    lines.map(|s| parse_fold(&s)).collect()
}

fn parse_fold(line: &str) -> Fold {
    let mut split = line.split_ascii_whitespace().skip(2)
        .next().unwrap()
        .split('=');
    
    let axis = split.next().unwrap();
    let num = split.next().unwrap().parse().unwrap();

    match axis {
        "x" => Fold::X(num),
        "y" => Fold::Y(num),
        _ => panic!(),
    }
}
