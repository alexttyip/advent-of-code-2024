#![feature(test)]

use itertools::Itertools;
use std::time::Instant;

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const PART2_VALID_PATTERNS: [[(isize, isize, char); 4]; 4] = [
    [(-1, -1, 'M'), (1, 1, 'S'), (1, -1, 'M'), (-1, 1, 'S')],
    [(-1, -1, 'S'), (1, 1, 'M'), (1, -1, 'M'), (-1, 1, 'S')],
    [(-1, -1, 'M'), (1, 1, 'S'), (1, -1, 'S'), (-1, 1, 'M')],
    [(-1, -1, 'S'), (1, 1, 'M'), (1, -1, 'S'), (-1, 1, 'M')],
];

fn read_grid() -> Vec<Vec<char>> {
    include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec()
}

fn count_matches_part1(grid: &[Vec<char>], i: usize, j: usize) -> usize {
    if grid[i][j] != 'X' {
        return 0;
    }

    DIRECTIONS
        .iter()
        .filter(|(di, dj)| {
            ['M', 'A', 'S'].iter().enumerate().all(|(n, c)| {
                let Some((ii, jj)) = i
                    .checked_add_signed(di * (n as isize + 1))
                    .zip(j.checked_add_signed(dj * (n as isize + 1)))
                else {
                    return false;
                };

                let Some(cc) = grid.get(ii).and_then(|line| line.get(jj)) else {
                    return false;
                };

                c == cc
            })
        })
        .count()
}

fn part1() -> usize {
    let grid = read_grid();

    (0..grid.len())
        .flat_map(|i| (0..grid.len()).map(move |j| (i, j)))
        .map(|(i, j)| count_matches_part1(&grid, i, j))
        .sum()
}

fn is_valid_cross_part2(grid: &[Vec<char>], i: usize, j: usize) -> bool {
    grid[i][j] == 'A'
        && PART2_VALID_PATTERNS.iter().any(|pattern| {
            pattern.iter().all(|&(di, dj, c)| {
                let Some((ii, jj)) = i.checked_add_signed(di).zip(j.checked_add_signed(dj)) else {
                    return false;
                };

                let Some(cc) = grid.get(ii).and_then(|line| line.get(jj)) else {
                    return false;
                };

                &c == cc
            })
        })
}

fn part2() -> usize {
    let grid = read_grid();

    (0..grid.len())
        .flat_map(|i| (0..grid.len()).map(move |j| (i, j)))
        .filter(|&(i, j)| is_valid_cross_part2(&grid, i, j))
        .count()
}

pub fn main() {
    let mut now = Instant::now();
    let part1 = part1();
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2 = part2();
    let part2_elapsed = now.elapsed();

    println!("--- Day 04 ---");
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    println!("Part 1 took: {:.2?}", part1_elapsed);
    println!("Part 2 took: {:.2?}", part2_elapsed);

    assert_eq!(part1, 2514);
    assert_eq!(part2, 1888);
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
