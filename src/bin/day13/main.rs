#![feature(test)]

use itertools::Itertools;
use num::Integer;
use regex::Regex;
use std::time::Instant;

fn solve(part1: bool) -> i64 {
    let file = include_str!("input.txt");

    let re = Regex::new(r"\d+").unwrap();

    let mut ans = 0;

    for chunk in &re
        .captures_iter(file)
        .filter_map(|caps| caps.get(0).and_then(|m| m.as_str().parse::<i64>().ok()))
        .chunks(6)
    {
        let mut numbers = chunk.collect_tuple::<(_, _, _, _, _, _)>().unwrap();

        // (ax, ay, bx, by, x, y)
        // ax * a_div + bx * b_div = x
        // ay * a_div + by * b_div = y

        if !part1 {
            numbers.4 += 10000000000000;
            numbers.5 += 10000000000000;
        }

        let ax = numbers.0 * numbers.1;
        let _ay = numbers.1 * numbers.0;
        let bx = numbers.2 * numbers.1;
        let by = numbers.3 * numbers.0;
        let x = numbers.4 * numbers.1;
        let y = numbers.5 * numbers.0;

        let (b_div, b_rem) = (x - y).div_rem(&(bx - by));
        let (a_div, a_rem) = (x - (bx * b_div)).div_rem(&ax);

        if a_rem == 0 && b_rem == 0 {
            ans += a_div * 3 + b_div;
        }
    }

    ans
}

fn part1() -> i64 {
    solve(true)
}

fn part2() -> i64 {
    solve(false)
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 13 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 26299);
    assert_eq!(part2, 107824497933339);
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
