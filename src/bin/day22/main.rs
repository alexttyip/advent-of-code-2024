#![feature(test)]

use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn mix_and_prune(num: i64, new_num: i64) -> i64 {
    (num ^ new_num) % 16777216
}

fn get_secret_numbers(mut num: i64) -> [i64; 2001] {
    let mut sequence = [0; 2001];

    sequence[0] = num;

    for i in sequence[1..].iter_mut() {
        num = mix_and_prune(num, num * 64);
        num = mix_and_prune(num, num / 32);
        num = mix_and_prune(num, num * 2048);
        *i = num;
    }

    sequence
}

fn part1() -> i64 {
    let mut ans = 0;

    for line in include_str!("input.txt").lines() {
        let num = line.parse::<i64>().unwrap();

        ans += get_secret_numbers(num)[2000];
    }

    ans
}

fn part2() -> i64 {
    let mut sequences = HashMap::new();
    let mut line_seq = HashSet::<(i64, i64, i64, i64)>::new();
    let mut secret_numbers: [i64; 2001];

    for line in include_str!("input.txt").lines() {
        let num = line.parse::<i64>().unwrap();

        secret_numbers = get_secret_numbers(num).map(|n| n % 10);
        line_seq.clear();

        (0..2000)
            .map(|i| secret_numbers[i + 1] - secret_numbers[i])
            .tuple_windows()
            .enumerate()
            .filter(|(_, window)| line_seq.insert(*window))
            .for_each(|(i, window)| *sequences.entry(window).or_default() += secret_numbers[i + 4]);
    }

    *sequences.values().max().unwrap()
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 22 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 20506453102);
    assert_eq!(part2, 2423);
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
