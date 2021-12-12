use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashMap, VecDeque};

type CavesGraph = HashMap<String,Vec<String>>;
type CavesPath = Vec<String>;
type CaveInvalidFn = fn(&String, &CavesPath) -> bool;

fn main() {
    let caves_graph = parse_caves_graph("input.txt");

    let paths_count = solve(&caves_graph, is_cave_small_and_visited);
    println!("Part 1: number of paths={}", paths_count);

    let paths_count = solve(&caves_graph, is_cave_ending_or_small_and_visited_twice);
    println!("Part 2: number of paths={}", paths_count);
}

fn solve(caves_graph: &CavesGraph, is_cave_invalid: CaveInvalidFn) -> usize {
    let mut partial_paths = VecDeque::from([vec!["start".to_string()]]);
    let mut completed_paths = Vec::new();

    while let Some(path) = partial_paths.pop_front() {
        let last_cave = path.iter().last().unwrap();
        let next_caves = caves_graph.get(last_cave).unwrap();

        for cave in next_caves {
            if is_cave_invalid(cave, &path) {
                continue;
            }

            let mut new_path = path.clone();
            new_path.push(cave.clone());

            if cave != "end" {
                partial_paths.push_back(new_path);
            } else {
                completed_paths.push(new_path);
            }
        }
    }

    completed_paths.len()
}

fn is_cave_small(cave: &String) -> bool {
    cave.chars().next().unwrap().is_ascii_lowercase()
}

fn is_cave_small_and_visited(cave: &String, path: &CavesPath) -> bool {
    is_cave_small(cave) && path.contains(cave)
}

fn is_cave_ending_or_small_and_visited_twice(cave: &String, path: &CavesPath) -> bool {
    if cave == "start" {
        true
    } else if !is_cave_small_and_visited(cave, path) {
        false
    } else {
        path_has_any_small_twice(path)
    }
}

fn path_has_any_small_twice(path: &CavesPath) -> bool {
    let mut smalls: Vec<&String> = path.iter().filter(|&c| is_cave_small(c)).collect();
    smalls.sort_unstable();
    smalls.windows(2).any(|caves| caves[0] == caves[1])
}

fn parse_caves_graph(filename: &str) -> CavesGraph {
    let f = File::open(filename).expect(format!("Can't open {}", filename).as_str());
    let reader = BufReader::new(f);

    let mut caves_graph = CavesGraph::new();

    for line in reader.lines().map(|l| l.unwrap()) {
        let caves: Vec<&str> = line.split('-').collect();
        assert_eq!(caves.len(), 2);

        caves_graph
            .entry(caves[0].to_string())
            .and_modify(|v| v.push(caves[1].to_string()))
            .or_insert(vec![caves[1].to_string()]);
        caves_graph
            .entry(caves[1].to_string())
            .and_modify(|v| v.push(caves[0].to_string()))
            .or_insert(vec![caves[0].to_string()]);
    }

    caves_graph
}