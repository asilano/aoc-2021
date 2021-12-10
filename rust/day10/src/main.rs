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

fn parts1and2(input: &String) {
    let mut score: u64 = 0;
    let pairs = HashMap::from([
        ('(', ')'),
        ('[', ']'),
        ('{', '}'),
        ('<', '>')
    ]);
    let penalties = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137)
    ]);
    let incomplete: Vec<Vec<char>> = input.lines().map(|line| {
        let mut stack = Vec::<char>::new();
        let mut wrong = false;
        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => { stack.push(c) },
                ')' | ']' | '}' | '>' => {
                    let last_open = stack.pop();
                    if pairs.get(&last_open.unwrap_or('X')) != Some(&c) {
                        score += penalties.get(&c).unwrap();
                        wrong = true;
                        break;
                    }
                },
                _ => unreachable!()
            }
        }
        if wrong { Vec::<char>::new() } else { stack }
    }).collect();

    println!("Corrupt score: {}", score);

    let close_vals = HashMap::from([
        ('(', 1),
        ('[', 2),
        ('{', 3),
        ('<', 4)
    ]);
    let mut scores: Vec<u64> = incomplete.iter()
                            .filter(|line| line.len() != 0)
                            .map(|line| {
        score = 0;
        for c in line.iter().rev() {
            score *= 5;
            score += close_vals.get(&c).unwrap();
        }
        score
    }).collect();
    scores.sort();
    let median = scores[scores.len() / 2];

    println!("Incomplete score: {}", median);
}

fn main() {
    let input = input_string();

    parts1and2(&input);
}
