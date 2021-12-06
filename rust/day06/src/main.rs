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

fn parse_input(input: String) -> HashMap<u32, u64> {
    let mut timers: HashMap<u32, u64> = HashMap::from([
        (0,0),(1,0),(2,0),(3,0),(4,0),(5,0),(6,0),(7,0),(8,0)
    ]);

    for fish in input.split(',') {
        let time = fish.parse::<u32>().unwrap();
        *timers.get_mut(&time).unwrap() += 1;
    };

    timers
}

fn part1and2(fish_timers: &HashMap<u32, u64>, days: u32)
{
    let mut fish_timers = fish_timers.clone();
    let mut babes = 0;
    let mut juves = 0;
    for day in 0..days {
        let spawn_key = day % 7;
        let new_adults = juves;
        juves = babes;
        babes = fish_timers[&spawn_key];
        *fish_timers.get_mut(&spawn_key).unwrap() += new_adults;
    }
    println!("Now have {} fish", fish_timers.values().sum::<u64>() + babes + juves);
}

fn main() {
    let input = input_string();
    let fish_timers = parse_input(input);

    part1and2(&fish_timers, 80);
    part1and2(&fish_timers, 256);
}
