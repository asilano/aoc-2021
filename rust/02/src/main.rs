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

fn parse_input<'a> (input:&'a String) -> Vec<(&'a str, i32)> {
  input.lines()
       .map(|l| l.split_once(' ').unwrap())
       .map(|parts| (parts.0, parts.1.parse::<i32>().unwrap()))
       .collect()
}

fn part1<'a>(plan: impl Iterator<Item=&'a(&'a str, i32)>) {
  let mut horiz = 0;
  let mut depth = 0;

  for instr in plan {
    match instr.0 {
      "forward" => {
        horiz += instr.1;
      },
      "up" => {
        depth -= instr.1;
      },
      "down" => {
        depth += instr.1;
      }
      _ => { panic!("Unexpected instruction {}", instr.0); }
    }
  }

  println!("You are at {} horiz, {} down (product {})", horiz, depth, horiz * depth);
}

fn part2<'a>(plan: impl Iterator<Item=&'a(&'a str, i32)>) {
  let mut horiz = 0;
  let mut depth = 0;
  let mut aim = 0;

  for instr in plan {
    match instr.0 {
      "forward" => {
        horiz += instr.1;
        depth += aim * instr.1
      },
      "up" => {
        aim -= instr.1;
      },
      "down" => {
        aim += instr.1;
      }
      _ => { panic!("Unexpected instruction {}", instr.0); }
    }
  }

  println!("You are at {} horiz, {} down (product {})", horiz, depth, horiz * depth);
}


fn main() {
  let input_str = input_string();
  let parsed_input = parse_input(&input_str);

  part1(parsed_input.iter());
  part2(parsed_input.iter());
}
