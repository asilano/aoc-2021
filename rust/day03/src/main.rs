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

fn count_zeroes(lines: &Vec<String>, bit: usize) -> usize {
    lines.iter()
         .filter(|n| n.chars().nth(bit) == Some('0'))
         .count()
}

fn part1(input_str: &String) {
    let lines: Vec<String> = input_str.lines().map(|x| x.chars().rev().collect()).collect();
    let diags_count = lines.len();
    let binary_len = lines[0].len();
    let half_diags = diags_count / 2;
    let mask = (1 << binary_len) - 1;

    let mut gamma: u32 = 0;
    for bit in 0..binary_len {
        let num_zeroes = count_zeroes(&lines, bit);
        if num_zeroes < half_diags {
            gamma += 1 << bit;
        }
    }
    let epsilon: u32 = !gamma & mask;

    println!("Power consumption is {} * {} = {}", gamma, epsilon, gamma * epsilon);
}

fn part2(input_str: &String) {
    let lines: Vec<String> = input_str.lines().map(|s| s.to_string()).collect();

    let mut oxy_lines = lines.clone();
    let mut bit: usize = 0;
    while oxy_lines.len() > 1 {
        let half_length = oxy_lines.len() / 2;
        let num_zeroes = count_zeroes(&oxy_lines, bit);
        oxy_lines.retain(|l| l.chars().nth(bit) == Some(if num_zeroes > half_length { '0' } else { '1' }));
        bit += 1;
    }
    let mut co2_lines = lines.clone();
    let mut bit: usize = 0;
    while co2_lines.len() > 1 {
        let half_length = co2_lines.len() / 2;
        let num_zeroes = count_zeroes(&co2_lines, bit);
        co2_lines.retain(|l| l.chars().nth(bit) == Some(if num_zeroes > half_length { '1' } else { '0' }));
        bit += 1;
    }

    let answer = u32::from_str_radix(&oxy_lines[0], 2).unwrap() *
        u32::from_str_radix(&co2_lines[0], 2).unwrap();
    println!("Oxygen: {}. CO2: {}. Answer: {}", oxy_lines[0], co2_lines[0], answer);
}

fn main() {
    let input_str = input_string();
    part1(&input_str);
    part2(&input_str);
}
