use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

// input
const ROOM_A1: [Type; 2] = [Type::D, Type::C];
const ROOM_B1: [Type; 2] = [Type::D, Type::A];
const ROOM_C1: [Type; 2] = [Type::B, Type::B];
const ROOM_D1: [Type; 2] = [Type::A, Type::C];

const ROOM_A2: [Type; 4] = [Type::D, Type::D, Type::D, Type::C];
const ROOM_B2: [Type; 4] = [Type::D, Type::C, Type::B, Type::A];
const ROOM_C2: [Type; 4] = [Type::B, Type::B, Type::A, Type::B];
const ROOM_D2: [Type; 4] = [Type::A, Type::A, Type::C, Type::C];

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    A = 0, B = 1, C = 2, D = 3, None
}

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Burrow {
    hallway: Hallway,
    rooms: [Room; 4],
}

type Hallway = [Type; 11];
type Room = Vec<Type>;
type State = (u32, Burrow);

fn main() {
    let burrow = Burrow {
        hallway: [Type::None; 11],
        rooms: [ROOM_A1.to_vec(), ROOM_B1.to_vec(), ROOM_C1.to_vec(), ROOM_D1.to_vec()],
    };
    let cost = solve(burrow);
    println!("Part 1: min cost={}", cost);

    let burrow = Burrow {
        hallway: [Type::None; 11],
        rooms: [ROOM_A2.to_vec(), ROOM_B2.to_vec(), ROOM_C2.to_vec(), ROOM_D2.to_vec()],
    };
    let cost = solve(burrow);
    println!("Part 2: min cost={}", cost);
}

fn solve(burrow: Burrow) -> u32 {
    let mut states: HashMap<Burrow, u32> = HashMap::new();
    states.insert(burrow.clone(), 0);
    
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, burrow)));

    let mut min_cost = u32::MAX;

    while let Some(Reverse((mut cost, mut burrow))) = queue.pop() {
        if cost > min_cost {
            continue;
        }

        // first execute all direct moves to destination: they're always the cheapest option
        let mut direct_moves_done = false;
        while !direct_moves_done {
            direct_moves_done = true;

            for pos in 0..burrow.hallway.len() {
                if burrow.can_move_from_hallway(pos) {
                    cost += burrow.move_from_hallway_to_dest(pos);
                    direct_moves_done = false;
                }
            }

            for room in 0..4 {
                if burrow.can_move_from_room_to_dest(room) {
                    cost += burrow.move_from_room_to_dest(room);
                    direct_moves_done = false;
                }
            }
        }

        // cost too high or finished?
        if cost > min_cost {
            continue;
        } else if burrow.finished() {
            min_cost = cost;
            continue;
        }

        // then, enqueue other possible moves
        for room in 0..4 {
            if !burrow.can_move_from_room(room) {
                continue;
            }

            for pos in 0..burrow.hallway.len() {
                if burrow.can_move_from_room_to_hallway(room, pos) {
                    let mut new_burrow = burrow.clone();
                    let move_cost = new_burrow.move_from_room_to_hallway(room, pos);
                    maybe_enqueue((cost + move_cost, new_burrow), &mut queue, &mut states);
                }
            }
        }
    }

    min_cost
}

fn maybe_enqueue(state: State, queue: &mut BinaryHeap<Reverse<State>>, states: &mut HashMap<Burrow, u32>) {
    let enqueue = match states.get(&state.1) {
        Some(&cost) => state.0 < cost,
        None => true,
    };
    if enqueue {
        states.insert(state.1.clone(), state.0);
        queue.push(Reverse(state));
    }
}

impl Burrow {
    fn finished(&self) -> bool {
        self.rooms[0].iter().all(|&a| a == Type::A)
        && self.rooms[1].iter().all(|&a| a == Type::B)
        && self.rooms[2].iter().all(|&a| a == Type::C)
        && self.rooms[3].iter().all(|&a| a == Type::D)
    }

    fn can_move_from_hallway(&self, pos: usize) -> bool {
        match self.hallway[pos] {
            Type::None => false,
            a => {
                let room_pos = Self::room_pos(a.destination());
                self.dest_room_ok(a) && !self.path_blocked(pos, room_pos)
            }
        }
    }
    
    fn can_move_from_room(&self, room_idx: usize) -> bool {
        let room = &self.rooms[room_idx];
        let room_slot = room.iter().position(|&a| a != Type::None);
        if let Some(room_slot) = room_slot {
            room[room_slot..].iter().any(|&a| a.destination() != room_idx)
        } else {
            false
        }
    }

    fn can_move_from_room_to_dest(&self, room_idx: usize) -> bool {
        if !self.can_move_from_room(room_idx) {
            return false;
        }

        let &amphipod = self.rooms[room_idx].iter()
                            .skip_while(|&&a| a == Type::None)
                            .next().unwrap();
        let start = Self::room_pos(room_idx);
        let end = Self::room_pos(amphipod.destination());

        self.dest_room_ok(amphipod) && !self.path_blocked(start, end)
    }

    fn can_move_from_room_to_hallway(&self, room_idx: usize, dest_pos: usize) -> bool {
        match dest_pos {
            2 | 4 | 6 | 8 => false, // in front of a room forbidden
            pos => self.can_move_from_room(room_idx) 
                   && !self.path_blocked(Self::room_pos(room_idx), pos),
        }
    }

    fn move_from_room_to_hallway(&mut self, room_idx: usize, pos: usize) -> u32 {
        let room_pos = Self::room_pos(room_idx);
        let room_slot =self.rooms[room_idx].iter()
                           .position(|&a| a != Type::None).unwrap();
        let amphipod = self.rooms[room_idx][room_slot];
        
        // exit from room
        self.rooms[room_idx][room_slot] = Type::None;
        let mut moves = 1 + room_slot as u32;

        // move to dest pos
        self.hallway[pos] = amphipod;
        moves += i32::abs(room_pos as i32 - pos as i32) as u32;

        moves * amphipod.move_cost()
    }
    
    fn move_from_hallway_to_dest(&mut self, pos: usize) -> u32 {
        let amphipod = self.hallway[pos];
        let room_idx = amphipod.destination();
        let room_pos = Self::room_pos(room_idx);
        let room_slot = self.rooms[room_idx].iter()
                            .rposition(|&a| a == Type::None).unwrap();
    
        // move to dest room's door
        self.hallway[pos] = Type::None;
        let mut moves = i32::abs(room_pos as i32 - pos as i32) as u32;
    
        // move inside room
        self.rooms[room_idx][room_slot] = amphipod;
        moves += 1 + room_slot as u32;
    
        moves * amphipod.move_cost()
    }

    fn move_from_room_to_dest(&mut self, room_idx: usize) -> u32 {
        let room_pos = Self::room_pos(room_idx);
        let mut cost = self.move_from_room_to_hallway(room_idx, room_pos);
        cost += self.move_from_hallway_to_dest(room_pos);
        cost
    }
    
    fn dest_room_ok(&self, amphipod: Type) -> bool {
        let room = &self.rooms[amphipod.destination()];
        room[0] == Type::None && room[1..].iter().all(|&a| a == Type::None || a == amphipod)
    }
    
    fn path_blocked(&self, start: usize, end: usize) -> bool {
        if start < end {
            self.hallway[start + 1..=end].iter().any(|&x| x != Type::None)
        } else {
            self.hallway[end..start].iter().any(|&x| x != Type::None)
        }
    }

    fn room_pos(room_idx: usize) -> usize {
        2 * (room_idx + 1)
    }
}

impl Type {
    fn destination(&self) -> usize {
        match self {
            Type::None => panic!("Empty space has no destinations"),
            &a => a as usize,
        }
    }

    fn move_cost(&self) -> u32 {
        match self {
            Type::A => 1,
            Type::B => 10,
            Type::C => 100,
            Type::D => 1000,
            Type::None => panic!("Empty space cannot move"),
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ch = match self {
            Type::A => 'A',
            Type::B => 'B',
            Type::C => 'C',
            Type::D => 'D',
            Type::None => '.',
        };
        write!(f, "{}", ch)
    }
}

impl std::fmt::Display for Burrow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#############\n#{}#\n###{}#{}#{}#{}###\n  #{}#{}#{}#{}#  \n  #########\n",
                  self.hallway.map(|a| format!("{}", a)).concat(),
                  self.rooms[0][0], self.rooms[1][0], self.rooms[2][0], self.rooms[3][0],
                  self.rooms[0][1], self.rooms[1][1], self.rooms[2][1], self.rooms[3][1])
    }
}