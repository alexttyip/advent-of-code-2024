#![feature(test)]

use std::time::Instant;

fn part1() -> u32 {
    let file = include_str!("input.txt");

    let mut v1 = Vec::with_capacity(1000);
    let mut v2 = Vec::with_capacity(1000);

    for line in file.lines() {
        v1.push(line[0..5].parse::<u32>().unwrap());
        v2.push(line[8..].parse::<u32>().unwrap());
    }

    v1.sort_unstable();
    v2.sort_unstable();

    v1.iter().zip(&v2).map(|(i1, i2)| i1.abs_diff(*i2)).sum()
}

fn part2() -> usize {
    let file = include_str!("input.txt");

    let mut v1 = Vec::with_capacity(1000);
    let mut counts = [0; 100_000];

    for line in file.lines() {
        v1.push(line[0..5].parse::<usize>().unwrap());
        counts[line[8..].parse::<usize>().unwrap()] += 1;
    }

    v1.iter().map(|i1| i1 * counts[*i1]).sum::<usize>()
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 01 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 1646452);
    assert_eq!(part2, 23609874);
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
