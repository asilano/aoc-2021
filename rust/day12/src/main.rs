use std::fs;
use std::env;
use std::collections::HashMap;

fn input_string() -> String {
  let args: Vec<String> = env::args().collect();
  let filename = match args.get(1) {
    Some(a) if a == "test" => "test_data.txt",
    _ => "data.txt"
  };
  fs::read_to_string(filename).unwrap()
}

#[derive(Debug, Clone)]
struct Cave {
    label: String,
    large: bool,
    neighbours: Vec<String>,

    visited: usize,
}

type CaveSystem = HashMap<String, Cave>;

fn apply_link(system: &mut CaveSystem, from: &String, to: &String) {
    match system.get_mut(from) {
        Some(cave) => {
            cave.neighbours.push(to.clone());
        },
        None => {
            system.insert(from.clone(),
                Cave {
                    label: from.clone(),
                    large: from.chars().nth(0).unwrap().is_ascii_uppercase(),
                    neighbours: vec![to.clone()],
                    visited: 0
                });
        }
    }
}
fn cave_system(input: String) -> CaveSystem {
    let mut system = HashMap::<String, Cave>::new();
    for line in input.lines() {
        let (label_a, label_b) = line.split_once('-').unwrap();

        apply_link(&mut system, &String::from(label_a), &String::from(label_b));
        apply_link(&mut system, &String::from(label_b), &String::from(label_a));
    }

    system
}

fn visit(label: &String,
            system: &mut CaveSystem,
            paths: &mut usize,
            double_visit: &mut Option<bool>,
            route: &mut Vec<String>) {
    route.push(String::from(label));
    let passages;
    {
        let mut cave = system.get_mut(label).unwrap();
        if cave.label == "end" {
            //println!("{}", route.join(","));
            route.pop();
            *paths += 1;
            return;
        }
        cave.visited += 1;
        if !cave.large && cave.visited == 2 { *double_visit = Some(true); }

        passages = cave.neighbours.clone();
    }

    for passage in passages {
        if passage == "start" { continue; }
        let next = system.get_mut(&passage).unwrap();
        match double_visit {
            None | Some(true) => {
                if next.visited > 0 && !next.large { continue; }
            },
            Some(false) => {
                if next.visited > 1 && !next.large { continue; }
            }
        }


        visit(&passage, system, paths, double_visit, route);
    }

    {
        let mut cave = system.get_mut(label).unwrap();
        if !cave.large && cave.visited == 2 { *double_visit = Some(false); }
        cave.visited -= 1;
    }
    route.pop();
}

fn part1(system: &mut CaveSystem) {
    let mut paths = 0;
    let mut route = Vec::<String>::new();
    visit(&String::from("start"), system, &mut paths, &mut None, &mut route);
    println!("Found {} paths", paths);
}

fn part2(system: &mut CaveSystem) {
    let mut paths = 0;
    let mut route = Vec::<String>::new();
    visit(&String::from("start"), system, &mut paths, &mut Some(false), &mut route);
    println!("Found {} paths", paths);
}

fn main() {
    let mut cave_system = cave_system(input_string());

    part1(&mut cave_system);
    part2(&mut cave_system);
}
