use std::fs;
use std::env;

fn input_string() -> String {
  let args: Vec<String> = env::args().collect();
  let filename = match args.get(1) {
    Some(a) if a == "test" => "test_data.txt",
    _ => "data.txt"
  };
  fs::read_to_string(filename).unwrap()
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Location {
    Room(char, u8),
    Corridor(u8)
}
use Location::{Room, Corridor};
#[derive(Debug, Clone)]
struct Amphipod {
    kind: char,
    location: Location
}
impl Amphipod {
    fn min_dist_to_target(&self) -> u32 {
        match self.location {
            Room(room, _) if room == self.kind => 0,
            Room(room, pos) => {
                // Distance is steps out of this room, plus distance along corridor, plus 1 step into right room
                (pos as i8 + (room_position(&room) as i8 - room_position(&self.kind) as i8).abs() + 1) as u32
            },
            Corridor(pos) => {
                // Distance is steps along corridor, plus 1 step into right room
                ((pos as i8 - room_position(&self.kind) as i8).abs() + 1) as u32
            }
        }
    }
}

fn parse_input(input: String) -> Vec<Amphipod> {
    let mut lines = input.lines();

    // Absorb the top wall and corridor
    lines.by_ref().next();
    lines.by_ref().next();

    lines.take(2)
        .enumerate()
        .flat_map(|(row, line)| line.chars()
                                    .filter(|c| "ABCD".contains(*c))
                                    .enumerate().map(move |(ix, c)| Amphipod {
                                        kind: c,
                                        location: Room("ABCD".as_bytes()[ix] as char, row as u8 + 1)
                                    })).collect()
}

fn room_position(kind: &char) -> u8 {
    match *kind {
        'A' => 2,
        'B' => 4,
        'C' => 6,
        'D' => 8,
        _ => unreachable!()
    }
}

fn step_cost(kind: &char) -> u32 {
    match *kind {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => unreachable!()
    }
}

fn possible_moves(amphipods: &Vec<Amphipod>, part2: bool) -> Vec<(usize, Location, u32)> {
    let mut possibles = Vec::<(usize, Location, u32)>::new();

    for (ix, pod) in amphipods.iter().enumerate() {
        match pod.location {
            Room(ident, position) => {
                // Don't move if in the right room, and every other amphipod in the same room but further back is
                // also of the same type.
                if ident == pod.kind {
                    if amphipods.iter()
                                .filter(|a| match a.location {
                                    Room(rm, loc) if rm == ident && loc > position => { true },
                                    _ => false
                                })
                                .all(|a| a.kind == ident)
                    { continue; }
                }

                // Can only move to the corridor. Can move to any reachable space,
                // except the doorway
                let doorway = room_position(&ident);
                for c_pos in 0..=10 {
                    if [2,4,6,8].contains(&c_pos) { continue; }
                    if amphipods.iter().all(|p|
                        match p.location {
                            Room(kind, loc) => {
                                !(kind == ident && loc < position)
                            },
                            Corridor(p_pos) => {
                                let route = if doorway > c_pos {
                                    c_pos..=doorway
                                } else {
                                    doorway..=c_pos
                                };
                                !route.contains(&p_pos)
                            }
                        }
                    ) {
                        let distance = (doorway as i8 - c_pos as i8).abs() as u8 + position;
                        let cost = distance as u32 * step_cost(&pod.kind);
                        possibles.push((ix, Corridor(c_pos), cost));
                    }
                }
            },
            Corridor(position) => {
                // Can only move to the right room. Can they get there?
                let doorway = room_position(&pod.kind);
                let get_there = amphipods.iter().all(|p|
                    match p.location {
                        Room(_,_) => true,
                        Corridor(p_pos) => {
                            let route = if doorway > position {
                                position+1..=doorway
                            } else {
                                doorway..=position-1
                            };
                            !route.contains(&p_pos)
                        }
                    }
                );

                // Is the room available?
                let available = amphipods.iter().all(|p|
                    match p.location {
                        Room(id, _) if id == pod.kind => p.kind == pod.kind,
                        _ => true
                    }
                );
                let back_of_room = if part2 { 4 } else { 2 };
                let dest_pos = (1..=back_of_room).filter(|dest| !amphipods.iter().any(|p| p.location == Room(pod.kind, *dest))).max();

                if get_there && available && dest_pos != None {
                    let distance = (doorway as i8 - position as i8).abs() as u8 + dest_pos.unwrap();
                    let cost = distance as u32 * step_cost(&pod.kind);
                        possibles.push((ix, Room(pod.kind, dest_pos.unwrap()), cost));
                }
            }
        }
    };

    possibles.sort_by(|p,q| p.2.cmp(&q.2));
    possibles
}

fn finished(amphipods: &Vec<Amphipod>) -> bool {
    amphipods.iter().all(|pod| match pod.location {
        Room(kind, _) => { kind == pod.kind },
        _ => false
    })
}

fn min_solve_from_position(amphipods: &mut Vec<Amphipod>) -> u32 {
    amphipods.iter().map(|pod| pod.min_dist_to_target() * step_cost(&pod.kind)).sum()
}

fn solve_from_position(amphipods: &mut Vec<Amphipod>, cost: &mut u32, best_cost: &mut Option<u32>, part2: bool) {
    if best_cost != &None && *cost + min_solve_from_position(amphipods) > best_cost.unwrap() { return }

    let possible_moves = possible_moves(amphipods, part2);

    for pod_move in possible_moves {
        if *cost + pod_move.2 > best_cost.unwrap_or(*cost + pod_move.2 + 1) { continue; }

        let backup = amphipods[pod_move.0].location.clone();

        amphipods[pod_move.0].location = pod_move.1;
        *cost += pod_move.2;
        if finished(amphipods) {
            //println!("Finished!");
            if *best_cost == None || best_cost.unwrap() > *cost { *best_cost = Some(*cost);
            println!("best: {}", best_cost.unwrap()) }
        }
        else {
            solve_from_position(amphipods, cost, best_cost, part2);
        }

        *cost -= pod_move.2;
        amphipods[pod_move.0].location = backup;
    }
}

fn parts1and2(mut amphipods: Vec<Amphipod>, part2: bool) {
    let mut cost = 0;
    let mut best_cost = None::<u32>;

    if part2 {
        // Add in the middle rows of amphipods
        for mut pod in &mut amphipods {
            if let Room(rm, pos) = pod.location {
                if pos == 2 {
                    pod.location = Room(rm, 4);
                }
            }
        }
        amphipods.push(Amphipod {
            kind: 'D',
            location: Room('A', 2)
        });
        amphipods.push(Amphipod {
            kind: 'C',
            location: Room('B', 2)
        });
        amphipods.push(Amphipod {
            kind: 'B',
            location: Room('C', 2)
        });
        amphipods.push(Amphipod {
            kind: 'A',
            location: Room('D', 2)
        });
        amphipods.push(Amphipod {
            kind: 'D',
            location: Room('A', 3)
        });
        amphipods.push(Amphipod {
            kind: 'B',
            location: Room('B', 3)
        });
        amphipods.push(Amphipod {
            kind: 'A',
            location: Room('C', 3)
        });
        amphipods.push(Amphipod {
            kind: 'C',
            location: Room('D', 3)
        });
    }

    solve_from_position(&mut amphipods, &mut cost, &mut best_cost, part2);

    println!("Best cost: {}", best_cost.unwrap());
}

fn main() {
    let input = input_string();
    let amphipods = parse_input(input);

    parts1and2(amphipods.clone(), false);
    parts1and2(amphipods.clone(), true);
}
