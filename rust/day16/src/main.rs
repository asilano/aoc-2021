use std::fs;
use std::env;

fn input_string() -> String {
  let args: Vec<String> = env::args().collect();
  let filename = match args.get(1) {
    Some(a) if a == "test" => "test_data.txt",
    Some(a) if a == "test2" => "test_data_2.txt",
    _ => "data.txt"
  };
  fs::read_to_string(filename).unwrap()
}

fn handle_literal_packet(bits_iter: &mut dyn Iterator<Item = char>) -> u64 {
    println!("  Literal packet");
    let mut value_str = String::new();
    loop {
        let flag = bits_iter.next().unwrap();
        for c in bits_iter.take(4) { value_str.push(c); }
        if flag == '0' { break }
    }

    let value = u64::from_str_radix(&value_str, 2).unwrap();
    println!("  Value: {}", value);
    value
}

fn handle_nested_packets_by_len<I>(sub_bits_iter: &mut std::iter::Peekable<I>, part1_vn_sum: &mut u32) -> Vec<u64>
    where I: Iterator<Item = char> {
    let mut values = Vec::new();
    while sub_bits_iter.peek() != None {
        values.push(parse_packet(sub_bits_iter, part1_vn_sum));
    }
    values
}

fn handle_nested_packets_by_count(bits_iter: &mut dyn Iterator<Item = char>, part1_vn_sum: &mut u32, sub_count: usize) -> Vec<u64> {
    (0..sub_count).map(|_| parse_packet(bits_iter, part1_vn_sum)).collect()
}

fn get_subpackets_for_operator(bits_iter: &mut dyn Iterator<Item = char>, part1_vn_sum: &mut u32) -> Vec<u64> {
    println!("  Operator packet");
    match bits_iter.next() {
        Some('0') => {
            let length_str = bits_iter.take(15).collect::<String>();
            if length_str.len() < 15 { return vec![] }
            let length = usize::from_str_radix(&length_str, 2).unwrap();
            println!("  Contains {} bits of packet", length);
            handle_nested_packets_by_len(&mut bits_iter.take(length).peekable(), part1_vn_sum)
        },
        Some('1') => {
            let count_str = bits_iter.take(11).collect::<String>();
            if count_str.len() < 11 { return vec![] }
            let count = usize::from_str_radix(&count_str, 2).unwrap();
            println!("  Contains {} packets", count);
            handle_nested_packets_by_count(bits_iter, part1_vn_sum, count)
        },
        None => { println!("  Stop - out of characters"); return vec![]; },
        _ => unreachable!()
    }
}

fn handle_sum_packet(bits_iter: &mut dyn Iterator<Item = char>, part1_vn_sum: &mut u32) -> u64 {
    println!("  Sum packet");
    get_subpackets_for_operator(bits_iter, part1_vn_sum).iter().sum()
}
fn handle_product_packet(bits_iter: &mut dyn Iterator<Item = char>, part1_vn_sum: &mut u32) -> u64 {
    println!("  Sum packet");
    get_subpackets_for_operator(bits_iter, part1_vn_sum).iter().product()
}
fn handle_minimum_packet(bits_iter: &mut dyn Iterator<Item = char>, part1_vn_sum: &mut u32) -> u64 {
    println!("  Sum packet");
    *get_subpackets_for_operator(bits_iter, part1_vn_sum).iter().min().unwrap()
}
fn handle_maximum_packet(bits_iter: &mut dyn Iterator<Item = char>, part1_vn_sum: &mut u32) -> u64 {
    println!("  Sum packet");
    *get_subpackets_for_operator(bits_iter, part1_vn_sum).iter().max().unwrap()
}
fn handle_greater_packet(bits_iter: &mut dyn Iterator<Item = char>, part1_vn_sum: &mut u32) -> u64 {
    println!("  Sum packet");
    let opands = get_subpackets_for_operator(bits_iter, part1_vn_sum);
    if opands[0] > opands[1] { 1 } else { 0 }
}
fn handle_less_packet(bits_iter: &mut dyn Iterator<Item = char>, part1_vn_sum: &mut u32) -> u64 {
    println!("  Sum packet");
    let opands = get_subpackets_for_operator(bits_iter, part1_vn_sum);
    if opands[0] < opands[1] { 1 } else { 0 }
}
fn handle_equal_packet(bits_iter: &mut dyn Iterator<Item = char>, part1_vn_sum: &mut u32) -> u64 {
    println!("  Sum packet");
    let opands = get_subpackets_for_operator(bits_iter, part1_vn_sum);
    if opands[0] == opands[1] { 1 } else { 0 }
}

fn parse_packet(bits_iter: &mut dyn Iterator<Item = char>, part1_vn_sum: &mut u32) -> u64 {
    let version_str = bits_iter.take(3).collect::<String>();
    if version_str.len() < 3 { return 0 }
    let type_str = bits_iter.take(3).collect::<String>();
    if type_str.len() < 3 { return 0 }

    println!("Parsing packet");
    println!("  Version: {}", version_str);
    *part1_vn_sum += u32::from_str_radix(&version_str, 2).unwrap();

    println!("  Type: {}", type_str);
    match type_str.as_str() {
        "100" => { handle_literal_packet(bits_iter) },
        "000" => { handle_sum_packet(bits_iter, part1_vn_sum) }
        "001" => { handle_product_packet(bits_iter, part1_vn_sum) }
        "010" => { handle_minimum_packet(bits_iter, part1_vn_sum) }
        "011" => { handle_maximum_packet(bits_iter, part1_vn_sum) }
        "101" => { handle_greater_packet(bits_iter, part1_vn_sum) }
        "110" => { handle_less_packet(bits_iter, part1_vn_sum) }
        "111" => { handle_equal_packet(bits_iter, part1_vn_sum) }
        _ => { unreachable!();}//handle_operator_packet(bits_iter, part1_vn_sum); }
    }
}

fn parse_BITS_string(input: &String) {
    let mut part1_vn_sum = 0u32;

    let mut bits_iter = input
        .chars()
        .flat_map(|x| format!("{:04b}", x.to_digit(16).unwrap()).chars().collect::<Vec<_>>())
        .peekable();

    let value = parse_packet(&mut bits_iter, &mut part1_vn_sum);

    println!("** Total version: {}", part1_vn_sum);
    println!("** Value: {}", value);
}

fn main() {
    let input = input_string();

    for line in input.lines() {
        println!("BITS: {}", line);
        parse_BITS_string(&line.to_string())
    }
}
