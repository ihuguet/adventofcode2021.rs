use std::fs::File;
use std::io::{BufRead, BufReader};

struct Img {
    img: Vec<Vec<u8>>,
    bg: u8,
}

fn main() {
    let (filter, mut img) = parse_input("input.txt");

    for i in 1..=50 {
        img = get_filtered_img(&filter, &img);
        if i == 2 {
            println!("Part 1: px lit count={}", count_lit(&img));
        }
    }

    println!("Part 2: px lit count={}", count_lit(&img));
}

fn get_filtered_img(filter: &Vec<u8>, img: &Img) -> Img {
    let rows = img.img.len() + 2;
    let cols = img.img[0].len() + 2;

    let mut new_img = Vec::with_capacity(rows);
    for i in 0..rows {
        let mut px_row = Vec::with_capacity(cols);
        for j in 0..cols {
            let i = i as isize - 1;  // indexes in orig img are (i - 1, j - 1)
            let j = j as isize - 1;
            let filter_idx = get_filter_idx(img, (i, j));
            let px = filter[filter_idx];
            px_row.push(px);
        }
        new_img.push(px_row);
    }

    let new_bg = if img.bg == 0 {
        filter[0]
    } else {
        filter[0b111111111]
    };

    Img{ img: new_img, bg: new_bg }
}

fn get_filter_idx(img: &Img, (i, j): (isize, isize)) -> usize {
    let mut shift = 8;
    let mut filter_idx = 0;
    for i in i - 1..=i + 1 {
        for j in j - 1..=j + 1 {
            let px = if valid_px(img, (i, j)) {
                img.img[i as usize][j as usize]
            } else {
                img.bg
            };
            filter_idx |= (px as usize) << shift;
            shift -= 1;
        }
    }
    filter_idx
}

fn valid_px(img: &Img, (i, j): (isize, isize)) -> bool {
    let i_len = img.img.len() as isize;
    let j_len = img.img[0].len() as isize;
    i >= 0 && i < i_len && j >= 0 && j < j_len
}

fn count_lit(img: &Img) -> u32 {
    assert!(img.bg == 0, "Infinite lit pxs");
    img.img.iter().flatten().map(|px| *px as u32).sum()
}

fn parse_input(filename: &str) -> (Vec<u8>, Img) {
    let f = File::open(filename).expect(&format!("Error opening {}", filename));
    let reader = BufReader::new(f);
    let mut lines = reader.lines().map(|l| l.unwrap());

    let filter = parse_line(&lines.next().unwrap());
    lines.next().unwrap();
    let img: Vec<_> = lines.map(|l| parse_line(&l)).collect();

    (filter, Img{ img, bg: 0 })
}

fn parse_line(line: &str) -> Vec<u8> {
    line.chars()
        .map(|ch| if ch == '#' { 1u8 } else { 0u8 })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_px() {
        let (filter, img) = parse_input("input_test.txt");

        let filter_idx = get_filter_idx(&img, (2, 2));
        assert_eq!(filter_idx, 34, "Wrong filter idx calculation");

        let px = filter[filter_idx];
        assert_eq!(px, 1);
    }

    #[test]
    fn filter_img() {
        let (filter, img) = parse_input("input_test.txt");

        let expect_img = ".##.##.\n#..#.#.\n##.#..#\n####..#\n.#..##.\n..##..#\n...#.#.";
        let expect_img = parse_img_str(expect_img);
        let filtered_img = get_filtered_img(&filter, &img);
        assert_eq!(filtered_img.img, expect_img);
        assert_eq!(filtered_img.bg, 0);

        let expect_img = ".......#.\n.#..#.#..\n#.#...###\n#...##.#.\n#.....#.#\n.#.#####.\n..#.#####\n...##.##.\n....###..";
        let expect_img = parse_img_str(expect_img);
        let filtered_img = get_filtered_img(&filter, &filtered_img);
        assert_eq!(filtered_img.img, expect_img);
        assert_eq!(filtered_img.bg, 0);

        assert_eq!(count_lit(&filtered_img), 35);
    }

    fn parse_img_str(input: &str) -> Vec<Vec<u8>> {
        input.lines().map(|l| parse_line(l)).collect()
    }
}