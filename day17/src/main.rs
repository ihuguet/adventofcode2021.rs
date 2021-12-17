use std::ops::RangeInclusive;

/// Solution for targets with X > 0 and Y < 0 only

const TGT_X: RangeInclusive<i32> = 119..=176;
const TGT_Y: RangeInclusive<i32> = -141..=-84;

enum YVResult {
    Miss(i32), Hit(i32), TooHigh(i32)
}

fn main() {
    let max_xv = *TGT_X.end();   // higher x vel will miss the target at 1st turn
    let max_yv = -TGT_Y.start(); // positive y vels will come back to y_pos=0 with the initial velocity, but negative
                                 // at higher velocities they will always miss the target the turn after comming back to 0

    let possible_x_vels = 0..=max_xv;
    let valid_xv: Vec<i32> = possible_x_vels.rev()
        .filter(|&xv| is_valid_xv(xv))
        .collect();
    
    let mut highest_hit: Option<(i32, (i32, i32))> = None;
    let mut hits_count = 0;
    for &xv in &valid_xv {
        for yv in *TGT_Y.start()..max_yv {
            match evaluate_yv(xv, yv) {
                YVResult::Hit(max_y) => {
                    hits_count += 1;
                    if highest_hit.is_none() || highest_hit.unwrap().0 < max_y {
                        highest_hit = Some((max_y, (xv, yv)));
                    }
                },
                YVResult::Miss(_) => (),
                YVResult::TooHigh(_) => break,
            }
        }
    }

    let (y, (xv, yv)) = highest_hit.unwrap();
    println!("Part 1: max_y={}, v=({}, {})", y, xv, yv);
    println!("Part 2: hits count={}", hits_count);
}

fn is_valid_xv(mut xv: i32) -> bool {
    let mut x = 0;
    while xv > 0 && x < *TGT_X.start() {
        x += xv;
        xv -= 1;
    }
    TGT_X.contains(&x)
}

fn evaluate_yv(mut xv: i32, mut yv: i32) -> YVResult {
    let (y_bottom, x_end) = (*TGT_Y.start(), *TGT_X.end());

    let (mut x, mut y) = (0, 0);
    let mut max_y = 0;
    while y > y_bottom {
        if xv > 0 {
            x += xv;
            xv -= 1;
        }
        y += yv;
        yv -= 1;

        if y > max_y {
            max_y = y;
        }

        if TGT_X.contains(&x) && TGT_Y.contains(&y) {
            return YVResult::Hit(max_y);
        } else if x > x_end {
            return YVResult::TooHigh(max_y);
        }
    }

    YVResult::Miss(max_y)
}
