use std::fs;

fn input_string() -> String {
  fs::read_to_string("data.txt").unwrap()
}

fn main() {
  let input_str = input_string();
  let depths: Vec<i32> = input_str.split_whitespace()
                                  .map(|x| x.parse::<i32>().unwrap())
                                  .collect();

  let windows: Vec<i32> = depths.windows(3).map(|w| w.iter().sum()).collect();
  let increase_count = windows.windows(2)
                              .filter(|wind| wind[1] > wind[0])
                              .count();

  println!("Increased {} times", increase_count);
}
