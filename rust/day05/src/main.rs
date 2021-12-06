use std::fs;
use std::env;
use std::collections::HashMap;

fn input_string() -> String {
  let args: Vec<String> = env::args().collect();
  let filename = match args.get(1) {
    Some(a) if a == "test" => "test_data.txt",
    _ => "data.txt"
  };
  fs::read_to_string(filename).unwrap()
}

#[derive(Debug)]
struct Vent {
    start_x: i32,
    start_y: i32,
    direction: (i32, i32),
    length: i32
}

fn parse_input(input: String) -> Vec<Vent> {
    input.lines().map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let start = parts[0];
        let end = parts[2];
        let (start_x, start_y) = match &start.split(',')
                                             .map(|n| n.parse::<i32>().unwrap())
                                             .collect::<Vec<i32>>()[..] {
                                                 &[a, b, ..] => (a, b),
                                                 _ => unreachable!()
                                             };
        let (end_x, end_y) = match &end.split(',')
                                       .map(|n| n.parse::<i32>().unwrap())
                                       .collect::<Vec<i32>>()[..] {
                                           &[a, b, ..] => (a, b),
                                           _ => unreachable!()
                                       };
        let direction = ((end_x - start_x).signum(),
                         (end_y - start_y).signum());
        let length = if start_x == end_x {
            (end_y - start_y).abs()
         } else {
            (end_x - start_x).abs()
         } + 1;
        Vent{ start_x, start_y, direction, length }
    }).collect()
}

fn part1and2(vents: &Vec<Vent>, part: usize) {
    let mut seafloor: HashMap<String, u32> = HashMap::new();
    for vent in vents {
        if part == 1 && vent.direction.0 != 0 && vent.direction.1 != 0 { continue; }
        let (mut cur_x, mut cur_y) = (vent.start_x, vent.start_y);
        for _ in 0..vent.length {
            let location = format!("{},{}", cur_x, cur_y);
            let current_count = match seafloor.get(&location) {
                Some(n) => *n,
                None => 0
            };
            seafloor.insert(location, current_count + 1);
            cur_x += vent.direction.0;
            cur_y += vent.direction.1;
        }
    }

    let danger = seafloor.values().filter(|n| **n > 1).count();

    println!("Danger count: {}", danger);
}

fn main() {
    let input_str = input_string();
    let vents = parse_input(input_str);

    part1and2(&vents, 1);
    part1and2(&vents, 2);
}
