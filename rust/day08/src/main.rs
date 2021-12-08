use std::fs;
use std::env;
use std::collections::HashMap;
use itertools::Itertools;

fn input_string() -> String {
  let args: Vec<String> = env::args().collect();
  let filename = match args.get(1) {
    Some(a) if a == "test" => "test_data.txt",
    _ => "data.txt"
  };
  fs::read_to_string(filename).unwrap()
}

fn parse_input(input: String) -> Vec<(Vec<String>, Vec<String>)> {
    input.lines().map(|l| {
        let (possible, output) = l.split_once('|').unwrap();
        (possible.split_whitespace().map(|s| s.to_string()).collect(),
            output.split_whitespace().map(|s| s.to_string()).collect())
    }).collect()
}

fn part1(displays: &Vec<(Vec<String>, Vec<String>)>) {
    let num_1478: usize = displays.iter().map(|d| {
        let outputs = &d.1;
        outputs.iter().filter(|o| match o.len() {
            2 | 3 | 4 | 7 => true,
            _ => false
        }).count()
    }).sum();

    println!("There are {} 1,4,7,8", num_1478);
}

fn part2(displays: &Vec<(Vec<String>, Vec<String>)>) {
    let mapping: Vec<HashMap<String, u8>> = displays.iter().map(|d| {
        segment_mapping(&d.0)
    }).collect();

    let values = displays.iter().map(|d| &d.1)
                        .zip(mapping)
                        .map(|(outputs, digits)| read_display(&outputs, &digits));

    println!("Sum is {}", values.sum::<u32>());
}

fn read_display(outputs: &Vec<String>, digit_map: &HashMap<String, u8>) -> u32 {
    let mut out: u32 = 0;
    for segments in outputs {
        let sorted_segs: String = segments.chars().sorted().collect();
        let digit_wrap = digit_map.get(&sorted_segs);
        let digit = match digit_wrap {
            Some(a) => *a,
            None => {
                println!("Couldn't find {} in {:?}", sorted_segs, digit_map);
                unreachable!();
            }
        };
        out = out * 10 + digit as u32;
    }

    out
}

fn segment_mapping(possibles: &Vec<String>) -> HashMap<String, u8> {
    let one = possibles.iter().find(
        |n| n.len() == 2).unwrap();
    let seven = possibles.iter().find(
        |n| n.len() == 3).unwrap();
    let four = possibles.iter().find(
        |n| n.len() == 4).unwrap();

    let (rights, tops): (Vec<char>, Vec<char>) =
        seven.chars().partition(|c| one.contains(*c));
    if tops.len() != 1 { unreachable!("Top has more than one option"); }
    let top = tops[0];

    if rights.len() != 2 { unreachable!("Right doesn't have two options"); }
    let (top_right, bottom_right) =
        if possibles.iter().filter(|n| n.contains(rights[0])).count() == 8 {
            // rights is [TR, BR]
            (rights[0], rights[1])
        } else {
            (rights[1], rights[0])
        };

    let tl_or_mids: Vec<char> = four.chars().partition(|c| one.contains(*c)).1;
    if tl_or_mids.len() != 2 { unreachable!("TL/Middle doesn't have two options"); }
    let (top_left, middle) =
        if possibles.iter().filter(|n| n.contains(tl_or_mids[0])).count() == 6 {
            //  is [TL, Middle]
            (tl_or_mids[0], tl_or_mids[1])
        } else {
            (tl_or_mids[1], tl_or_mids[0])
        };

    // Just BL and Bottom to identify. They are the letters not yet used, where
    // BL appears in 4 digits, and bottom in 7.
    let bottom_left = ('a'..='g').find(|c| {
        ![top, top_left, top_right, middle, bottom_right].contains(c) &&
        possibles.iter().filter(|n| n.contains(*c)).count() == 4
    }).unwrap();
    let bottom = ('a'..='g').find(|c| {
        ![top, top_left, top_right, middle, bottom_right].contains(c) &&
        possibles.iter().filter(|n| n.contains(*c)).count() == 7
    }).unwrap();
println!("Decoded {:?}", possibles);
    HashMap::from([
        (vec![top, top_left, top_right, bottom_left, bottom_right, bottom]
            .iter()
            .sorted()
            .collect(), 0),
        (vec![top_right, bottom_right]
            .iter()
            .sorted()
            .collect(), 1),
        (vec![top, top_right, middle, bottom_left, bottom]
            .iter()
            .sorted()
            .collect(), 2),
        (vec![top, top_right, middle, bottom_right, bottom]
            .iter()
            .sorted()
            .collect(), 3),
        (vec![top_left, top_right, middle, bottom_right]
            .iter()
            .sorted()
            .collect(), 4),
        (vec![top, top_left, middle, bottom_right, bottom]
            .iter()
            .sorted()
            .collect(), 5),
        (vec![top, top_left, middle, bottom_left, bottom_right, bottom]
            .iter()
            .sorted()
            .collect(), 6),
        (vec![top, top_right, bottom_right]
            .iter()
            .sorted()
            .collect(), 7),
        (vec![top, top_left, top_right, middle, bottom_left, bottom_right, bottom]
            .iter()
            .sorted()
            .collect(), 8),
        (vec![top, top_left, top_right, middle, bottom_right, bottom]
            .iter()
            .sorted()
            .collect(), 9),
    ])
}

fn main() {
    let input = input_string();
    let displays = parse_input(input);

    part1(&displays);
    part2(&displays);
}
