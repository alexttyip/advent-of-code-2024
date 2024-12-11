#![feature(test)]

use itertools::Itertools;
use std::time::Instant;

const OPERATIONS: [fn(u64, u64) -> u64; 3] = [
    |i, j| i + j,
    |i, j| i * j,
    |i, j| i * 10u64.pow(j.checked_ilog10().unwrap_or(0) + 1) + j,
];

fn parse() -> impl Iterator<Item = (u64, Vec<u64>)> {
    include_str!("input.txt").lines().map(|line| {
        let mut ns = line
            .split(&[':', ' '])
            .filter_map(|s| s.parse::<u64>().ok());

        (ns.next().unwrap(), ns.collect_vec())
    })
}

fn operate(numbers: &[u64], part1: bool) -> Vec<u64> {
    let mut numbers = numbers.to_owned();

    if numbers.len() == 1 {
        return numbers;
    }

    let tail = numbers.pop().unwrap();

    let mut results = Vec::new();

    for num in operate(&numbers, part1) {
        for op in OPERATIONS[..(if part1 { 2 } else { 3 })].iter() {
            results.push(op(num, tail));
        }
    }

    results
}

fn part1() -> u64 {
    parse()
        .filter_map(|(test_value, numbers)| {
            operate(&numbers, true)
                .contains(&test_value)
                .then_some(test_value)
        })
        .sum()
}

fn part2() -> u64 {
    parse()
        .filter_map(|(test_value, numbers)| {
            operate(&numbers, false)
                .contains(&test_value)
                .then_some(test_value)
        })
        .sum()
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 07 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 3119088655389);
    assert_eq!(part2, 264184041398847);
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
