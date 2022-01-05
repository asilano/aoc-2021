use std::fs;
use std::env;
//use std::collections::{VecDeque, HashSet};
use std::process::exit;

fn input_string() -> String {
  let args: Vec<String> = env::args().collect();
  let filename = match args.get(1) {
    _ => "data-distilled.txt"
  };
  fs::read_to_string(filename).unwrap()
}

fn parse_input(input: String) -> Vec<Vec<i64>> {
    input.lines().map(|line| {
        line.split_whitespace().map(|num| num.parse::<i64>().unwrap()).collect()
    }).collect()
}

// fn digits(mut number: u64) -> Vec<i64> {
//     let mut digits = VecDeque::<i64>::new();
//     while number != 0 {
//         digits.push_front((number % 10) as i64);
//         number /= 10;
//     }

//     digits.into_iter().collect()
// }

fn subroutine(param1: i64, param2: i64, param3: i64, w: i64, z: &mut i64) {
    let x = *z % 26 + param2;
    *z /= param1;

    if x != w {
        *z *= 26;
        *z += w + param3;
    }
}

fn part1(constants: &Vec<Vec<i64>>) {
    let divisions_remain: Vec<i64> = (0..14).map(|index| 26i64.pow(
        constants[index..].iter().filter(|c_set| c_set[0] == 26).count() as u32)
    ).collect();

    let mut test = 99_999_999_999_999u64;
    while test > 11_111_111_111_111u64 {
        let mut z = 0i64;
        for index in 0..14 {
            let digit = (test / 10u64.pow(13u32 - index as u32)) % 10;
            if digit == 0 || z > divisions_remain[index] {
                test -= 10u64.pow(13u32 - index as u32) - 1;
                z = -1;
                break;
            }
            subroutine(constants[index][0], constants[index][1], constants[index][2], digit as i64, &mut z);
        }

        if z == 0 {
            println!("Biggest: {}", test);
            return;
        }
        test -= 1;
    }
}
fn part2(constants: &Vec<Vec<i64>>) {
    let divisions_remain: Vec<i64> = (0..14).map(|index| 26i64.pow(
        constants[index..].iter().filter(|c_set| c_set[0] == 26).count() as u32)
    ).collect();

    let mut test = 11_111_111_111_111u64;
    while test < 99_999_999_999_999u64 {
        let mut z = 0i64;
        for index in 0..14 {
            let digit = (test / 10u64.pow(13u32 - index as u32)) % 10;
            if digit == 0 || z > divisions_remain[index] {
                test += 10u64.pow(13u32 - index as u32) - 1;
                z = -1;
                break;
            }
            subroutine(constants[index][0], constants[index][1], constants[index][2], digit as i64, &mut z);
        }

        if z == 0 {
            println!("Biggest: {}", test);
            exit(0);
        }
        test += 1;
    }
}

fn main() {
    let constants = parse_input(input_string());
    part1(&constants);
    part2(&constants);
}
