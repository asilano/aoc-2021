use std::fs;
use std::env;
use std::collections::HashMap;
use std::cell::RefCell;
use std::fmt;

fn input_string() -> String {
  let args: Vec<String> = env::args().collect();
  let filename = match args.get(1) {
    Some(a) if a == "test" => "test_data.txt",
    _ => "data.txt"
  };
  fs::read_to_string(filename).unwrap()
}

#[derive(PartialEq)]
enum Reduce {
    None,
    Explode(usize),
    Split(usize)
}

#[derive(Clone)]
struct SnailNumber {
    arena: HashMap<usize, Node>,
    root: usize
}
impl SnailNumber {
    fn iter(&self) -> SnailIter {
        SnailIter { snail_number: self, current_node_index: self.root }
    }

    fn add(&mut self, other: &Self) {
        let next_ident = self.next_ident();
        let right_ident = other.root + next_ident;

        // Move all of other's nodes into self's arena
        for (ident, other_node) in other.arena.iter() {
            let new_ident = ident + next_ident;
            let mut new_node = other_node.clone();
            new_node.ident = new_ident;
            match new_node.parent {
                Some(a) => { new_node.parent = Some(a + next_ident) },
                None => {}
            }
            match new_node.left_child {
                Some(a) => { new_node.left_child = Some(a + next_ident) },
                None => {}
            }
            match new_node.right_child {
                Some(a) => { new_node.right_child = Some(a + next_ident) },
                None => {}
            }
            self.arena.insert(new_ident, new_node);
        }
        let next_ident = self.next_ident();
        self.arena.insert(next_ident, Node {
                                    value: None,
                                    ident: next_ident,
                                    parent: None,
                                    depth: RefCell::new(0),
                                    left_child: Some(self.root),
                                    right_child: Some(right_ident),
                                    previous: RefCell::new(None)
        });

        self.arena.get_mut(&self.root).unwrap().parent = Some(next_ident);
        self.arena.get_mut(&right_ident).unwrap().parent = Some(next_ident);
        self.root = next_ident;
    }

    fn reduce(&mut self) {

        loop {
            let mut action = Reduce::None;
            let mut next = None::<usize>;

            let mut iter = self.iter();
            loop {
                let onode = iter.by_ref().next();
                let node = match onode {
                    Some(a) => a,
                    None => { break; }
                };

                if *node.depth.borrow() >= 5 {
                    action = Reduce::Explode(node.parent.unwrap());

                    // About to break. Absorb the right of the pair
                    iter.by_ref().next();
                    next = match iter.by_ref().next() {
                        Some(n) => Some(n.ident),
                        None => None
                    };
                    break;
                }

                if node.value.unwrap() >= 10 && action == Reduce::None {
                    action = Reduce::Split(node.ident);
                }
            }

            match action {
                Reduce::Explode(ident) => {
                    let left: u32;
                    let right: u32;
                    let prev: Option<usize>;
                    {
                        let exploder = self.arena.get(&ident).unwrap();
                        let left_child = self.arena.get(&exploder.left_child.unwrap()).unwrap();
                        left = left_child.value.unwrap();
                        right = self.arena.get(&exploder.right_child.unwrap()).unwrap().value.unwrap();
                        prev = *left_child.previous.borrow();
                    }
                    if prev != None {
                        let prev_node = self.arena.get_mut(&prev.unwrap()).unwrap();
                        prev_node.value = Some(prev_node.value.unwrap() + left);
                    }
                    if next != None {
                        let next_node = self.arena.get_mut(&next.unwrap()).unwrap();
                        next_node.value = Some(next_node.value.unwrap() + right);
                    }

                    let exploder = self.arena.get_mut(&ident).unwrap();
                    exploder.left_child = None;
                    exploder.right_child = None;
                    exploder.value = Some(0);
                },
                Reduce::Split(ident) => {
                    let left: u32;
                    let right: u32;
                    let left_ident = self.next_ident();
                    let right_ident = left_ident + 1;
                    {
                        let split_node = self.arena.get_mut(&ident).unwrap();
                        left = split_node.value.unwrap() / 2;
                        right = left + split_node.value.unwrap() % 2;
                        split_node.value = None;
                        split_node.left_child = Some(left_ident);
                        split_node.right_child = Some(right_ident);
                    }
                    self.arena.insert(left_ident, Node {
                        value: Some(left),
                        ident: left_ident,
                        parent: Some(ident),
                        depth: RefCell::new(0),
                        left_child: None,
                        right_child: None,
                        previous: RefCell::new(None)
                    });
                    self.arena.insert(right_ident, Node {
                        value: Some(right),
                        ident: right_ident,
                        parent: Some(ident),
                        depth: RefCell::new(0),
                        left_child: None,
                        right_child: None,
                        previous: RefCell::new(None)
                    });
                },
                Reduce::None => { break; }
            }
        }
    }

    fn magnitude(&self) -> u32 {
        self.arena.get(&self.root).unwrap().magnitude(&self.arena)
    }

    fn next_ident(&self) -> usize {
        self.arena.keys().max().unwrap() + 1
    }
}
impl fmt::Debug for SnailNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("Root: {}, {:#?}\n", self.root, self.arena))
    }
}

#[derive(Clone)]
struct Node {
    value: Option<u32>,
    ident: usize,
    parent: Option<usize>,
    depth: RefCell<usize>,
    left_child: Option<usize>,
    right_child: Option<usize>,
    previous: RefCell<Option<usize>>
}
impl Node {
    fn new() -> Node {
        Node {
            value: None,
            ident: 0,
            parent: None,
            depth: RefCell::new(0),
            left_child: None,
            right_child: None,
            previous: RefCell::new(None)
        }
    }
    fn value_node(value: u32, parent: usize) -> Node {
        Node {
            value: Some(value),
            ident: 0,
            parent: Some(parent),
            depth: RefCell::new(0),
            left_child: None,
            right_child: None,
            previous: RefCell::new(None)
        }
    }

    fn magnitude(&self, arena: &HashMap<usize, Node>) -> u32 {
        match self.value {
            Some(n) => { n },
            None => {
                let left_node = arena.get(&self.left_child.unwrap()).unwrap();
                let right_node = arena.get(&self.right_child.unwrap()).unwrap();
                3 * left_node.magnitude(arena) + 2 * right_node.magnitude(arena)
            }
        }
    }
}
impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.value {
            Some(a) => { f.write_fmt(format_args!("{}", a)) },
            None => { f.write_fmt(format_args!("[*{},*{}]", self.left_child.unwrap(), self.right_child.unwrap())) }
        }
    }
}

struct SnailIter<'a> {
    snail_number: &'a SnailNumber,
    current_node_index: usize
}
impl<'a> Iterator for SnailIter<'a> {
    type Item = &'a Node;
    fn next(&mut self) -> Option<Self::Item> {
        let mut current_node = self.snail_number.arena.get(&self.current_node_index).unwrap();
        if self.current_node_index == self.snail_number.root {
            let mut current_depth = 0;
            *current_node.depth.borrow_mut() = 0;

            // Straight down the left side
            while let Some(left_child) = current_node.left_child {
                current_node = self.snail_number.arena.get(&left_child).unwrap();
                current_depth += 1;
                *current_node.depth.borrow_mut() = current_depth;
            }

            if self.current_node_index != self.snail_number.root {
                *current_node.previous.borrow_mut() = Some(self.current_node_index);
            }
            self.current_node_index = current_node.ident;
            return Some(current_node)
        }
        else {
            // Step back up to parent
            let mut current_depth = *current_node.depth.borrow();
            while let Some(parent) = current_node.parent {
                let parent_node = self.snail_number.arena.get(&parent).unwrap();
                current_depth -= 1;
                if parent_node.left_child == Some(current_node.ident) {
                    // Come back up from a left child, so descend to right child then all the way left
                    current_node = self.snail_number.arena.get(&parent_node.right_child.unwrap()).unwrap();
                    current_depth += 1;
                    *current_node.depth.borrow_mut() = current_depth;

                    while let Some(left_child) = current_node.left_child {
                        current_node = self.snail_number.arena.get(&left_child).unwrap();
                        current_depth += 1;
                        *current_node.depth.borrow_mut() = current_depth;
                    }

                    *current_node.previous.borrow_mut() = Some(self.current_node_index);
                    self.current_node_index = current_node.ident;
                    return Some(current_node)
                }
                else { current_node = parent_node }
            }
        }

        None
    }
}

fn parse_node(arena: &mut HashMap<usize, Node>, chars: &mut std::str::Chars, parent: Option<usize>, node_index: &mut usize) {
    let mut node = Node::new();
    node.ident = *node_index;
    node.parent = parent;

    // Left side
    *node_index += 1;
    node.left_child = Some(*node_index);
    match chars.next() {
        Some('[') => {
            parse_node(arena, chars, Some(node.ident), node_index);
        }
        Some(digit) => {
            let mut value_node = Node::value_node(digit.to_digit(10).unwrap(), node.ident);
            value_node.ident = *node_index;
            arena.insert(*node_index, value_node);
        }
        None => unreachable!()
    }

    // Absorb the comma
    assert_eq!(chars.next(), Some(','));

    // Right side
    *node_index += 1;
    node.right_child = Some(*node_index);
    match chars.next() {
        Some('[') => {
            parse_node(arena, chars, Some(node.ident), node_index);
        }
        Some(digit) => {
            let mut value_node = Node::value_node(digit.to_digit(10).unwrap(), node.ident);
            value_node.ident = *node_index;
            arena.insert(*node_index, value_node);
        }
        None => unreachable!()
    }

    // Absorb the close bracket
    assert_eq!(chars.next(), Some(']'));

    arena.insert(node.ident, node);
}

fn parse_input(input: String) -> Vec<SnailNumber> {
    input.lines().map(|line| {
        let mut number = SnailNumber { arena: HashMap::<usize, Node>::new(), root: 0 };
        let mut chars = line.chars();
        let mut node_index = 0;

        // Absorb the opening bracket
        chars.by_ref().next();
        parse_node(&mut number.arena, &mut chars, None, &mut node_index);

        number
    }).collect()
}

fn part1(summands: &Vec<SnailNumber>) {
    let mut total = summands[0].clone();

    for number in summands.iter().skip(1) {
        total.add(number);
        total.reduce();
    }
    println!("{}", total.magnitude());
}

fn part2(summands: &Vec<SnailNumber>) {
    let mut max_mag = 0;

    for lix in 0..summands.len() {
        for rix in 0..summands.len() {
            if lix == rix { continue; }

            let mut left = summands[lix].clone();
            let right = summands[rix].clone();
            left.add(&right);
            left.reduce();
            let magnitude = left.magnitude();
            if magnitude > max_mag {
                max_mag = magnitude;
            }
        }
    }

    println!("{}", max_mag);
}

fn main() {
    let input = input_string();
    let summands = parse_input(input);

    part1(&summands);
    part2(&summands);
}
