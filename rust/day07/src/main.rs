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

fn parse_input(input: String) -> Vec<i32> {
    input.split(',').map(|n| n.parse::<i32>().unwrap()).collect()
}

fn part2(positions: &Vec<i32>) {
    // With distance function between square and linear, the minimal location is
    // between mean and median.
    let count = positions.len();
    let mid = count / 2;
    let median = if mid % 2 == 0 {
        (positions[mid - 1] + positions[mid]) as f64 / 2.0
    } else {
        positions[mid] as f64
    };
    let mean = positions.iter().sum::<i32>() as f64 / count as f64;

    let mut min_fuel: i32 = -1;
    let (min, max) = if mean < median {
        (mean.floor() as i32, median.ceil() as i32)
    } else {
        (median.floor() as i32, mean.ceil() as i32)
    };
    for target in min..=max {
        let fuel: i32 = positions.iter()
                            .map(|x| {
                                let dist = (x - target).abs();
                                dist * (dist + 1) / 2
                            }).sum();
        if fuel < min_fuel || min_fuel == -1 {
            min_fuel = fuel;
        }
    }

    println!("Min fuel: {}", min_fuel);
}

fn main() {
    let mut positions = parse_input(input_string());
    positions.sort();

    // Part1 - the minimal deviation is at the median
    let count = positions.len();
    let mid = count / 2;
    if count % 2 == 0 {
        let med = [positions[mid - 1], positions[mid]];
        let fuel: [i32; 2] = med.map(|m| positions.iter().map(|x| (x - m).abs()).sum());
        println!("Min dist is {}", if fuel[0] < fuel[1] {fuel[0]} else {fuel[1]});
    } else {
        let med = positions[mid];
        println!("Min dist is {}", positions.iter().map(|x| (x - med).abs()).sum::<i32>());
    }

    part2(&positions);
}
