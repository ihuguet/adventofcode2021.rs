use std::collections::HashMap;

const PLAYER_1_START: u32 = 7;
const PLAYER_2_START: u32 = 6;

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
struct Player {
    pos: u32,
    points: u32,
}

#[derive(Eq, PartialEq, Hash)]
struct Universe {
    active: usize,
    players: [Player; 2],
}

const DIRAC_DICES_COMBINATIONS: [(u32, u32); 7] = [  
    (3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1) // (dices_sum, frequency)
];

fn main() {
    part1();
    part2();
}

fn part1() {
    let mut players = [
        Player{ pos: PLAYER_1_START, points: 0 },
        Player{ pos: PLAYER_2_START, points: 0 },
    ];
    let mut active = 0;
    let mut dice: u32 = 0;
    let mut rolls_count = 0;

    while players[0].points < 1000 && players[1].points < 1000 {
        let dice1 = if dice < 100 { dice + 1 } else { 1 };
        let dice2 = if dice1 < 100 { dice1 + 1 } else { 1 };
        let dice3 = if dice2 < 100 { dice2 + 1 } else { 1 };
        rolls_count += 3;

        let adv = dice1 + dice2 + dice3;
        
        players[active].pos = (players[active].pos - 1 + adv) % 10 + 1;
        players[active].points += players[active].pos;

        active = if active == 0 { 1 } else { 0 };
        dice = dice3;
    }

    let looser_points = players[0].points.min(players[1].points);
    println!("Part 1: points={}, rolls={}, mult={}", looser_points, rolls_count, looser_points * rolls_count);
}

fn part2() {
    let players = [
        Player{ pos: PLAYER_1_START, points: 0 },
        Player{ pos: PLAYER_2_START, points: 0 },
    ];
    let mut universes: HashMap<Universe, u64> = HashMap::new(); // value is the count of times this universe is repeated
    universes.insert(Universe { active: 0, players}, 1);
    
    let mut wins = [0u64, 0u64];

    while universes.len() > 0 {
        let mut new_universes = HashMap::new();

        for (universe, universe_count) in universes {
            for &(dices_sum, dices_sum_count) in &DIRAC_DICES_COMBINATIONS {
                let Universe {active, mut players} = universe;

                players[active].pos = (players[active].pos - 1 + dices_sum) % 10 + 1;
                players[active].points += players[active].pos;

                let created_universes_count = universe_count * dices_sum_count as u64;

                if players[active].points >= 21 {
                    wins[active] += created_universes_count;
                } else {
                    let active = if active == 0 { 1 } else { 0 };
                    new_universes
                        .entry(Universe { active, players })
                        .and_modify(|count| *count += created_universes_count)
                        .or_insert(created_universes_count);
                }
            }
        }

        universes = new_universes;
    }

    println!("Part 2: max wins={}", wins[0].max(wins[1]));
}
