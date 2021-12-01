use std::fs;

fn input_string() -> String {
  fs::read_to_string("data.txt").unwrap()
}

fn main() {
  let input_str = input_string();
  let depths_iter = input_str.split_whitespace();

  let mut increase_count = 0;
  let mut last_depth: Option<i32> = None;

  for depth in depths_iter {
    let depth_i = depth.parse::<i32>().unwrap();
    match last_depth {
      Some(x) => {
        if x < depth_i {
          increase_count += 1;
        }
      }
      None => {}
    }

    last_depth = Some(depth_i);
  }

  println!("Increased {} times", increase_count);
}
