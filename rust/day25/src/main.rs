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

fn move_east(seafloor: &mut Vec<Vec<char>>) -> usize {
    let mut to_move = Vec::<(usize, usize)>::new();
    let width = seafloor[0].len();

    for (ix, row) in seafloor.iter().enumerate() {
        for (jx, cell) in row.iter().enumerate() {
            if *cell == '>' && seafloor[ix][(jx + 1) % width] == '.' {
                to_move.push((ix, jx))
            }
        }
    }

    for (row, col) in &to_move {
        seafloor[*row][*col] = '.';
        seafloor[*row][(col + 1) % width] = '>';
    }

    to_move.len()
}
fn move_south(seafloor: &mut Vec<Vec<char>>) -> usize {
    let mut to_move = Vec::<(usize, usize)>::new();
    let height = seafloor.len();

    for (ix, row) in seafloor.iter().enumerate() {
        for (jx, cell) in row.iter().enumerate() {
            if *cell == 'v' && seafloor[(ix + 1) % height][jx] == '.' {
                to_move.push((ix, jx))
            }
        }
    }

    for (row, col) in &to_move {
        seafloor[*row][*col] = '.';
        seafloor[(row + 1) % height][*col] = 'v';
    }

    to_move.len()
}

fn part1(mut seafloor: Vec<Vec<char>>) {
    let mut tick = 0;
    loop {
        tick += 1;
        let mut moved = 0;
        moved += move_east(&mut seafloor);
        moved += move_south(&mut seafloor);

        if moved == 0 { break; }
    }

    println!("Pileup after {}", tick);
    println!("{}", seafloor.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<String>>().join("\n"));
}

fn main() {
    let input = input_string();
    let seafloor: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    part1(seafloor.clone());
}
