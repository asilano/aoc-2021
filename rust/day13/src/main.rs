use std::fs;
use std::env;
use std::collections::HashSet;

fn input_string() -> String {
  let args: Vec<String> = env::args().collect();
  let filename = match args.get(1) {
    Some(a) if a == "test" => "test_data.txt",
    _ => "data.txt"
  };
  fs::read_to_string(filename).unwrap()
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: u32,
    y: u32
}
enum Direction {
    Up, Left
}
type Manual = (HashSet<Point>, Vec<(Direction, u32)>);

fn parse_input(input: String) -> Manual {
    let mut lines = input.lines();
    let points: HashSet<Point> = HashSet::from_iter(
        lines.by_ref().take_while(|l| !l.trim().is_empty())
                        .map(|l| {
                            let (x, y) = l.split_once(',').unwrap();
                            Point {
                                x: x.parse::<u32>().unwrap(),
                                y: y.parse::<u32>().unwrap()
                            }
                        }));
    let folds: Vec<(Direction, u32)> = lines.map(|l| {
        match l.split_whitespace().last().unwrap().split_once('=') {
            Some((cardinal, pt)) => {
                if cardinal == "x" {
                    (Direction::Left, pt.parse::<u32>().unwrap())
                }
                else {
                    (Direction::Up, pt.parse::<u32>().unwrap())
                }
            },
            _ => unreachable!()
        }
    }).collect();

    (points, folds)
}

fn fold_once(dots: &mut HashSet<Point>, fold: &(Direction, u32)) {
    let moving_dots: Vec<Point> = Vec::from_iter(dots.iter().filter(|dot| {
        (match &fold.0 {
            Direction::Up => dot.y,
            Direction::Left => dot.x
        }) > fold.1
    }).cloned());

    dots.retain(|dot| !moving_dots.contains(dot));
    dots.extend(moving_dots.iter().map(|dot|
        match &fold.0 {
            Direction::Up => { Point { x: dot.x, y: &fold.1 - (dot.y - &fold.1) }},
            Direction::Left => { Point { x: &fold.1 - (dot.x - &fold.1), y: dot.y }}
        }
    ));
}

fn output_manual(dots: &HashSet<Point>) {
    let x_extent = dots.iter().max_by_key(|dot| dot.x).unwrap().x;
    let y_extent = dots.iter().max_by_key(|dot| dot.y).unwrap().y;

    for y in 0..=y_extent {
        for x in 0..=x_extent {
            print!("{}", if dots.contains(&Point { x, y }) {'#'} else {' '});
        }
        println!("");
    }
}

fn parts1and2(mut manual: Manual) {
    let mut folds_iter = manual.1.iter();
    fold_once(&mut manual.0, folds_iter.by_ref().next().unwrap());

    println!("{} dots remain", manual.0.len());

    for fold in folds_iter {
        fold_once(&mut manual.0, fold);
    }

    output_manual(&manual.0);
}

fn main() {
    let manual = parse_input(input_string());
    parts1and2(manual);
}
