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

fn parse_input(input: String) -> Vec<Vec<u32>> {
    input.lines().map(|row| row.chars().flat_map(|c| c.to_digit(10)).collect()).collect()
}

fn sinks(depth_map: &Vec<Vec<u32>>) -> Vec<(usize, usize, u32)> {
    let mut sinks = Vec::<(usize, usize, u32)>::new();
    for (y, row) in depth_map.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            let neighbours = [
                if y > 0 {
                    depth_map.get(y - 1).and_then(|r| r.get(x))
                } else { None },
                depth_map.get(y + 1).and_then(|r| r.get(x)),
                if x > 0 {
                    depth_map.get(y).and_then(|r| r.get(x - 1))
                } else { None },
                depth_map.get(y).and_then(|r| r.get(x + 1)),
            ];
            let local_min = neighbours.iter().all(|n| cell < n.unwrap_or(&99));
            if local_min { sinks.push((y, x, *cell)); }
        }
    }
    sinks
}

fn part1(depth_map: &Vec<Vec<u32>>) {
    let mut total_risk = 0;
    for (_y, _x, cell) in sinks(depth_map) { total_risk += cell + 1 }

    println!("Total risk: {}", total_risk);
}

fn part2(depth_map: &Vec<Vec<u32>>) {
    let sinks = sinks(depth_map);
    let mut basin_sizes: Vec<usize> = sinks.iter().map(|(y, x, cell)| {
        let mut size: usize = 0;
        println!("Making basin from {} @ {}, {}", cell, y, x);
        let mut marked = Vec::<(usize, usize)>::new();
        add_cell_to_basin(depth_map, *y, *x, *cell, &mut size, &mut marked);

        size
    }).collect();
    basin_sizes.sort();
    println!("Basin sizes {:?}", basin_sizes);
    let answer: usize = basin_sizes.iter().rev().take(3).product();
    println!("Biggest three product: {}", answer);
}

fn add_cell_to_basin(depth_map: &Vec<Vec<u32>>, y: usize, x: usize, cell: u32, size: &mut usize, marked: &mut Vec<(usize, usize)>) {
    *size += 1;
    let neighbours = [
        if y > 0 {
            Some((y-1, x, depth_map.get(y - 1).and_then(|r| r.get(x))))
        } else { None },
        Some((y+1, x, depth_map.get(y + 1).and_then(|r| r.get(x)))),
        if x > 0 {
            Some((y, x-1, depth_map.get(y).and_then(|r| r.get(x - 1))))
        } else { None },
        Some((y, x+1, depth_map.get(y).and_then(|r| r.get(x + 1)))),
    ];
    for n in neighbours {
        match n {
            Some((adj_y, adj_x, Some(adj_cell))) if *adj_cell > cell &&
                                                    *adj_cell != 9 &&
                                                    !marked.contains(&(adj_y, adj_x))=> {
                println!("  adding {} @ {}, {}", adj_cell, adj_y, adj_x);
                marked.push((adj_y, adj_x));
                add_cell_to_basin(depth_map, adj_y, adj_x, *adj_cell, size, marked);
            },
            _ => {}
        }
    }
}

fn main() {
    let depth_map = parse_input(input_string());

    part1(&depth_map);
    part2(&depth_map);
}
