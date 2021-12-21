use std::fs;
use std::env;
use ndarray::{arr1, arr2, Array1, Array2};



fn rotations() -> [Array2<i32>; 24] {
    let IDENT: Array2<i32> = arr2(&[
        [ 1,  0,  0],
        [ 0,  1,  0],
        [ 0,  0,  1]
    ]);
    let XR: Array2<i32> = arr2(&[
        [ 1,  0,  0],
        [ 0,  0,  1],
        [ 0, -1,  0]
    ]);
    let YR: Array2<i32> = arr2(&[
        [ 0,  0,  1],
        [ 0,  1,  0],
        [-1,  0,  0]
    ]);
    [
        IDENT.clone(),
        XR.clone(), XR.dot(&XR), XR.dot(&XR).dot(&XR),
        YR.clone(), YR.dot(&XR), YR.dot(&XR).dot(&XR), YR.dot(&XR).dot(&XR).dot(&XR),
        YR.dot(&YR), YR.dot(&YR).dot(&XR), YR.dot(&YR).dot(&XR).dot(&XR), YR.dot(&YR).dot(&XR).dot(&XR).dot(&XR),
        YR.dot(&YR).dot(&YR), YR.dot(&YR).dot(&YR).dot(&XR), YR.dot(&YR).dot(&YR).dot(&XR).dot(&XR), YR.dot(&YR).dot(&YR).dot(&XR).dot(&XR).dot(&XR),
        XR.dot(&YR), XR.dot(&YR).dot(&XR), XR.dot(&YR).dot(&XR).dot(&XR), XR.dot(&YR).dot(&XR).dot(&XR).dot(&XR),
        XR.dot(&XR).dot(&XR).dot(&YR), XR.dot(&XR).dot(&XR).dot(&YR).dot(&XR), XR.dot(&XR).dot(&XR).dot(&YR).dot(&XR).dot(&XR), XR.dot(&XR).dot(&XR).dot(&YR).dot(&XR).dot(&XR).dot(&XR)
    ]
}

fn input_string() -> String {
  let args: Vec<String> = env::args().collect();
  let filename = match args.get(1) {
    Some(a) if a == "test" => "test_data.txt",
    _ => "data.txt"
  };
  fs::read_to_string(filename).unwrap()
}

// Turn the input into a vector<sensor> of vector<beacon 3-point>
fn parse_input(input: String) -> Vec<Vec<Array1<i32>>> {
    let mut output = Vec::<Vec<Array1<i32>>>::new();
    let mut builder: &mut Vec<Array1<i32>> = &mut Vec::new();

    for line in input.lines() {
        if line.starts_with("---") {
            output.push(Vec::<Array1<i32>>::new());
            builder = output.last_mut().unwrap();
            continue;
        }
        if line.is_empty() { continue; }

        builder.push(arr1(&line.split(',').map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>()))
    }

    output
}

#[derive(Clone, Eq)]
struct Offset {
    from_ix: usize,
    to_ix: usize,
    offset: Array1<i32>
}
impl PartialEq for Offset {
    fn eq(&self, other: &Self) -> bool {
        self.offset == other.offset
    }
}

fn make_offsets(one_sensed: &Vec<Array1<i32>>) -> Vec<Offset> {
    let mut output = Vec::<Offset>::new();
    for src in 0..one_sensed.len() {
        for dest in 0..one_sensed.len(){
            if src == dest { continue; }
            output.push(Offset {
                from_ix: src,
                to_ix: dest,
                offset: &one_sensed[dest] - &one_sensed[src]
            });
        }
    }
    output
}

fn combine_sensor_maps(sensed: &mut Vec<Vec<Array1<i32>>>, offsets: &mut Vec<Vec<Offset>>) {
    let mut sensor_locations = Vec::<Vec<Array1<i32>>>::new();
    for _ in 0..sensed.len() {
        sensor_locations.push(vec![arr1(&[0,0,0])]);
    }

    while sensed.len() > 1 {
        let mut keep_ix = None::<usize>;
        let mut rotate_ix = None::<usize>;
        let mut rotn = None::<Array2<i32>>;
        let mut keep_from_ix = None::<usize>;
        let mut rotate_from_ix = None::<usize>;

        println!("Considering {} sets", sensed.len());
        'outer: for x in 0..offsets.len() {
            for y in x+1..offsets.len() {
                let source = &offsets[x];
                let candidate = &offsets[y];

                for rotation in rotations() {
                    let mut rotated_offsets = candidate.clone();
                    for offset in rotated_offsets.iter_mut() {
                        offset.offset = rotation.dot(&offset.offset);
                    }
                    let match_count = rotated_offsets.iter().filter(|o| source.contains(*o)).count();
                    if match_count >= 132 {
                        //println!("Sensors {} and {} match {} times with {:?}", x, y, match_count, rotation);
                        keep_ix = Some(x);
                        rotate_ix = Some(y);
                        rotn = Some(rotation.clone());

                        for ro in rotated_offsets.iter() {
                            let koo = source.iter().find(|so| *so == ro);
                            match koo {
                                Some(ko) => {
                                    keep_from_ix = Some(ko.from_ix);
                                    rotate_from_ix = Some(ro.from_ix);
                                    break;
                                },
                                None => {}
                            }
                        };
                        break 'outer;
                    }
                }
            }
        }

        // Found a matching set of offsets for sensor-sets. Rotate the sensed locations of the beacons in the
        // second set.
        let rotn_mx = rotn.unwrap();
        for beacon in sensed[rotate_ix.unwrap()].iter_mut() {
            *beacon = rotn_mx.dot(&beacon.clone());
        }

        // Now determine the relative recorded position of matching beacons
        let keep_beacon = &sensed[keep_ix.unwrap()][keep_from_ix.unwrap()];
        let rotate_beacon = &sensed[rotate_ix.unwrap()][rotate_from_ix.unwrap()];
        let translate = keep_beacon - rotate_beacon;

        // For each beacon in the rotating set, translate it to the keeping set's frame of reference then add it
        // if it's not already there.
        let mut holding_vec = Vec::<Array1<i32>>::new();
        for beacon in sensed[rotate_ix.unwrap()].iter() {
            let translated_beacon = beacon + &translate;
            if !sensed[keep_ix.unwrap()].contains(&translated_beacon) {
                holding_vec.push(translated_beacon);
            }
        }
        sensed[keep_ix.unwrap()].append(&mut holding_vec);
        sensed.remove(rotate_ix.unwrap());
        offsets.remove(rotate_ix.unwrap());
        offsets[keep_ix.unwrap()] = make_offsets(&sensed[keep_ix.unwrap()]);

        // Update the sensors in the rotating set as well and translate them _away_, before adding them to the fixed set
        holding_vec.clear();
        for sensor in sensor_locations[rotate_ix.unwrap()].iter() {
            holding_vec.push(rotn_mx.dot(sensor) + &translate);
        }
        sensor_locations[keep_ix.unwrap()].append(&mut holding_vec);
        sensor_locations.remove(rotate_ix.unwrap());

        println!("Moved {} into {}; latter now has {} elements", rotate_ix.unwrap(), keep_ix.unwrap(), sensed[keep_ix.unwrap()].len());
    }
    println!("({}) - {:#?}", sensor_locations.len(), sensor_locations[0]);
    let mut max_dist = 0;
    let locs = &sensor_locations[0];
    for x in 0..locs.len() {
        for y in x+1..locs.len() {
            let dist = (locs[x][0] - locs[y][0]).abs() + (locs[x][1] - locs[y][1]).abs() + (locs[x][2] - locs[y][2]).abs();
            if dist > max_dist { max_dist = dist }
        }
    }
    println!("Max manhattan: {}", max_dist);
}

fn main() {
    let input = input_string();
    let mut sensed = parse_input(input);
    let mut offsets: Vec<Vec<Offset>> = sensed.iter().map(|one| make_offsets(one)).collect();

    combine_sensor_maps(&mut sensed, &mut offsets);
    sensed[0].sort_by(|a,b| a.get(0).unwrap().cmp(b.get(0).unwrap()));
    //println!("{:#?}", sensed[0]);
}
