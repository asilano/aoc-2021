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

type Ruleset = HashMap<[u8; 2], u8>;

fn parse_input(input: String) -> (String, Ruleset) {
    let mut lines = input.lines();
    let polymer = lines.next().unwrap();
    lines.next();
    let ruleset = Ruleset::from_iter(
        lines.map(|l| {
            let chars = l.as_bytes();
            (chars[0..2].try_into().expect("slice of wrong length"), chars[6])
        })
    );

    (String::from(polymer), ruleset)
}

fn part1(mut polymer: String, ruleset: &Ruleset) {
    for _ in 0..10 {
        let insertions: Vec<Option<&u8>> = polymer.as_bytes().windows(2).map(|cs|
            ruleset.get(cs)
        ).collect();

        let len = polymer.len();
        for (ix, insert) in insertions.iter().rev().enumerate() {
            match insert {
                Some(c) => polymer.insert(len - ix - 1, **c as char),
                None => {}
            }

        }
        //println!("{}", polymer);
    }

    let counts = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().map(|c| polymer.chars().filter(|pc| *pc == c).count()).filter(|n| *n!=0);
    let min = counts.clone().min();
    let max = counts.clone().max();
    println!("Difference = {}", max.unwrap() - min.unwrap());
}

fn part2(polymer: String, ruleset: &Ruleset) {
    let mut pair_counts = HashMap::<[u8; 2], usize>::new();
    for pair in polymer.as_bytes().windows(2) {
        let count = pair_counts.get_mut(pair);
        match count {
            Some(n) => { *n += 1 },
            None => { pair_counts.insert(pair.try_into().expect("wrong size"), 1); }
        }
    }

    for _ in 0..40 {
        let mut swap = HashMap::<[u8; 2], usize>::new();

        for rule in ruleset {
            let o_existing = pair_counts.get(rule.0);
            if o_existing == None { continue }
            let existing = o_existing.unwrap();
            let left_key = [rule.0[0], *rule.1];
            let left = swap.get_mut(&left_key);
            match left {
                Some(n) => { *n += existing },
                None => { swap.insert(left_key, *existing); }
            }
            let right_key = [*rule.1, rule.0[1]];
            let right = swap.get_mut(&right_key);
            match right {
                Some(n) => { *n += existing },
                None => { swap.insert(right_key, *existing); }
            }
        }

        pair_counts = swap;
    }

    let mut counts = HashMap::<u8, usize>::new();
    for (pair, num) in pair_counts {
        let count = counts.get_mut(&pair[0]);
        match count {
            Some(n) => { *n += num },
            None => { counts.insert(pair[0], num); }
        }
        let count = counts.get_mut(&pair[1]);
        match count {
            Some(n) => { *n += num },
            None => { counts.insert(pair[1], num); }
        }
    }

    // Account for the end characters not being double-counted
    counts.get_mut(&polymer.as_bytes()[0]).and_then(|n| {*n += 1; Some(n)});
    counts.get_mut(&polymer.as_bytes().last().unwrap()).and_then(|n| {*n += 1; Some(n)});

    let min = counts.values().min();
    let max = counts.values().max();
    println!("Difference = {}", max.unwrap() / 2 - min.unwrap() / 2);
}

fn main() {
    let input = input_string();
    let (polymer, ruleset) = parse_input(input);

    part1(polymer.clone(), &ruleset);
    part2(polymer.clone(), &ruleset);
}
