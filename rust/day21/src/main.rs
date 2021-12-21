use std::collections::HashMap;
use num_bigint::{BigUint, ToBigUint};

fn part1(p1_start: &u32, p2_start: &u32) {
    let mut p1_now = *p1_start;
    let mut p2_now = *p2_start;
    let mut p1_total = 0;
    let mut p2_total = 0;
    let mut player = 1;
    let mut die = 0;

    loop {
        let roll = 3 * die + 6;
        die += 3;

        if player == 1 {
            p1_now += roll;
            p1_now %= 10;
            if p1_now == 0 { p1_now = 10 }
            p1_total += p1_now;
        }
        else {
            p2_now += roll;
            p2_now %= 10;
            if p2_now == 0 { p2_now = 10 }
            p2_total += p2_now;
        }

        if p1_total >= 1000 || p2_total >= 1000 { break }
        player = (player % 2) + 1;
    }

    println!("{}", p1_total.min(p2_total) * die);
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct State {
    p1_now: u32,
    p2_now: u32,
    p1_score: u32,
    p2_score: u32
}

fn part2(p1_start: &u32, p2_start: &u32) {
    let mut player = 1;
    let mut player_1_universes: BigUint = 0.to_biguint().unwrap();
    let mut player_2_universes: BigUint = 0.to_biguint().unwrap();
    let mut state_counts = HashMap::<State, BigUint>::new();
    let initial_state = State {
        p1_now: *p1_start,
        p2_now: *p2_start,
        p1_score: 0,
        p2_score: 0
    };
    state_counts.insert(initial_state, 1.to_biguint().unwrap());

    while !state_counts.is_empty() {
        let mut temp_counts = HashMap::<State, BigUint>::new();
        for roll in [
            "111",
            "112","121","211",
            "113","122","131","212","221","311",
            "123","132","213","222","231","312","321",
            "133","223","232","313","322","331",
            "233","323","332",
            "333",
        ] {
            let advance: u32 = roll.chars().map(|c| c.to_digit(10).unwrap()).sum();
            for (r_state, count) in &state_counts {
                let mut state = r_state.clone();

                if player == 1 {
                    state.p1_now += advance;
                    state.p1_now %= 10;
                    if state.p1_now == 0 { state.p1_now = 10 }
                    state.p1_score += state.p1_now;
                }
                else {
                    state.p2_now += advance;
                    state.p2_now %= 10;
                    if state.p2_now == 0 { state.p2_now = 10 }
                    state.p2_score += state.p2_now;
                }

                if state.p1_score >= 21 {
                    player_1_universes += count;
                } else if state.p2_score >= 21 {
                    player_2_universes += count;
                } else {
                    let so_far = temp_counts.get_mut(&state);
                    match so_far {
                        Some(n) => { *n += count; },
                        None => { temp_counts.insert(state, count.clone()); }
                    }
                }
            }
        }

        state_counts = temp_counts;
        player = (player % 2) + 1;
    }

    println!("{} ({})", player_1_universes, player_2_universes);
}

fn main() {
    let p1_start = 4;
    let p2_start = 9;
    part1(&p1_start, &p2_start);
    part2(&p1_start, &p2_start);
}
