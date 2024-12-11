#![feature(test)]

use itertools::Itertools;
use num::Complex;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

const N: isize = 50;

fn checked_add(complex: Complex<isize>, n: isize, antinodes: &mut HashSet<Complex<isize>>) -> bool {
    let Complex { re, im } = complex;

    if re.is_negative() || im.is_negative() || re >= n || im >= n {
        return false;
    }

    antinodes.insert(complex);

    true
}

fn parse() -> HashMap<char, Vec<Complex<isize>>> {
    let mut freqencies = HashMap::<char, Vec<Complex<isize>>>::new();

    for (x, line) in include_str!("input.txt").lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }

            freqencies
                .entry(c)
                .or_default()
                .push(Complex::new(x as isize, y as isize));
        }
    }

    freqencies
}

fn part1() -> usize {
    let frequencies = parse();

    let mut antinodes = HashSet::new();

    for nodes in frequencies.values() {
        for pair in nodes.iter().combinations(2) {
            let a1 = pair[0];
            let a2 = pair[1];

            let diff = a2 - a1;

            checked_add(a1 - diff, N, &mut antinodes);
            checked_add(a2 + diff, N, &mut antinodes);
        }
    }

    antinodes.len()
}

fn part2() -> usize {
    let frequencies = parse();

    let mut antinodes = HashSet::new();

    for nodes in frequencies.values() {
        for pair in nodes.iter().combinations(2) {
            let a1 = pair[0];
            let a2 = pair[1];

            let diff = a2 - a1;

            let mut i = 0;

            while checked_add(a1 - (diff * i), N, &mut antinodes) {
                i += 1;
            }

            i = 0;

            while checked_add(a2 + (diff * i), N, &mut antinodes) {
                i += 1;
            }
        }
    }

    antinodes.len()
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 08 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 341);
    assert_eq!(part2, 1134);
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
