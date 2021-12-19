use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BTreeSet;

type Coords = [i32; 3];

fn main() {
    let f = File::open("input.txt").expect("Error opening input.txt");
    let reader = BufReader::new(f);
    let mut lines = reader.lines().map(|l| l.unwrap());

    let mut scanners_all_rotations = Vec::new();
    while let Some(beacons) = parse_scanner_beacons(&mut lines) {
        let beacons_all_rotations = calculate_all_rotations(beacons);
        scanners_all_rotations.push(beacons_all_rotations);
    }

    // scanner 0 coords (without transformation) used as reference
    let mut beacons_abs = vec![scanners_all_rotations.swap_remove(0).swap_remove(0)];
    let mut scanners_abs = vec![[0, 0, 0]];

    while scanners_all_rotations.len() > 0 {
        for i in (0..scanners_all_rotations.len()).rev() {
            let scanner = &scanners_all_rotations[i];

            for beacons in &beacons_abs {
                if let Some((scanner_coords,new_beacons)) = get_overlapping_beacons(scanner, beacons) {
                    beacons_abs.push(new_beacons);
                    scanners_abs.push(scanner_coords);
                    scanners_all_rotations.remove(i);
                    println!("{} scanners left", scanners_all_rotations.len());
                    break;
                }
            }
        }
    }

    // remove duplicates
    let beacons_abs = beacons_abs.iter()
        .flatten()
        .copied()
        .collect::<BTreeSet<Coords>>();
    println!("Part 1: beacons count={}", beacons_abs.len());

    let mut max = 0;
    while let Some(coords1) = scanners_abs.pop() {
        for coords2 in &scanners_abs {
            let diff = sub_coords(*coords2, coords1);
            let manhattan_dist = i32::abs(diff[0]) + i32::abs(diff[1]) + i32::abs(diff[2]);
            if manhattan_dist > max {
                max = manhattan_dist;
            }
        }
    }
    println!("Part 2: max mahattan dist={}", max);
}

fn calculate_all_rotations(beacons: Vec<Coords>) -> Vec<Vec<Coords>> {
    let orientations = [ // conversions for X axis facing other directions
        // axis        sign
        ([0, 1, 2], [1, 1, 1]),   // X
        ([2, 1, 0], [1, 1, -1]),  // -Z
        ([0, 1, 2], [-1, 1, -1]), // -X
        ([2, 1, 0], [-1, 1, 1]),  // Z
        ([1, 0, 2], [-1, 1, 1]),  // Y
        ([1, 0, 2], [1, -1, 1])   // -Y
    ];
    let rotations = [ // conversions for rotations around X axis
        ([0, 1, 2], [1, 1, 1]),   // 0
        ([0, 2, 1], [1, -1, 1]),  // 90
        ([0, 1, 2], [1, -1, -1]), // 180
        ([0, 2, 1], [1, 1, -1])   // 270
    ];

    let mut beacons_all_rotations = Vec::new();
    for orientation in &orientations {
        for rotation in &rotations {
            let beacons_this_axes = beacons.iter()
                .map(|coords| axes_convert(*rotation, axes_convert(*orientation, *coords)))
                .collect();
            beacons_all_rotations.push(beacons_this_axes);
        }
    }
    beacons_all_rotations
}

fn axes_convert(conversion: ([usize;3], [i32;3]), coords: Coords) -> Coords {
    let (axes, mult) = conversion;
    [coords[axes[0]] * mult[0], coords[axes[1]] * mult[1], coords[axes[2]] * mult[2]]
}

fn get_overlapping_beacons(scanner: &Vec<Vec<Coords>>, beacons: &Vec<Coords>) -> Option<(Coords, Vec<Coords>)> {
    for candidate_beacons in scanner {
        if let Some(offset) = calc_offset(beacons, candidate_beacons) {
            let transformed_beacons = candidate_beacons.iter()
                .map(|coords| sub_coords(*coords, offset))
                .collect();
            let scanner_coords = sub_coords([0, 0, 0], offset);
            return Some((scanner_coords, transformed_beacons));
        }
    }
    None
}

fn calc_offset(beacons1: &Vec<Coords>, beacons2: &Vec<Coords>) -> Option<Coords> {
    for &b1 in beacons1 {
        for &b2 in beacons2 {
            let offset = sub_coords(b2, b1);
            if try_offset(beacons1, beacons2, offset) {
                return Some(offset);
            }
        }
    }
    None
}

fn try_offset(beacons1: &Vec<Coords>, beacons2: &Vec<Coords>, offset: Coords) -> bool {
    let mut matches = 0;
    for &b2 in beacons2 {
        let b2 = sub_coords(b2, offset);
        for &b1 in beacons1 {
            if b1 == b2 {
                matches += 1;
                break;
            }
        }
    }
    matches >= 12
}

fn sub_coords(c1: Coords, c2: Coords) -> Coords {
    [c1[0] - c2[0], c1[1] - c2[1], c1[2] - c2[2]]
}

fn parse_scanner_beacons<T>(mut lines: T) -> Option<Vec<Coords>>
where
    T: Iterator<Item = String>
{
    if let None = lines.next() {
        return None;
    }

    let mut coords_list: Vec<Coords> = Vec::new();
    while let Some(line) = lines.next() {
        if line.trim() == "" {
            break;
        }
        let v: Vec<i32> = line.split(',').map(|s| s.parse().unwrap()).collect();
        coords_list.push(v.try_into().unwrap());
    }
    Some(coords_list)
}
