#![feature(test)]

use regex::Regex;
use std::time::Instant;

fn part1() -> u32 {
    let file = include_str!("input.txt");

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(file)
        .map(|cap| {
            cap.extract::<2>()
                .1
                .iter()
                .filter_map(|s| s.parse::<u32>().ok())
                .product::<u32>()
        })
        .sum()
}

fn part2() -> u32 {
    let file = include_str!("input.txt");

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    let mut enabled = true;

    re.captures_iter(file).fold(0, |acc, cap| {
        let mut iter = cap.iter();

        if let Some(Some(m)) = iter.next() {
            let s = m.as_str();

            if s == "don't()" {
                enabled = false;
                return acc;
            } else if s == "do()" {
                enabled = true;
                return acc;
            }
        }

        if !enabled {
            return acc;
        }

        acc + iter
            .filter_map(|m| m.and_then(|mm| mm.as_str().parse::<u32>().ok()))
            .product::<u32>()
    })
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 03 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 181345830);
    assert_eq!(part2, 98729041);
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
