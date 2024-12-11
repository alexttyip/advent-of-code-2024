#![feature(test)]

use num::Integer;
use std::collections::HashMap;
use std::mem::swap;
use std::time::Instant;

fn digits(i: u64) -> u32 {
    i.checked_ilog10().unwrap_or(0) + 1
}

fn update(state: &mut HashMap<u64, u64>, i: u64, count: u64) {
    *state.entry(i).or_default() += count;
}

fn simulate(part1: bool) -> u64 {
    let mut state: HashMap<u64, u64> = include_str!("input.txt")
        .split_ascii_whitespace()
        .filter_map(|s| Some((s.parse::<u64>().ok()?, 1)))
        .collect();

    let mut new_state = HashMap::new();

    for _ in 0..(if part1 { 25 } else { 75 }) {
        for (&i, &count) in state.iter() {
            if i == 0 {
                update(&mut new_state, 1, count);
                continue;
            }

            let digits = digits(i).div_rem(&2);

            if digits.1 == 0 {
                let (div, rem) = i.div_rem(&(10u64.pow(digits.0)));
                update(&mut new_state, div, count);
                update(&mut new_state, rem, count);
                continue;
            }

            update(&mut new_state, i * 2024, count);
        }

        swap(&mut state, &mut new_state);
        new_state.clear();
    }

    state.values().sum()
}

fn part1() -> u64 {
    simulate(true)
}
fn part2() -> u64 {
    simulate(false)
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 11 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 185205);
    assert_eq!(part2, 221280540398419);
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        b.iter(part1);
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        b.iter(part2);
    }
}
