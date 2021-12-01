use std::fs;

fn input_string() -> String {
  fs::read_to_string("data.txt").unwrap()
}

fn main() {
  let input_str = input_string();
  let depths: Vec<i32> = input_str.split_whitespace()
                                  .map(|x| x.parse::<i32>().unwrap())
                                  .collect();

  let mut increase_count = 0;
  let mut last_depth: Option<i32> = None;

  for three_depths in depths.windows(3) {
    let depth_sum = three_depths.iter().sum();
    match last_depth {
      Some(x) => {
        if x < depth_sum {
          increase_count += 1;
        }
      }
      None => {}
    }

    last_depth = Some(depth_sum);
  }

  println!("Increased {} times", increase_count);
}
