use std::fs;
use std::env;
use std::ops::RangeInclusive;
use regex::Regex;
use std::collections::HashSet;

fn input_string() -> String {
  let args: Vec<String> = env::args().collect();
  let filename = match args.get(1) {
    Some(a) if a == "test" => "test_data.txt",
    _ => "data.txt"
  };
  fs::read_to_string(filename).unwrap()
}

struct Instruction {
    turn_on: bool,
    xr: RangeInclusive<i32>,
    yr: RangeInclusive<i32>,
    zr: RangeInclusive<i32>,
}

fn parse_input(input: String) -> Vec<Instruction> {
    let matcher = Regex::new(r"(on|off) x=(-?\d*)\.\.(-?\d*),y=(-?\d*)\.\.(-?\d*),z=(-?\d*)\.\.(-?\d*)").unwrap();
    input.lines().map(|l| {
        let caps = matcher.captures(&l).unwrap();
        Instruction {
            turn_on: caps.get(1).unwrap().as_str() == "on",
            xr: caps.get(2).unwrap().as_str().parse::<i32>().unwrap()..=caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
            yr: caps.get(4).unwrap().as_str().parse::<i32>().unwrap()..=caps.get(5).unwrap().as_str().parse::<i32>().unwrap(),
            zr: caps.get(6).unwrap().as_str().parse::<i32>().unwrap()..=caps.get(7).unwrap().as_str().parse::<i32>().unwrap(),
        }
    }).collect()
}

fn part1(instructions: &Vec<Instruction>) {
    let mut within_fifty = 0;
    for x in -50..=50 {
        for y in -50..=50 {
            for z in -50..=50 {
                for instruct in instructions.iter().rev() {
                    if instruct.xr.contains(&x) && instruct.yr.contains(&y) && instruct.zr.contains(&z) {
                        if instruct.turn_on { within_fifty += 1 }
                        break;
                    }
                }
            }
        }
    }
    println!("On cubes within fifty: {}", within_fifty);
}

#[derive(Copy, Clone, Debug)]
struct Cuboid {
    min_corner: (i32, i32, i32),
    max_corner: (i32, i32, i32)
}
impl Cuboid {
    fn intersection(&self, other: &Self) -> Option<Cuboid> {
        if self.min_corner.0 > other.max_corner.0 ||
            self.max_corner.0 < other.min_corner.0  ||
            self.min_corner.1 > other.max_corner.1 ||
            self.max_corner.1 < other.min_corner.1  ||
            self.min_corner.2 > other.max_corner.2 ||
            self.max_corner.2 < other.min_corner.2 {
            return None;
        }

        Some(Cuboid {
            min_corner: (self.min_corner.0.max(other.min_corner.0),
                         self.min_corner.1.max(other.min_corner.1),
                         self.min_corner.2.max(other.min_corner.2)),
            max_corner: (self.max_corner.0.min(other.max_corner.0),
                         self.max_corner.1.min(other.max_corner.1),
                         self.max_corner.2.min(other.max_corner.2)),
        })
    }

    fn size(&self) -> usize {
        (self.max_corner.0 - self.min_corner.0 + 1) as usize *
        (self.max_corner.1 - self.min_corner.1 + 1) as usize *
        (self.max_corner.2 - self.min_corner.2 + 1) as usize
    }
}

#[derive(Clone, Debug)]
struct Regions {
    positive: Vec<Cuboid>,
    negative: Vec<Cuboid>
}
impl Regions {
    fn add(&mut self, cuboid: Cuboid) {
        // Intersect the subtracted cube with each negative-counting region. These will get added to the positive
        // regions, but we'll need to store them temporarily first.
        let mut holding_vec = Vec::<Cuboid>::new();
        for region in self.negative.iter() {
            match region.intersection(&cuboid) {
                Some(cub) => { holding_vec.push(cub) },
                None => {}
            }
        }

        // Add the new region into the ones that count positive. But wherever it overlaps an existing positive
        // region, first add the intersection into the negative count, since we're now counting it double.
        for region in self.positive.iter() {
            match region.intersection(&cuboid) {
                Some(cub) => { self.negative.push(cub) },
                None => {}
            }
        }

        self.positive.append(&mut holding_vec);
        self.positive.push(cuboid);
    }

    fn subtract(&mut self, cuboid: Cuboid) {
        // Intersect the subtracted cube with each positive-counting region. These will get added to the negative
        // regions, but we'll need to store them temporarily first.
        let mut holding_vec = Vec::<Cuboid>::new();
        for region in self.positive.iter() {
            match region.intersection(&cuboid) {
                Some(cub) => { holding_vec.push(cub) },
                None => {}
            }
        }

        // Intersect the subtracted cube with each negative-counting region. These will get added to the positive
        // regions, since we're now double-counting them.
        for region in self.negative.iter() {
            match region.intersection(&cuboid) {
                Some(cub) => { self.positive.push(cub) },
                None => {}
            }
        }

        self.negative.append(&mut holding_vec);
    }

    fn on_size(&self) -> usize {
        let positive = self.positive.iter().map(|r| r.size()).sum::<usize>();
        let negative = self.negative.iter().map(|r| r.size()).sum::<usize>();

        //println!("{:?} - {:?}", self.positive.iter().map(|r| r.size()).collect::<Vec<usize>>(), self.negative.iter().map(|r| r.size()).collect::<Vec<usize>>());
        positive - negative
    }
}

fn part2(instructions: &Vec<Instruction>) {
    let mut regions = Regions {
        positive: Vec::<Cuboid>::new(),
        negative: Vec::<Cuboid>::new()
    };

    for instruct in instructions {
        if instruct.turn_on {
            regions.add(Cuboid {
                min_corner: (*instruct.xr.start(), *instruct.yr.start(), *instruct.zr.start()),
                max_corner: (*instruct.xr.end(), *instruct.yr.end(), *instruct.zr.end())
            })
        } else {
            regions.subtract(Cuboid {
                min_corner: (*instruct.xr.start(), *instruct.yr.start(), *instruct.zr.start()),
                max_corner: (*instruct.xr.end(), *instruct.yr.end(), *instruct.zr.end())
            })
        }
    }
    println!("On count: {}", regions.on_size());
}

fn main() {
    let instructions = parse_input(input_string());
    part1(&instructions);
    part2(&instructions);
}
