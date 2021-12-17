use std::fs;
use std::env;
use std::ops::RangeInclusive;
use regex::Regex;

fn input_string() -> String {
  let args: Vec<String> = env::args().collect();
  let filename = match args.get(1) {
    Some(a) if a == "test" => "test_data.txt",
    _ => "data.txt"
  };
  fs::read_to_string(filename).unwrap()
}

fn parse_input(input: String) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let matcher = Regex::new(r"target area: x=(-?\d*)\.\.(-?\d*), y=(-?\d*)\.\.(-?\d*)").unwrap();
    let caps = matcher.captures(&input).unwrap();

    (caps.get(1).unwrap().as_str().parse::<i32>().unwrap()..=caps.get(2).unwrap().as_str().parse::<i32>().unwrap(),
     caps.get(3).unwrap().as_str().parse::<i32>().unwrap()..=caps.get(4).unwrap().as_str().parse::<i32>().unwrap())
}

fn calc_valid_dx(target_x: &RangeInclusive<i32>) -> RangeInclusive<i32> {
    if *target_x.start() < 0 || *target_x.end() < 0 { unreachable!() }

    // Maximal x-distance travelled is triangular, so 0.5*dx(dx+1). Therefore we need dx(dx+1) >= 2*target_x.start
    // dx^2+dx-2*start >= 0. Quadratic formula says
    // dx >= (-1 + sqrt(1+8*start))/2
    let min_dx = ((((1 + 8 * target_x.start()) as f64).sqrt() - 1f64) / 2f64).ceil() as i32;

    // And any dx greater than the far end of the target is plainly too large
    min_dx..=*target_x.end()
}

fn calc_valid_dy(target_y: &RangeInclusive<i32>) -> RangeInclusive<i32> {
    if *target_y.start() > 0 || *target_y.end() > 0 { unreachable!() }

    // We'll overshoot the target if our downward speed as we pass 0 is greater than the the bottom of the target
    // Therefore, our range of valid speeds is...
    *target_y.start() ..= -target_y.start()
}

fn parts1and2(target_x: &RangeInclusive<i32>, target_y: &RangeInclusive<i32>, valid_dx_range: &RangeInclusive<i32>, valid_dy_range: &RangeInclusive<i32>) {
    println!("{:?} {:?} {:?} {:?}", target_x, target_y, valid_dx_range, valid_dy_range);
    let mut max_dy = *valid_dy_range.start();
    let mut valid = 0;

    for start_dx in valid_dx_range.clone() {
        for start_dy in valid_dy_range.clone() {
            let mut dx = start_dx;
            let mut dy = start_dy;
            let mut x = 0i32;
            let mut y = 0i32;
            loop {
                x += dx; y += dy;
                if dx > 0 { dx -= 1 }
                dy -= 1;
                if target_x.contains(&x) && target_y.contains(&y) {
                    if start_dy > max_dy { max_dy = start_dy }
                    valid += 1;
                    break;
                }
                if y < *target_y.start() { break; }
            }
        }
    }

    println!("Best dy: {}. Height: {}", max_dy, max_dy * (max_dy + 1) / 2);
    println!("Total valid: {}", valid);
}

fn main() {
    let input = input_string();
    let (target_x, target_y) = parse_input(input);

    let valid_dx_range = calc_valid_dx(&target_x);
    let valid_dy_range = calc_valid_dy(&target_y);
    parts1and2(&target_x, &target_y, &valid_dx_range, &valid_dy_range);
}
