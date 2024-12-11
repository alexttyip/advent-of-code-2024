#![feature(test)]

use itertools::Itertools;
use num::Complex;
use std::collections::HashSet;
use std::time::Instant;

const N: isize = 130;

fn parse() -> (HashSet<Complex<isize>>, Complex<isize>) {
    let mut piles = HashSet::new();

    let mut guard = Complex::ZERO;

    for (x, line) in include_str!("input.txt").lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            let point = Complex::new(x as isize, y as isize);

            if c == '#' {
                piles.insert(point);
            } else if c == '^' {
                guard = point;
            }
        }
    }

    (piles, guard)
}

fn simulate_paths(
    piles: &HashSet<Complex<isize>>,
    mut guard: Complex<isize>,
) -> HashSet<Complex<isize>> {
    let mut dir = -Complex::ONE;
    let mut path = HashSet::new();

    while (0..N).contains(&guard.re) && (0..N).contains(&guard.im) {
        path.insert(guard);

        if piles.contains(&(guard + dir)) {
            dir *= -Complex::I;
            continue;
        }

        guard += dir;
    }

    path
}

fn part1() -> usize {
    let (piles, guard) = parse();

    simulate_paths(&piles, guard).len()
}

fn part2() -> usize {
    let (piles, original_guard) = parse();

    let path = simulate_paths(&piles, original_guard);

    let mut loops = 0;
    let mut visited = HashSet::new();

    for &new_pile in path.iter().unique() {
        let mut guard = original_guard;
        let mut dir = -Complex::ONE;
        visited.clear();

        while (0..N).contains(&guard.re) && (0..N).contains(&guard.im) {
            if !visited.insert((guard, dir)) {
                loops += 1;
                break;
            }

            if piles.contains(&(guard + dir)) || guard + dir == new_pile {
                dir *= -Complex::I;
                continue;
            }

            guard += dir;
        }
    }

    loops
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 06 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 5145);
    assert_eq!(part2, 1523);
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
