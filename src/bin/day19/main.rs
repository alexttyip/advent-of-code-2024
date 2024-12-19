#![feature(test)]

use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

fn parse() -> (Vec<String>, String) {
    let (stripes_str, towels_str) = include_str!("input.txt").split_once("\n\n").unwrap();
    let stripes = stripes_str
        .split(", ")
        .map(|s| s.to_owned())
        .sorted_unstable()
        .collect_vec();

    (stripes, towels_str.to_owned())
}

fn part1() -> u64 {
    let (stripes, towels_str) = parse();

    let mut ans = 0;
    let mut reachable = HashSet::new();

    for towel in towels_str.lines() {
        reachable.clear();
        reachable.insert(0);

        for idx in 0..towel.len() {
            if !reachable.contains(&idx) {
                continue;
            };

            let mut finished = false;

            let low = stripes.partition_point(|stripe| towel[idx..idx + 1] > stripe[0..1]);

            for stripe in &stripes[low..] {
                if towel[idx..] == *stripe {
                    finished = true;
                    break;
                }

                if towel[idx..].starts_with(stripe) {
                    reachable.insert(idx + stripe.len());
                } else {
                    let cmp_length = towel[idx..].len().min(stripe.len());
                    if towel[idx..idx + cmp_length] < stripe[0..cmp_length] {
                        break;
                    }
                }
            }

            if finished {
                ans += 1;
                break;
            }
        }
    }

    ans
}

fn part2() -> u64 {
    let (stripes, towels_str) = parse();

    let mut ans = 0;
    let mut parents_count_for_char = HashMap::new();

    for towel in towels_str.lines() {
        parents_count_for_char.clear();
        parents_count_for_char.insert(0, 1);

        for idx in 0..towel.len() {
            let Some(&self_count) = parents_count_for_char.get(&idx) else {
                continue;
            };

            let low = stripes.partition_point(|stripe| towel[idx..idx + 1] > stripe[0..1]);

            for stripe in &stripes[low..] {
                if towel[idx..] == *stripe {
                    ans += self_count;
                    continue;
                }

                if towel[idx..].starts_with(stripe) {
                    *parents_count_for_char
                        .entry(idx + stripe.len())
                        .or_default() += self_count;
                } else {
                    let cmp_length = towel[idx..].len().min(stripe.len());
                    if towel[idx..idx + cmp_length] < stripe[0..cmp_length] {
                        break;
                    }
                }
            }
        }
    }

    ans
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 19 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 300);
    assert_eq!(part2, 624802218898092);
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
