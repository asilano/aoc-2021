use std::fs;
use std::env;
use std::fmt;

fn input_string() -> String {
  let args: Vec<String> = env::args().collect();
  let filename = match args.get(1) {
    Some(a) if a == "test" => "test_data.txt",
    _ => "data.txt"
  };
  fs::read_to_string(filename).unwrap()
}

#[derive(Clone)]
struct Octopus {
    energy: u32,
    flashing: bool,
    flashed: bool,
}
impl fmt::Debug for Octopus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.energy)
    }
}

fn parse_input(input: String) -> Vec<Octopus> {
    let colony: Vec<Octopus> = input.lines().flat_map(|line| line.chars().map(|c| {
        Octopus {
            energy: c.to_digit(10).unwrap(),
            flashing: false,
            flashed: false,
        }
    })).collect();

    colony
}

fn tick(colony: &mut Vec<Octopus>, flashes: &mut u32) {
    // First, increment every Octopus's energy
    for octo in colony.iter_mut() {
        octo.energy += 1;
        if octo.energy >= 10 {
            *flashes += 1;
            octo.flashing = true;
        }
    }

    // Now, as long as any octopus is flashing...
    while colony.iter().any(|o| o.flashing) {
        let mut ixes_to_update = Vec::<usize>::new();

        // Propagate that flash by marking the flashing oct as flashed
        // and bumping its neighbours
        for (ix, octo) in colony.iter_mut().enumerate() {
            if octo.flashing {
                octo.flashing = false;
                octo.flashed = true;
                octo.energy = 0;

                if ix > 9 {
                    // Add row above
                    if ix % 10 != 0 {
                        ixes_to_update.push(ix - 11);
                    }
                    ixes_to_update.push(ix - 10);
                    if ix % 10 != 9 {
                        ixes_to_update.push(ix - 9);
                    }
                }
                if ix % 10 != 0 {
                    ixes_to_update.push(ix - 1);
                }
                if ix % 10 != 9 {
                    ixes_to_update.push(ix + 1);
                }
                if ix < 90 {
                    // Add row below
                    if ix % 10 != 0 {
                        ixes_to_update.push(ix + 9);
                    }
                    ixes_to_update.push(ix + 10);
                    if ix % 10 != 9 {
                        ixes_to_update.push(ix + 11);
                    }
                }

            }
        }

        for update_ix in ixes_to_update.iter() {
            if !colony[*update_ix].flashed {
                colony[*update_ix].energy += 1;

                if colony[*update_ix].energy == 10 {
                    *flashes += 1;
                    colony[*update_ix].flashing = true;
                }
            }
        }
    }

    // At the end of the tick, all octopodes stop flashing and being flashed
    for octo in colony.iter_mut() {
        octo.flashing = false;
        octo.flashed = false;
    }
}

fn part1(colony: &mut Vec<Octopus>) {
    let mut flashes = 0;
    for _i in 0..100 {
        tick(colony, &mut flashes);
    }

    println!("Total flashes: {}", flashes);
    //println!("{:?}", colony);
}

fn part2(colony: &mut Vec<Octopus>) {
    let mut ix = 0;
    loop {
        let mut flashes = 0;
        ix += 1;
        tick(colony, &mut flashes);
        if flashes == 100 { break; }
    }

    println!("Synched at: {}", ix);
    //println!("{:?}", colony);
}


fn main() {
    let input = input_string();
    let colony = parse_input(input);
    part1(&mut colony.clone());
    part2(&mut colony.clone());
}
