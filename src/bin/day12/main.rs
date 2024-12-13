#![feature(test)]

use itertools::Itertools;
use num::Complex;
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

const N: isize = 140;

fn parse() -> HashMap<Complex<isize>, char> {
    let mut grid = HashMap::new();

    for (x, line) in include_str!("input.txt").lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            grid.insert(Complex::new(x as isize, y as isize), c);
        }
    }

    grid
}

fn flood(
    grid: &HashMap<Complex<isize>, char>,
    origin: Complex<isize>,
    visited: &mut HashSet<Complex<isize>>,
) -> HashSet<Complex<isize>> {
    let mut region = HashSet::new();

    let mut queue = VecDeque::from([origin]);

    while let Some(pos) = queue.pop_front() {
        if visited.contains(&pos) {
            continue;
        }

        visited.insert(pos);
        region.insert(pos);

        let plot = grid.get(&pos).unwrap();

        let mut direction = -Complex::ONE;

        for _ in 0..4 {
            let neighbour_pos = pos + direction;

            if grid
                .get(&neighbour_pos)
                .is_some_and(|neighbour| plot == neighbour)
            {
                queue.push_back(neighbour_pos);
            }

            direction *= -Complex::I;
        }
    }

    region
}

fn calculate_perimeter(region: &HashSet<Complex<isize>>, part1: bool) -> usize {
    let mut perimeter = 0;

    for pos in region
        .iter()
        .sorted_unstable_by(|p1, p2| (p1.re, p1.im).cmp(&(p2.re, p2.im)))
    {
        // Check up
        let mut neighbour_pos = pos - Complex::ONE;

        if pos.re == 0 {
            if part1 || !region.contains(&(pos - Complex::I)) {
                perimeter += 1;
            }
        } else if !region.contains(&neighbour_pos)
            && (part1
                || !region.contains(&(pos - Complex::I))
                || region.contains(&(pos + Complex::new(-1, -1))))
        {
            perimeter += 1;
        }

        // Check right
        neighbour_pos = pos + Complex::I;

        if pos.im == N - 1 {
            if part1 || !region.contains(&(pos - Complex::ONE)) {
                perimeter += 1;
            }
        } else if !region.contains(&neighbour_pos)
            && (part1
                || !region.contains(&(pos - Complex::ONE))
                || region.contains(&(pos + Complex::new(-1, 1))))
        {
            perimeter += 1;
        }

        // Check down
        neighbour_pos = pos + Complex::ONE;

        if pos.re == N - 1 {
            if part1 || !region.contains(&(pos - Complex::I)) {
                perimeter += 1;
            }
        } else if !region.contains(&neighbour_pos)
            && (part1
                || !region.contains(&(pos - Complex::I))
                || region.contains(&(pos + Complex::new(1, -1))))
        {
            perimeter += 1;
        }

        // Check left
        neighbour_pos = pos - Complex::I;

        if pos.im == 0 {
            if part1 || !region.contains(&(pos - Complex::ONE)) {
                perimeter += 1;
            }
        } else if !region.contains(&neighbour_pos)
            && (part1
                || !region.contains(&(pos - Complex::ONE))
                || region.contains(&(pos + Complex::new(-1, -1))))
        {
            perimeter += 1;
        }
    }

    perimeter
}

fn calculate(part1: bool) -> usize {
    let grid = parse();

    let mut ans = 0;

    let mut visited = HashSet::new();

    for x in 0..N {
        for y in 0..N {
            let pos = Complex::new(x, y);

            if visited.contains(&pos) {
                continue;
            }

            let region = flood(&grid, pos, &mut visited);

            let perimeter = calculate_perimeter(&region, part1);

            ans += region.len() * perimeter;
        }
    }

    ans
}

fn part1() -> usize {
    calculate(true)
}

fn part2() -> usize {
    calculate(false)
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 12 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 1396562);
    assert_eq!(part2, 844132);
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
