use std::fs;
use std::env;
use std::collections::HashMap;
use std::str::Lines;

fn input_string() -> String {
  let args: Vec<String> = env::args().collect();
  let filename = match args.get(1) {
    Some(a) if a == "test" => "test_data.txt",
    _ => "data.txt"
  };
  fs::read_to_string(filename).unwrap()
}

#[derive(Clone, Copy, PartialEq)]
enum State { Dark, Light }
use State::{Dark, Light};

#[derive(Clone)]
struct InfinitePlane {
    known: HashMap<(i32, i32), State>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
    infinite_reaches: State
}
impl InfinitePlane {
    fn new(map: &Vec<&str>) -> InfinitePlane {
        InfinitePlane {
            known: HashMap::from_iter(map.iter().enumerate().flat_map(|(y, line)|
                line.chars().enumerate().map(move |(x, cell)| {
                    ((x as i32, y as i32), match cell {
                        '#' => Light,
                        '.' => Dark,
                        _ => unreachable!()
                    })
            }))),
            min_x: 0,
            max_x: map[0].len() as i32,
            min_y: 0,
            max_y: map.len() as i32,
            infinite_reaches: Dark
        }
    }
    fn get(&self, x: i32, y: i32) -> State {
        match self.known.get(&(x, y)) {
            Some(state) => *state,
            None => self.infinite_reaches
        }
    }

    fn set(&mut self, x: i32, y: i32, state: State) {
        self.known.insert((x, y), state);
    }

    fn enhance(&mut self, algorithm: &Vec<State>) {
        let mut new_known = HashMap::<(i32, i32), State>::new();
        for x in self.min_x-1..=self.max_x+1 {
            for y in self.min_y-1..=self.max_y+1 {
                let key = [
                    self.get(x-1,y-1), self.get(x,y-1), self.get(x+1,y-1),
                    self.get(x-1,y), self.get(x,y), self.get(x+1,y),
                    self.get(x-1,y+1), self.get(x,y+1), self.get(x+1,y+1)
                ];
                let mut key_i = 0;
                for state in key {
                    key_i *= 2;
                    if state == Light { key_i += 1; }
                }
                new_known.insert((x,y), algorithm[key_i]);
            }
        }
        self.known = new_known;
        self.min_x -= 1;
        self.max_x += 1;
        self.min_y -= 1;
        self.max_y += 1;
        self.infinite_reaches = match self.infinite_reaches {
            Dark => algorithm[0],
            Light => algorithm[511]
        }
    }

    fn print(&self) {
        for y in self.min_x-1..=self.max_x+1 {
            for x in self.min_y-1..=self.max_y+1 {
                print!("{}", match self.get(x,y) {
                    Dark => '.',
                    Light => '#'
                });
            }
            println!("");
        }
    }

    fn lit_count(&self) -> usize {
        self.known.values().filter(|s| **s == Light).count()
    }
}

fn parse_input(input: &String) -> (Vec<State>, Vec<&str>) {
    let mut lines = input.lines();
    let algo = lines.by_ref().next().unwrap().chars().map(|c| match c {
        '.' => Dark,
        '#' => Light,
        _ => unreachable!()
    }).collect();
    lines.by_ref().next();
    (algo, lines.collect())
}

fn parts1and2(plane: &mut InfinitePlane, algorithm: &Vec<State>, times: usize) {
    for _ in 0..times {
        plane.enhance(algorithm);
    }
    println!("Lit: {}", plane.lit_count());
}

fn main() {
    let input = input_string();
    let (algorithm, map) = parse_input(&input);
    let plane = InfinitePlane::new(&map);
    parts1and2(&mut plane.clone(), &algorithm, 2);
    parts1and2(&mut plane.clone(), &algorithm, 50);
}
