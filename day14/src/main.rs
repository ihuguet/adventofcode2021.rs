use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BTreeMap;

type Insertions = BTreeMap<(char, char), char>;
type PairsCounters = BTreeMap<(char, char), u64>;
type CharsCounters = BTreeMap<char, u64>;

fn main() {
    let (polymer, insertions) = parse_input("input.txt");
    let (mut pairs_counts, mut chars_counts) = create_counters(&polymer);

    for i in 1..=40 {
        let mut new_pairs_counts = PairsCounters::new();

        for ((ch0, ch1), count) in pairs_counts {
            let ch_insert = *insertions.get(&(ch0, ch1)).unwrap();

            chars_counts
                .entry(ch_insert)
                .and_modify(|v| *v += count)
                .or_insert(count);
            new_pairs_counts
                .entry((ch0, ch_insert))
                .and_modify(|v| *v += count)
                .or_insert(count);
            new_pairs_counts
                .entry((ch_insert, ch1))
                .and_modify(|v| *v += count)
                .or_insert(count);
        }

        pairs_counts = new_pairs_counts;

        if i == 10 {
            let min = chars_counts.values().min().unwrap();
            let max = chars_counts.values().max().unwrap();
            println!("Part 1: max={}, min={}, max-min={}", max, min, max - min);
        }
    }

    let min = chars_counts.values().min().unwrap();
    let max = chars_counts.values().max().unwrap();
    println!("Part 2: max={}, min={}, max-min={}", max, min, max - min);
}

fn parse_input(filename: &str) -> (String, Insertions) {
    let f = File::open(filename).expect(&format!("Can't open {}", filename));
    let reader = BufReader::new(f);
    let mut lines = reader.lines().map(|l| l.unwrap());

    let polymer = lines.next().expect("Error reading file");

    lines.next().expect("Error reading file");

    let mut insertions = Insertions::new();
    for line in lines {
        let split: Vec<&str> = line.split(" -> ").collect();
        let c1 = split[0].chars().nth(0).unwrap();
        let c2 = split[0].chars().nth(1).unwrap();
        let cm = split[1].chars().nth(0).unwrap();
        insertions.insert((c1, c2), cm);
    }

    (polymer, insertions)
}

fn create_counters(polymer: &str) -> (PairsCounters, CharsCounters) {
    let mut pairs_counts = PairsCounters::new();
    for chs in polymer.as_bytes().windows(2) {
        let pair = (chs[0] as char, chs[1] as char);
        *pairs_counts.entry(pair).or_default() += 1;
    }

    let mut chars_counts = CharsCounters::new();
    for ch in polymer.chars() {
        *chars_counts.entry(ch).or_default() += 1;
    }

    (pairs_counts, chars_counts)
}
