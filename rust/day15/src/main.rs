use std::fs;
use std::env;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

fn input_string() -> String {
  let args: Vec<String> = env::args().collect();
  let filename = match args.get(1) {
    Some(a) if a == "test" => "test_data.txt",
    _ => "data.txt"
  };
  fs::read_to_string(filename).unwrap()
}

type Edge = (usize, usize);

struct Cell {
    location: (usize, usize),
    risk: u32,

    // Fields for Dijkstra
    edges_out: Vec<Edge>
}

struct DijkState {
    location: (usize, usize),
    best_dist: u32,
    parent: Option<(usize, usize)>
}
impl PartialEq for DijkState {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location
    }
}
impl Eq for DijkState {}
impl Ord for DijkState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.best_dist.cmp(&self.best_dist)
            .then_with(|| self.location.cmp(&other.location))
    }
}
impl PartialOrd for DijkState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

type Floor = HashMap<(usize, usize), Cell>;

fn parse_input(input: &String, quin: bool) -> (Floor, usize, usize) {
    let base_rows = input.lines().count();
    let rows = base_rows * (if quin { 5 } else { 1 });
    let base_cols = input.lines().next().unwrap().len();
    let cols = base_cols * (if quin { 5 } else { 1 });

    let mut lines = input.lines().cycle();

    let mut floor = HashMap::new();
    for row in 0..rows {
        let line = lines.next().unwrap();
        let mut risks = line.chars().cycle();
        for col in 0..cols {
            let mut neighbours = Vec::<(usize, usize)>::new();
            if row > 0 { neighbours.push((row - 1, col)) }
            if col > 0 { neighbours.push((row, col - 1)) }
            if row < rows - 1 { neighbours.push((row + 1, col)) }
            if col < cols - 1 { neighbours.push((row, col + 1)) }

            let risk = risks.next().unwrap();
            let actual_risk = ((risk.to_digit(10).unwrap() as usize + row / base_rows + col / base_cols) % 9) as u32;

            floor.insert((row, col), Cell {
                location: (row, col),
                risk: if actual_risk == 0 { 9 } else { actual_risk },
                edges_out: neighbours
            });
        }

    };

    (floor, rows, cols)
}

fn part1(floor: &Floor, rows: usize, cols: usize) {
    let mut heap = BinaryHeap::new();
    let start = floor.get(&(0,0)).unwrap();
    heap.push(DijkState {
        location: start.location,
        best_dist: 0,
        parent: None
    });

    let mut best_dists = HashMap::from([((0,0), 0)]);

    while let Some(current) = heap.pop() {
        if current.location == (rows - 1, cols - 1) {
            println!("Reached end with risk: {}", current.best_dist);
            return;
        }

        if *best_dists.get(&current.location).unwrap() < current.best_dist {
            // Superseded
            continue;
        }

        let cell = floor.get(&current.location).unwrap();
        for edge in cell.edges_out.iter() {
            let next_cell = floor.get(edge).unwrap();
            let next = DijkState {
                location: *edge,
                best_dist: current.best_dist + next_cell.risk,
                parent: Some(current.location)
            };

            let best_so_far = best_dists.get(edge);
            if best_so_far == None || next.best_dist < *best_so_far.unwrap() {
                best_dists.insert(*edge, next.best_dist);
                heap.push(next);
            }
        }
    }
}

fn main() {
    let input = input_string();

    let (floor_pt1, rows, cols) = parse_input(&input, false);
    part1(&floor_pt1, rows, cols);

    let (floor_pt2, rows, cols) = parse_input(&input, true);
    part1(&floor_pt2, rows, cols);
}
